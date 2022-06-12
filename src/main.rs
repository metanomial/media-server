use clap::Parser;
use rocket::response::content::RawHtml;
use std::path::PathBuf;

mod static_files;

/// Simple media server
#[derive(Parser)]
#[clap(version)]
struct Cli {
  /// Path to library
  #[clap()]
  path: Option<PathBuf>,

  /// Enable verbose output
  #[clap(short, long)]
  verbose: bool,
}

#[rocket::launch]
async fn launch() -> _ {
  let cli = Cli::parse();
  let path = match cli.path {
    Some(path) => path.canonicalize().unwrap(),
    None => std::env::current_dir().unwrap(),
  };
  if !path.is_dir() {
    println!("Path is not a directory");
}
  rocket::build()
    .mount("/", rocket::routes![index])
    .mount("/", static_files::routes())
}

#[rocket::get("/")]
fn index() -> RawHtml<&'static str> {
  let html = "<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title>Media Server</title>
    <link rel=\"stylesheet\" href=\"/main.css\">
    <link rel=\"icon\" type=\"image/png\" sizes=\"32x32\" href=\"/favicon-32x32.png\">
    <link rel=\"icon\" type=\"image/png\" sizes=\"16x16\" href=\"/favicon-16x16.png\">
    <link rel=\"apple-touch-icon\" sizes=\"180x180\" href=\"/apple-touch-icon.png\">
    <link rel=\"manifest\" href=\"/site.webmanifest\">
  </head>
  <body>
    <main>Not implemented</main>
  </body>
</html>";
  RawHtml(html)
}
