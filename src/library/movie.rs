use crate::{library::dir_or_err, logger::Logger};
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

  /// Calculate URL slug from metadata and/or path
  pub fn slug(&self) -> String {
    todo!()
  }

  /// Calculate display title from metadata or path
  pub fn title(&self) -> String {
    todo!()
  }

  /// Calculate release year from metadata or path
  pub fn year(&self) -> String {
    todo!()
  }

  // /// Extracts the title and year from the basename of a movie directory.
  // ///
  // /// Returns a tuple representing `(title, year)`.
  // ///
  // /// Returns the basename as `title` and a `None` as `year` if the year
  // /// is not present or the directory name cannot be parsed.
  // pub fn parse_basename(path: &PathBuf) -> (String, Option<i32>) {
  //   use regex::Regex;
  //   // Movie folder name pattern: "Movie Display Title (YYYY)"
  //   let basename_regex = Regex::new(r"^(.+)\s+\((\d{4})\)$").unwrap();
  //   let basename = path.file_name().unwrap().to_string_lossy();
  //   match basename_regex.captures(&basename) {
  //     // Folder name is properly formatted as "Title (YYYY)"
  //     Some(captures) => (
  //       captures.get(0).unwrap().as_str().to_string(),
  //       captures.get(1).map(|m| m.as_str().parse().unwrap()),
  //     ),
  //     // Folder name cannot be parsed
  //     None => (String::from(basename), None),
  //   }
  // }
}

impl fmt::Display for Movie {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} ({})", self.title(), self.year())
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
