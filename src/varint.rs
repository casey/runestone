fn encode(mut n: u128) -> Vec<u8> {
  let mut out = Vec::new();

  loop {
    let mut byte = n as u8 % 128;

    if !out.is_empty() {
      byte |= 0b1000_0000;
    }

    out.push(byte);

    if n < 128 {
      break;
    }

    n = n / 128 - 1;
  }

  out.reverse();

  out
}

fn decode(buffer: &[u8]) -> u128 {
  let mut n = 0;
  let mut i = 0;

  loop {
    let b = buffer.get(i).cloned().unwrap() as u128;

    if b < 128 {
      return n + b;
    }

    n += b - 127;
    n *= 128;

    i += 1;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn powers_of_two_round_trip_successfully() {
    for i in 0..128 {
      let n = 1 << i;
      let encoded = encode(n);
      let decoded = decode(&encoded);
      assert_eq!(decoded, n);
    }
  }

  #[test]
  fn alternating_bit_strings_round_trip_successfully() {
    let mut n = 0;

    for i in 0..129 {
      n = n << 1 | i % 2;
      let encoded = encode(n);
      let decoded = decode(&encoded);
      assert_eq!(decoded, n);
    }
  }

  #[test]
  #[ignore]
  fn decoding_integer_over_max_is_an_error() {
    todo!()
  }

  #[test]
  #[ignore]
  fn streams_of_integers_are_decoded_correctly() {
    todo!()
  }
}
