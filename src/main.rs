use clap::Parser;
use std::{path::PathBuf, process};

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
    Some(path) => match path.canonicalize() {
      Ok(path) => path,
      Err(_) => {
        println!("Invalid path");
        process::exit(1)
      }
    },
    None => match std::env::current_dir() {
      Ok(path) => path,
      Err(_) => {
        println!("Cannot read current directory");
        process::exit(1)
      }
    },
  };
  if !path.is_dir() {
    println!("Path is not a directory");
    process::exit(1)
  }
  rocket::build()
    .mount("/", pages::routes())
    .mount("/", static_files::routes())
}
