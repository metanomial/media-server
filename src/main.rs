#![forbid(unsafe_code)]
#![feature(io_error_more)]

use clap::Parser;
use std::path::PathBuf;

mod library;
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
  use library::Library;
  use rocket::config::LogLevel;
  use simplelog::*;

  let cli = Cli::parse();
  let (log_level, rocket_log_level) = match cli.verbose {
    true => (LevelFilter::Info, LogLevel::Normal),
    false => (LevelFilter::Warn, LogLevel::Critical),
  };

  simplelog::TermLogger::init(
    log_level,
    Config::default(),
    TerminalMode::Mixed,
    ColorChoice::Auto,
  )
  .unwrap();

  let library = match Library::load(cli.path) {
    Ok(l) => l,
    Err(e) => {
      log::error!("{}", e);
      std::process::exit(1);
    }
  };

  let config = rocket::Config::figment().merge(("log_level", rocket_log_level));

  rocket::custom(config)
    .manage(library)
    .mount("/", pages::routes())
    .mount("/", static_files::routes())
}
