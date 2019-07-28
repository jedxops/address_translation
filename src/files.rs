use rocket::response::NamedFile;
// use rocket::response::Redirect;
use rocket::http::RawStr;
use std::io;
use std::path::{Path, PathBuf};

#[get("/static/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}
#[get("/first")]
pub fn index1() -> io::Result<NamedFile> {
    NamedFile::open("static/index1.html")
}
#[get("/search/<term>")]
pub fn response(term: &RawStr) -> String {
    format!("You typed in {}.", term)
}
