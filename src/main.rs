use clap::{Args, Parser, Subcommand};
use rocket::response::content::RawHtml;
use std::path::PathBuf;

mod static_files;

/// Simple media server
#[derive(Parser)]
#[clap(version)]
struct Cli {
  #[clap(subcommand)]
  command: Command,

  /// Enable verbose output
  #[clap(short, long)]
  verbose: bool,
}

#[derive(Subcommand)]
enum Command {
  Start(Start),
  Setup(Setup),
}

/// Load and serve a media library
#[derive(Args)]
struct Start {
  path: Option<PathBuf>,
}

/// Setup a new media library
#[derive(Args)]
struct Setup {
  path: Option<PathBuf>,
}

#[rocket::main]
async fn main() {
  let cli = Cli::parse();
  match &cli.command {
    Command::Start(cmd) => start(&cmd.path).await,
    Command::Setup(cmd) => setup(&cmd.path),
  };
}

async fn start(_: &Option<PathBuf>) {
  rocket::build()
    .mount("/", rocket::routes![index])
    .mount("/", static_files::routes())
    .launch()
    .await
    .map_err(|error| println!("{:?}", error))
    .ok();
}

fn setup(_: &Option<PathBuf>) {
  println!("Not implemented.");
}

#[rocket::get("/")]
fn index() -> RawHtml<&'static str> {
  let html = "<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title>Media Server</title>
    <link rel=\"stylesheet\" href=\"/main.css\">
    <link rel=\"icon\" type=\"image/png\" sizes=\"32x32\" href=\"/favicon-32x32.png\">
    <link rel=\"icon\" type=\"image/png\" sizes=\"16x16\" href=\"/favicon-16x16.png\">
    <link rel=\"apple-touch-icon\" sizes=\"180x180\" href=\"/apple-touch-icon.png\">
    <link rel=\"manifest\" href=\"/site.webmanifest\">
  </head>
  <body>
    <main>Not implemented</main>
  </body>
</html>";
  RawHtml(html)
}
