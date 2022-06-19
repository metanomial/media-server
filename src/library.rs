use crate::movie::Movie;
use std::{io, path::PathBuf};

/// Media library
pub struct Library {
  /// Movie collection
  pub movies: Vec<Movie>,
  /// Root directory
  pub root: PathBuf,
}

impl Library {
  /// Loads a media library into memory.
  ///
  /// If the optional `path` argument is not given,
  /// the library will be loaded from the current directory.
  ///
  /// If specified, `path` must be a directory.
  pub fn load(path: Option<PathBuf>) -> io::Result<Library> {
    let path = match path {
      Some(p) => p.canonicalize(),
      None => std::env::current_dir(),
    }?;
    if !path.is_dir() {
      return Err(io::Error::new(
        io::ErrorKind::Other,
        "Path is not a directory",
      ));
    }
    Ok(Library {
      movies: Movie::load_all(path.join("Movies")),
      root: path,
    })
  }
}
