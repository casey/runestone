use {
  self::{directive::Directive, runestone::Runestone},
  bitcoin::{
    opcodes,
    script::{self, Instruction},
    Transaction,
  },
  serde::Serialize,
  std::fmt::{self, Display, Formatter},
};

mod directive;
mod runestone;
mod varint;

#[derive(Debug, PartialEq)]
pub enum Error {
  Script(script::Error),
  Opcode(opcodes::All),
  Payload,
}

impl From<script::Error> for Error {
  fn from(error: script::Error) -> Self {
    Self::Script(error)
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Script(err) => write!(f, "failed to parse script: {err}"),
      Self::Opcode(op) => write!(f, "non-push opcode {op} in payload"),
      Self::Payload => write!(f, "payload length was not a multiple of 16"),
    }
  }
}

impl std::error::Error for Error {}

impl Runestone {
  pub fn decipher(transaction: &Transaction) -> Result<Option<Self>, Error> {
    let Some(payload) = Runestone::payload(transaction)? else {
      return Ok(None);
    };

    if payload.len() % 16 != 0 {
      return Err(Error::Payload);
    }

    let integers = payload
      .chunks_exact(16)
      .map(|chunk| u128::from_le_bytes(chunk.try_into().unwrap()))
      .collect::<Vec<u128>>();

    let mut directives = Vec::new();
    let mut decimals = None;
    let mut symbol = None;

    for chunk in integers.chunks(3) {
      match chunk {
        [id, amount, output] => directives.push(Directive {
          id: *id,
          amount: *amount,
          output: *output,
        }),
        [d] => decimals = Some(*d),
        [d, s] => {
          decimals = Some(*d);
          symbol = Some(*s);
        }
        _ => unreachable!(),
      }
    }

    Ok(Some(Self {
      directives,
      decimals,
      symbol,
    }))
  }

  fn payload(transaction: &Transaction) -> Result<Option<Vec<u8>>, Error> {
    for output in &transaction.output {
      let mut instructions = output.script_pubkey.instructions();

      if instructions.next().transpose()? != Some(Instruction::Op(opcodes::all::OP_RETURN)) {
        continue;
      }

      if instructions.next().transpose()? != Some(Instruction::PushBytes(b"RUNE_TEST".into())) {
        continue;
      }

      let mut payload = Vec::new();

      for result in instructions {
        match result? {
          Instruction::PushBytes(push) => payload.extend_from_slice(push.as_bytes()),
          Instruction::Op(op) => return Err(Error::Opcode(op)),
        }
      }

      return Ok(Some(payload));
    }

    Ok(None)
  }
}

#[cfg(test)]
mod tests {
  use {
    super::*,
    bitcoin::{locktime, script::PushBytes, ScriptBuf, TxOut},
  };

  #[test]
  fn deciphering_transaction_with_no_outputs_returns_none() {
    assert_eq!(
      Runestone::decipher(&Transaction {
        input: Vec::new(),
        output: Vec::new(),
        lock_time: locktime::absolute::LockTime::ZERO,
        version: 0,
      }),
      Ok(None)
    );
  }

  #[test]
  fn deciphering_transaction_with_non_op_return_output_returns_none() {
    assert_eq!(
      Runestone::decipher(&Transaction {
        input: Vec::new(),
        output: vec![TxOut {
          script_pubkey: script::Builder::new().push_slice([]).into_script(),
          value: 0
        }],
        lock_time: locktime::absolute::LockTime::ZERO,
        version: 0,
      }),
      Ok(None)
    );
  }

  #[test]
  fn deciphering_transaction_with_bare_op_return_returns_none() {
    assert_eq!(
      Runestone::decipher(&Transaction {
        input: Vec::new(),
        output: vec![TxOut {
          script_pubkey: script::Builder::new()
            .push_opcode(opcodes::all::OP_RETURN)
            .into_script(),
          value: 0
        }],
        lock_time: locktime::absolute::LockTime::ZERO,
        version: 0,
      }),
      Ok(None)
    );
  }

  #[test]
  fn deciphering_transaction_with_non_matching_op_return_returns_none() {
    assert_eq!(
      Runestone::decipher(&Transaction {
        input: Vec::new(),
        output: vec![TxOut {
          script_pubkey: script::Builder::new()
            .push_opcode(opcodes::all::OP_RETURN)
            .push_slice(b"FOOO")
            .into_script(),
          value: 0
        }],
        lock_time: locktime::absolute::LockTime::ZERO,
        version: 0,
      }),
      Ok(None)
    );
  }

  #[test]
  fn deciphering_valid_runestone_with_invalid_script_returns_script_error() {
    let result = Runestone::decipher(&Transaction {
      input: Vec::new(),
      output: vec![TxOut {
        script_pubkey: ScriptBuf::from_bytes(vec![opcodes::all::OP_PUSHBYTES_4.to_u8()]),
        value: 0,
      }],
      lock_time: locktime::absolute::LockTime::ZERO,
      version: 0,
    });

    match result {
      Ok(_) => panic!("expected error"),
      Err(Error::Script(_)) => {}
      Err(err) => panic!("unexpected error: {err}"),
    }
  }

