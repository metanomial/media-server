use rocket::http::ContentType;
use rocket::response::content::RawCss;
use rocket::Route;
use std::include_bytes;
use std::vec::Vec;

#[rocket::get("/favicon.ico")]
fn favicon() -> (ContentType, &'static [u8]) {
  let bytes = include_bytes!("../static/favicon.ico");
  (ContentType::Icon, bytes)
}

#[rocket::get("/favicon-16x16.png")]
fn favicon_16x16() -> (ContentType, &'static [u8]) {
  let bytes = include_bytes!("../static/favicon-16x16.png");
  (ContentType::PNG, bytes)
}

#[rocket::get("/favicon-32x32.png")]
fn favicon_32x32() -> (ContentType, &'static [u8]) {
  let bytes = include_bytes!("../static/favicon-32x32.png");
  (ContentType::PNG, bytes)
}

#[rocket::get("/apple-touch-icon.png")]
fn apple_touch_icon() -> (ContentType, &'static [u8]) {
  let bytes = include_bytes!("../static/apple-touch-icon.png");
  (ContentType::PNG, bytes)
}

#[rocket::get("/android-chrome-192x192.png")]
fn android_chrome_192x192() -> (ContentType, &'static [u8]) {
  let bytes = include_bytes!("../static/android-chrome-192x192.png");
  (ContentType::PNG, bytes)
}

#[rocket::get("/android-chrome-512x512.png")]
fn android_chrome_512x512() -> (ContentType, &'static [u8]) {
  let bytes = include_bytes!("../static/android-chrome-512x512.png");
  (ContentType::PNG, bytes)
}

#[rocket::get("/main.css")]
fn stylesheet() -> RawCss<&'static str> {
  RawCss(include_str!("../static/main.css"))
}

#[rocket::get("/site.webmanifest")]
fn manifest() -> (ContentType, &'static str) {
  let text = include_str!("../static/site.webmanifest");
  (ContentType::new("application", "manifest+json"), text)
}

pub fn routes() -> Vec<Route> {
  rocket::routes![
    favicon,
    favicon_16x16,
    favicon_32x32,
    apple_touch_icon,
    android_chrome_192x192,
    android_chrome_512x512,
    stylesheet,
    manifest
  ]
}
