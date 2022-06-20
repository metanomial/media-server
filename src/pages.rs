use crate::library::Library;
use maud::{html, Markup, DOCTYPE};
use rocket::{Route, State};

fn page_title(title: Option<&str>) -> Markup {
  html! {
    title {
      "Media Server"
      @match title {
        Some(title) => { " | " (title) },
        None => "",
      }
    }
  }
}

fn menu() -> Markup {
  html! {
    nav {
      menu {
        li {
          a href="/" { "Home" }
        }
        li {
          a href="/movies" { "Movies" }
        }
      }
    }
  }
}

fn header(title: Option<&str>) -> Markup {
  html! {
    @match title {
      Some(title) => {
        header {
          h1 {
            (title)
          }
          select {
            option default value="new" { "New" }
            option value="alphabetical" { "Alphabetical" }
          }
        }
      },
      None => ""
    }
  }
}

fn base(title: Option<&str>, content: Markup) -> Markup {
  html! {
    (DOCTYPE)
    head {
      meta charset="utf-8";
      meta name="viewport" content="width=device-width, initial-scale=1.0";
      (page_title(title))
      link rel="stylesheet" href="/main.css";
      link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png";
      link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png";
      link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png";
      link rel="manifest" href="/site.webmanifest";
    }
    body {
      (menu())
      (header(title))
      main {
        (content)
      }
    }
  }
}

#[rocket::get("/")]
fn home() -> Markup {
  base(None, html! {})
}

#[rocket::get("/movies")]
fn movies(library: &State<Library>) -> Markup {
  base(
    Some("Movies"),
    html! {
      @for (_, movie) in &library.movies {
        figure class="movie" {
          figcaption {
            (movie)
          }
        }
      }
    },
  )
}

pub fn routes() -> Vec<Route> {
  rocket::routes![home, movies]
}
