use {
  bitcoin::{consensus::Encodable, locktime, opcodes, script, Transaction, TxOut},
  executable_path::executable_path,
  std::{
    io::Write,
    process::{Command, Stdio},
    str,
  },
};

mod decipher;
