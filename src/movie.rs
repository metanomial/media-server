use regex::Regex;
use std::{io, path::PathBuf};

/// Movie metadata
pub struct Movie {
  /// Path to movie directory
  pub path: PathBuf,
  /// Title of the movie
  pub title: String,
  /// Year released
  pub year: Option<i32>,
}

impl Movie {
  /// Loads all movies in the given directory.
  ///
  /// Returns an empty vector if the directory cannot be loaded.
  pub fn load_all(path: PathBuf) -> Vec<Movie> {
    match path.read_dir() {
      Ok(r) => r
        .filter_map(|e| match e {
          Ok(e) => Movie::load(e.path()).ok(),
          Err(_) => None,
        })
        .collect(),
      Err(_) => vec![],
    }
  }

  /// Loads a movie from the given directory.
  pub fn load(path: PathBuf) -> io::Result<Movie> {
    if !path.is_dir() {
      return Err(io::Error::new(io::ErrorKind::Other, "Not a directory"));
    }
    let basename_regex = Regex::new(r"^(?P<title>[^()]+)(?: \((?P<year>\d{4})\))?$").unwrap();
    let basename = path.file_name().unwrap().to_string_lossy();
    let captures = basename_regex.captures(&basename);
    let title_year: (String, Option<i32>) = match captures {
      Some(captures) => (
        String::from(captures.name("title").unwrap().as_str()),
        match captures.name("year") {
          Some(m) => Some(m.as_str().parse().unwrap()),
          None => None,
        },
      ),
      None => (String::from(basename), None),
    };
    Ok(Movie {
      path,
      title: title_year.0,
      year: title_year.1,
    })
  }
}
