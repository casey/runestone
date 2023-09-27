use super::*;

struct KillOnDrop(process::Child);

impl Drop for KillOnDrop {
  fn drop(&mut self) {
    self.0.kill().unwrap()
  }
}

#[test]
fn server_returns_homepage() {
  let port = TcpListener::bind("127.0.0.1:0")
    .unwrap()
    .local_addr()
    .unwrap()
    .port();

  let _server = KillOnDrop(
    Command::new(executable_path("runestone"))
      .args(["server", "--http-port", &port.to_string()])
      .spawn()
      .unwrap(),
  );

  for i in 0..100 {
    if reqwest::get(format!("http://localhost:{port}")).is_ok() {
      break;
    }

    if i == 99 {
      panic!("server failed to start");
    }

    thread::sleep(Duration::from_millis(100));
  }

  assert_eq!(
    reqwest::get(format!("http://localhost:{port}"))
      .unwrap()
      .text()
      .unwrap(),
    "Hello, world!"
  );
}
