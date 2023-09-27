use super::*;

#[test]
fn transaction_with_no_runestone_returns_null() {
  let child = Command::new(executable_path("runestone"))
    .arg("decipher")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .unwrap();

  let transaction = Transaction {
    input: Vec::new(),
    output: Vec::new(),
    lock_time: locktime::absolute::LockTime::ZERO,
    version: 0,
  };

  let mut buffer = Vec::new();

  transaction.consensus_encode(&mut buffer).unwrap();

  child.stdin.as_ref().unwrap().write_all(&buffer).unwrap();

  let output = child.wait_with_output().unwrap();

  assert!(output.status.success());

  let stdout = str::from_utf8(&output.stdout).unwrap();

  assert_eq!(stdout, "null\n");
}

#[test]
fn transaction_with_runestone_returns_serialized_runestone() {
  let child = Command::new(executable_path("runestone"))
    .arg("decipher")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .unwrap();

  let transaction = Transaction {
    input: Vec::new(),
    output: vec![TxOut {
      script_pubkey: script::Builder::new()
        .push_opcode(opcodes::all::OP_RETURN)
        .push_slice(b"RUNE_TEST")
        .into_script(),
      value: 0,
    }],
    lock_time: locktime::absolute::LockTime::ZERO,
    version: 0,
  };

  let mut buffer = Vec::new();

  transaction.consensus_encode(&mut buffer).unwrap();

  child.stdin.as_ref().unwrap().write_all(&buffer).unwrap();

  let output = child.wait_with_output().unwrap();

  assert!(output.status.success());

  let stdout = str::from_utf8(&output.stdout).unwrap();

  assert_eq!(
    stdout,
    r#"{
  "directives": [],
  "decimals": null,
  "symbol": null
}
"#
  );
}
