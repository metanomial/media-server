use std::fmt::Display;

/// Logging interface
pub struct Logger {
  /// Verbose output setting
  is_verbose: bool,
}

impl Logger {
  /// Initializes a logger.
  ///
  /// The `is_verbose` argument enables verbose logging. When disabled,
  /// the `verbose` method does not print to standard out.
  pub fn new(is_verbose: bool) -> Logger {
    Logger { is_verbose }
  }

  /// Prints to standard output.
  pub fn log(&self, message: impl Display) {
    println!("{}", message);
  }

  /// Prints to standard error.
  pub fn error(&self, message: impl Display) {
    eprintln!("{}", message);
  }

  /// Prints to standard output if verbose output is enabled.
  pub fn verbose(&self, message: impl Display) {
    if self.is_verbose {
      println!("{}", message);
    }
  }
}
