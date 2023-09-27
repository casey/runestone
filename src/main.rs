use {
  axum::routing::get,
  axum::Router,
  bitcoin::{consensus::Decodable, Transaction},
  clap::{
    builder::{
      styling::{AnsiColor, Effects},
      Styles,
    },
    Parser,
  },
  runestone::Runestone,
  std::{error::Error, io, net::ToSocketAddrs},
  tokio::runtime::Runtime,
};

#[derive(Parser)]
#[command(
  version,
  styles = Styles::styled()
    .header(AnsiColor::Green.on_default() | Effects::BOLD)
    .usage(AnsiColor::Green.on_default() | Effects::BOLD)
    .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
    .placeholder(AnsiColor::Cyan.on_default()))
]
enum Subcommand {
  #[command(
    about = "Read a bitcoin transaction from standard input and print a JSON representation of its runestone."
  )]
  Decipher,
  #[command(about = "Start the explorer.")]
  Server,
}

fn main() -> Result<(), Box<dyn Error>> {
  match Subcommand::parse() {
    Subcommand::Decipher => {
      let transaction = Transaction::consensus_decode(&mut io::stdin())
        .map_err(|err| format!("Failed to decode transaction: {err}"))?;

      let message = Runestone::decipher(&transaction)?;

      serde_json::to_writer_pretty(&io::stdout(), &message)?;
      println!();
    }
    Subcommand::Server => Runtime::new()?.block_on(async {
      let addr = ("0.0.0.0", 80).to_socket_addrs()?.next().unwrap();

      axum_server::Server::bind(addr)
        .serve(Router::new().route("/", get(home)).into_make_service())
        .await
    })?,
  }

  Ok(())
}

async fn home() -> &'static str {
  "Hello, world!"
}
