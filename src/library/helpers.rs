use anyhow::{bail, Result};
use std::path::Path;

/// Errors if the given load path is not a directory.
pub fn load_dir_or_err(entity: &str, path: &Path) -> Result<()> {
  if !path.is_dir() {
    bail!(
      r#"Cannot load {}: path "{}" is not a directory"#,
      entity,
      path.to_string_lossy()
    )
  }
  Ok(())
}
