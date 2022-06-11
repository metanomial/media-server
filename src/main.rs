mod static_files;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", static_files::routes())
}
