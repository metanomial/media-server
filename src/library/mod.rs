mod movie;

use crate::logger::Logger;
use movie::Movie;
use std::{collections::HashMap, io, path::PathBuf};

/// Errors if the given path is not a directory
pub fn dir_or_err(path: impl Into<PathBuf>) -> io::Result<()> {
  match path.into().is_dir() {
    true => Ok(()),
    false => Err(io::Error::from(io::ErrorKind::NotADirectory)),
  }
}

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
    let movies = Movie::load_root(path.join("Movies"), logger);
    let library = Library {
      path,
      movies: movies.unwrap_or_default(),
    };

    Ok(library)
  }
}
