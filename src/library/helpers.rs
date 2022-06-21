use std::{io, path::PathBuf};

/// Errors if the given path is not a directory
pub fn dir_or_err(path: impl Into<PathBuf>) -> io::Result<()> {
  if !path.into().is_dir() {
    return Err(io::Error::from(io::ErrorKind::NotADirectory));
  }
  Ok(())
}

#[cfg(tests)]
mod tests {
  #[test]
  fn dir_or_does_err() {
    asserteq!(dir_or_err(PathBuf::new("..")), ());
    asserteq!(dir_or_err(
      PathBuf::new("mod.rs"),
      Err(io::Error::from(io::ErrorKind::NotADirectory))
    ));
  }
}
