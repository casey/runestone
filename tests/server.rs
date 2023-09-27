use super::*;

struct KillOnDrop(std::process::Child);

impl Drop for KillOnDrop {
  fn drop(&mut self) {
    assert!(Command::new("kill")
      .arg(self.0.id().to_string())
      .status()
      .unwrap()
      .success());
  }
}

#[test]
fn server_returns_homepage() {
  let _server = KillOnDrop(
    Command::new(executable_path("runestone"))
      .arg("server")
      .spawn()
      .unwrap(),
  );

  for i in 0..100 {
    if reqwest::get("http://localhost").is_ok() {
      break;
    }

    if i == 99 {
      panic!("server failed to start");
    }

    thread::sleep(Duration::from_millis(100));
  }

  assert_eq!(
    reqwest::get("http://localhost").unwrap().text().unwrap(),
    "Hello, world!"
  );
}
