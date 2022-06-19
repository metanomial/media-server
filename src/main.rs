use crate::library::Library;
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
  let cli = Cli::parse();
  let library = match Library::load(cli.path) {
    Ok(l) => l,
    Err(e) => {
      println!("{}", e);
      std::process::exit(1);
    }
  };
  println!("Loaded {} movies", library.movies.len());
  rocket::build()
    .manage(library)
    .mount("/", pages::routes())
    .mount("/", static_files::routes())
}
