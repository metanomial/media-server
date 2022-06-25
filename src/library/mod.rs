mod helpers;
mod movie;

use crate::library::helpers::load_dir_or_err;
use anyhow::Result;
use log::info;
use movie::{Movie, MovieCollection};
use std::path::PathBuf;

const MOVIES_DIRECTORY: &str = "Movies";

/// Media library
pub struct Library {
  /// Path to root of media library
  pub path: PathBuf,

  /// Movie collection
  pub movies: MovieCollection,
}

impl Library {
  /// Loads media library metadata from storage.
  ///
  /// If the optional `path` argument is not given,
  /// the library will be loaded from the current directory.
  ///
  /// If specified, `path` must be a directory.
  pub fn load(path: Option<PathBuf>) -> Result<Library> {
    // Resolve path
    let path = match path {
      Some(p) => p.canonicalize(),
      None => std::env::current_dir(),
    }?;
    load_dir_or_err("library", &path)?;
    info!("Loading library from {}", path.to_string_lossy());

    // Initialize the library
    let movies = Movie::load_collection(path.join(MOVIES_DIRECTORY)).unwrap_or_default();
    let library = Library { path, movies };
    info!("Loaded library");

    Ok(library)
  }
}