  #[test]
  fn deciphering_valid_runestone_with_invalid_script_postfix_returns_script_error() {
    let mut script_pubkey = script::Builder::new()
      .push_opcode(opcodes::all::OP_RETURN)
      .push_slice(b"RUNE_TEST")
      .into_script()
      .into_bytes();

    script_pubkey.push(opcodes::all::OP_PUSHBYTES_4.to_u8());

    let result = Runestone::decipher(&Transaction {
      input: Vec::new(),
      output: vec![TxOut {
        script_pubkey: ScriptBuf::from_bytes(script_pubkey),
        value: 0,
      }],
      lock_time: locktime::absolute::LockTime::ZERO,
      version: 0,
    });

    match result {
      Ok(_) => panic!("expected error"),
      Err(Error::Script(_)) => {}
      Err(err) => panic!("unexpected error: {err}"),
    }
  }

  #[test]
  fn deciphering_empty_runestone_is_successful() {
    assert_eq!(
      Runestone::decipher(&Transaction {
        input: Vec::new(),
        output: vec![TxOut {
          script_pubkey: script::Builder::new()
            .push_opcode(opcodes::all::OP_RETURN)
            .push_slice(b"RUNE_TEST")
            .into_script(),
          value: 0
        }],
        lock_time: locktime::absolute::LockTime::ZERO,
        version: 0,
      }),
      Ok(Some(Runestone {
        directives: Vec::new(),
        decimals: None,
        symbol: None,
      }))
    );
  }

  #[test]
  fn deciphering_non_empty_runestone_is_successful() {
    let payload = 1u128
      .to_le_bytes()
      .into_iter()
      .chain(2u128.to_le_bytes())
      .chain(3u128.to_le_bytes())
      .collect::<Vec<u8>>();

    let payload: &PushBytes = payload.as_slice().try_into().unwrap();

    assert_eq!(
      Runestone::decipher(&Transaction {
        input: Vec::new(),
        output: vec![TxOut {
          script_pubkey: script::Builder::new()
            .push_opcode(opcodes::all::OP_RETURN)
            .push_slice(b"RUNE_TEST")
            .push_slice(payload)
            .into_script(),
          value: 0
        }],
        lock_time: locktime::absolute::LockTime::ZERO,
        version: 0,
      }),
      Ok(Some(Runestone {
        directives: vec![Directive {
          id: 1,
          amount: 2,
          output: 3,
        }],
        decimals: None,
        symbol: None,
      }))
    );
  }

  #[test]
  fn additional_integer_is_decimals() {
    let payload = 1u128
      .to_le_bytes()
      .into_iter()
      .chain(2u128.to_le_bytes())
      .chain(3u128.to_le_bytes())
      .chain(4u128.to_le_bytes())
      .collect::<Vec<u8>>();

    let payload: &PushBytes = payload.as_slice().try_into().unwrap();

    assert_eq!(
      Runestone::decipher(&Transaction {
        input: Vec::new(),
        output: vec![TxOut {
          script_pubkey: script::Builder::new()
            .push_opcode(opcodes::all::OP_RETURN)
            .push_slice(b"RUNE_TEST")
            .push_slice(payload)
            .into_script(),
          value: 0
        }],
        lock_time: locktime::absolute::LockTime::ZERO,
        version: 0,
      }),
      Ok(Some(Runestone {
        directives: vec![Directive {
          id: 1,
          amount: 2,
          output: 3,
        }],
        decimals: Some(4),
        symbol: None,
      }))
    );
  }

  #[test]
  fn additional_two_integers_are_decimals_and_symbol() {
    let payload = 1u128
      .to_le_bytes()
      .into_iter()
      .chain(2u128.to_le_bytes())
      .chain(3u128.to_le_bytes())
      .chain(4u128.to_le_bytes())
      .chain(5u128.to_le_bytes())
      .collect::<Vec<u8>>();

    let payload: &PushBytes = payload.as_slice().try_into().unwrap();

    assert_eq!(
      Runestone::decipher(&Transaction {
        input: Vec::new(),
        output: vec![TxOut {
          script_pubkey: script::Builder::new()
            .push_opcode(opcodes::all::OP_RETURN)
            .push_slice(b"RUNE_TEST")
            .push_slice(payload)
            .into_script(),
          value: 0
        }],
        lock_time: locktime::absolute::LockTime::ZERO,
        version: 0,
      }),
      Ok(Some(Runestone {
        directives: vec![Directive {
          id: 1,
          amount: 2,
          output: 3,
        }],
        decimals: Some(4),
        symbol: Some(5),
      }))
    );
  }

  #[test]
  fn payload_pushes_are_concatinated() {
    assert_eq!(
      Runestone::decipher(&Transaction {
        input: Vec::new(),
        output: vec![TxOut {
          script_pubkey: script::Builder::new()
            .push_opcode(opcodes::all::OP_RETURN)
            .push_slice(b"RUNE_TEST")
            .push_slice(1u128.to_le_bytes())
            .push_slice(2u128.to_le_bytes())
            .push_slice(3u128.to_le_bytes())
            .push_slice(4u128.to_le_bytes())
            .push_slice(5u128.to_le_bytes())
            .into_script(),
          value: 0
        }],
        lock_time: locktime::absolute::LockTime::ZERO,
        version: 0,
      }),
      Ok(Some(Runestone {
        directives: vec![Directive {
          id: 1,
          amount: 2,
          output: 3,
        }],
        decimals: Some(4),
        symbol: Some(5),
      }))
    );
  }
}
