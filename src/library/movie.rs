use crate::{library::dir_or_err, logger::Logger};
use regex::Regex;
use std::{
  fmt, io,
  path::{Path, PathBuf},
};

/// Movie collection
pub type MovieCollection = std::collections::HashMap<String, Movie>;

/// Movie
#[derive(Debug)]
pub struct Movie {
  /// Path to movie directory
  pub path: PathBuf,

  /// Metadata
  pub metadata: Option<MovieMetadata>,
}

impl Movie {
  /// Loads all movies in the given root directory into the given collection.
  pub fn load_root(path: PathBuf, logger: &Logger) -> io::Result<MovieCollection> {
    logger.verbose(format!(
      "Loading Movies collection at {}",
      path.to_string_lossy()
    ));
    let collection: MovieCollection = path
      .read_dir()?
      .filter_map(Result::ok)
      .filter_map(|entry| Movie::load(entry.path(), logger).ok())
      .map(|movie| {
        logger.verbose(format!("Loaded {}", movie));
        (movie.slug(), movie)
      })
      .collect();
    logger.verbose(format!("Loaded {} movies", collection.len()));
    Ok(collection)
  }

  /// Loads a movie from the given path.
  fn load(path: PathBuf, logger: &Logger) -> io::Result<Movie> {
    dir_or_err(&path)?;
    let metadata = MovieMetadata::load(path.as_path(), logger).ok();
    Ok(Movie { path, metadata })
  }

  /// Calculates URL slug from title and year.
  pub fn slug(&self) -> String {
    todo!()
  }

  /// Calculates display title from metadata, falling back to path.
  pub fn title(&self) -> String {
    self
      .metadata
      .as_ref()
      .map(|m| m.title.as_ref())
      .flatten()
      .map(|t| t.into())
      .unwrap_or_else(|| self.path_title_year().0)
  }

  /// Calculates release year from metadata, falling back to path.
  pub fn year(&self) -> Option<u32> {
    self
      .metadata
      .as_ref()
      .map(|m| m.year)
      .flatten()
      .or_else(|| self.path_title_year().1)
  }

  /// Captures display title and year from directory name.
  fn path_title_year(&self) -> (String, Option<u32>) {
    let regex = Regex::new(r"^(.+)\s+\(([1-9]\d{3})\)").unwrap();
    let basename = self.path.file_name().unwrap().to_string_lossy();
    match regex.captures(&basename) {
      Some(c) => (
        c.get(0).unwrap().as_str().to_string(),
        c.get(1).unwrap().as_str().parse().ok(),
      ),
      None => (basename.to_string(), None),
    }
  }
}

impl fmt::Display for Movie {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let title = self.title();
    match self.year() {
      Some(y) => write!(f, "{} ({})", title, y),
      None => write!(f, "{}", title),
    }
  }
}

/// Movie edition
pub struct MovieEdition {
  /// Basename of edition files
  basename: String,

  /// Edition-specific metadata
  pub metadata: Option<MovieMetadata>,
}

/// Movie metadata
#[derive(Debug)]
pub struct MovieMetadata {
  /// Movie title
  pub title: Option<String>,

  /// Original title
  pub originaltitle: Option<String>,

  /// Release year
  pub year: Option<u32>,

  /// Plot summary of the movie
  pub summary: Option<String>,
}

impl MovieMetadata {
  pub fn load(path: &Path, logger: &Logger) -> io::Result<MovieMetadata> {
    todo!()
  }
}
