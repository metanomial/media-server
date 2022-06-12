use clap::Parser;
use rocket::response::content::RawHtml;
use std::path::PathBuf;

mod pages;
mod static_files;

/// Simple media server
#[derive(Parser)]
#[clap(version)]
struct Cli {
  /// Path to library
  #[clap()]
  path: Option<PathBuf>,

  /// Enable verbose output
  #[clap(short, long)]
  verbose: bool,
}

#[rocket::launch]
async fn launch() -> _ {
  let cli = Cli::parse();
  let path = match cli.path {
    Some(path) => path.canonicalize().unwrap(),
    None => std::env::current_dir().unwrap(),
  };
  if !path.is_dir() {
    println!("Path is not a directory");
}
  rocket::build()
    .mount("/", pages::routes())
    .mount("/", static_files::routes())
}
