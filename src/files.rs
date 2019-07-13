use rocket::response::NamedFile;
use rocket::response::Redirect;
use std::path::{Path, PathBuf};
use std::io;
use rocket::http::RawStr;

#[get("/static/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}
#[get("/search/<term>")]
pub fn response(term: &RawStr) -> String {
    format!("You typed in {}.", term)
}
/*#[post("/search")]
pub fn compute() -> io::Result<NamedFile>{
    NamedFile::open("static/result.html")
} */
