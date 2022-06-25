use crate::library::helpers::load_dir_or_err;
use anyhow::Result;
use regex::Regex;
use std::{fmt, path::PathBuf};

const MOVIE_METADATA_FILE: &str = "movie.nfo";

/// Movie collection
pub type MovieCollection = std::collections::HashMap<String, Movie>;

/// Movie
pub struct Movie {
  /// Path to movie directory
  pub path: PathBuf,

  /// Movie NFO metadata
  pub nfo: MovieNfo,
}

impl Movie {
  /// Loads a collection of movies in the given root directory.
  pub fn load_collection(path: PathBuf) -> Result<MovieCollection> {
    use log::info;
    load_dir_or_err("movies collection", &path)?;
    info!("Loading movies collection at {}", path.to_string_lossy());
    let collection: MovieCollection = path
      .read_dir()?
      .filter_map(Result::ok)
      .filter_map(|entry| {
        let path = entry.path();
        info!("Loading movie at {}", path.to_string_lossy());
        Movie::load(path).ok()
      })
      .inspect(|movie| info!("Loaded {}", movie))
      .map(Movie::key_value_pair)
      .collect();
    info!("Loaded {} movies", collection.len());
    Ok(collection)
  }

  /// Loads a movie from the given path.
  fn load(path: PathBuf) -> Result<Movie> {
    load_dir_or_err("movie", &path)?;
    let nfo = MovieNfo::load(path.join(MOVIE_METADATA_FILE)).unwrap_or_default();
    Ok(Movie { path, nfo })
  }

  /// Creates a key-value pair for collecting into a `MovieCollection`.
  pub fn key_value_pair(self) -> (String, Movie) {
    (self.slug(), self)
  }

  /// Calculates a URL slug from metadata.
  pub fn slug(&self) -> String {
    let title = self.title().to_lowercase().replace(" ", "-");
    let year = self.year().map(|y| y.to_string()).unwrap_or_default();
    format!("{}-{}", title, year)
  }

  /// Gets the movie directory basename.
  pub fn basename(&self) -> String {
    self.path.file_name().unwrap().to_string_lossy().to_string()
  }

  /// Gets the display title of the movie.
  ///
  /// Falls back on extracting the title from the directory basename
  /// if the title field is missing from the NFO metadata file.
  pub fn title(&self) -> String {
    let basename = self.basename();
    match &self.nfo.title {
      Some(t) => t,
      None => Regex::new(r"^([^(]*)")
        .unwrap()
        .captures(&basename)
        .unwrap()
        .get(0)
        .unwrap()
        .as_str(),
    }
    .trim()
    .to_string()
  }

  /// Gets the release year of the movie.
  ///
  /// Falls back on extracting the year from the directory basename
  /// if the year field is missing in the NFO metadata file.
  pub fn year(&self) -> Option<u32> {
    let basename = self.basename();
    self.nfo.year.or_else(|| {
      Regex::new(r"^[^(]*\(([1-9][0-9]{3})\)")
        .unwrap()
        .captures(&basename)
        .map(|c| c.get(0))?
        .map(|m| m.as_str().to_string().parse().ok())?
    })
  }
}

impl fmt::Display for Movie {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let title = self.title();
    match &self.year() {
      Some(y) => write!(f, "{} ({})", title, y),
      None => write!(f, "{}", title),
    }
  }
}

/// Movie metadata
#[derive(serde::Deserialize, Default)]
#[serde(rename = "movie")]
pub struct MovieNfo {
  /// Movie title
  pub title: Option<String>,

  /// Original title
  #[serde(rename = "originaltitle")]
  pub original_title: Option<String>,

  /// Release year
  pub year: Option<u32>,

  /// Plot summary of the movie
  #[serde(rename = "plot")]
  pub summary: Option<String>,
}

impl MovieNfo {
  /// Loads a movie NFO metadata file from the given path.
  pub fn load(path: PathBuf) -> Result<MovieNfo> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    Ok(quick_xml::de::from_reader(reader)?)
  }
}
