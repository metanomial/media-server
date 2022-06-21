mod helpers;
mod movie;

use crate::logger::Logger;
use helpers::dir_or_err;
use movie::Movie;
use std::{collections::HashMap, io, path::PathBuf};

const MOVIES_DIRECTORY: &str = "Movies";

/// Media library
pub struct Library {
  /// Path to root of media library
  pub path: PathBuf,

  /// Movie collection
  pub movies: HashMap<String, Movie>,
}

impl Library {
  /// Loads media library metadata from storage.
  ///
  /// If the optional `path` argument is not given,
  /// the library will be loaded from the current directory.
  ///
  /// If specified, `path` must be a directory.
  pub fn load(path: Option<PathBuf>, logger: &Logger) -> io::Result<Library> {
    // Resolve path
    let path = match path {
      Some(p) => p.canonicalize(),
      None => std::env::current_dir(),
    }?;
    logger.log(format!("Loading library from {}", path.to_string_lossy()));
    dir_or_err(&path)?;

    // Initialize the library
    let movies = Movie::load_collection(path.join(&MOVIES_DIRECTORY), logger).unwrap_or_default();
    let library = Library { path, movies };

    Ok(library)
  }
}
