#![forbid(unsafe_code)]
#![feature(io_error_more)]

use clap::Parser;
use library::Library;
use logger::Logger;
use std::path::PathBuf;

mod library;
mod logger;
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
  let logger = Logger::new(cli.verbose);
  let library = match Library::load(cli.path, &logger) {
    Ok(l) => l,
    Err(e) => {
      logger.error(e);
      std::process::exit(1);
    }
  };
  rocket::build()
    .manage(logger)
    .manage(library)
    .mount("/", pages::routes())
    .mount("/", static_files::routes())
}
