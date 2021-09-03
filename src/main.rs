#![feature(proc_macro_hygiene, decl_macro)]
#![allow(non_snake_case)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rust_embed;

pub mod utils;

use std::{ffi::OsStr, io::Cursor, path::PathBuf};

use askama::Template;
use chrono::{Datelike, Local};

use lazy_static::lazy_static;
use rocket::{
    http::{ContentType, Status},
    response,
};
use rustc_version_runtime::version;


lazy_static! {
    static ref EXE: String = std::env::current_exe()
        .unwrap()
        .as_path()
        .to_string_lossy()
        .to_string();
    static ref VERSION: String = version().to_string();
}

#[derive(RustEmbed)]
#[folder = "public/"]
struct Static;


#[derive(Template)]
#[template(path = "index/index.html")]
struct IndexTemplate {
    title: String,
    year: String,
    path: String,
    version: String,
}



#[get("/")]
fn index() -> IndexTemplate {
    IndexTemplate {
        year: Local::now().date().year().to_string(),
        path: EXE.to_string(),
        version: VERSION.to_string(),
        title: "Vishesh Choudhary".to_owned(),
    }
}



#[get("/static/<file..>")]
fn public<'r>(file: PathBuf) -> response::Result<'r> {
    let filename = file.display().to_string();
    Static::get(&filename).map_or_else(
        || Err(Status::NotFound),
        |d| {
            let ext = file
                .as_path()
                .extension()
                .and_then(OsStr::to_str)
                .ok_or_else(|| Status::new(400, "Could not get file extension"))?;
            let content_type = ContentType::from_extension(ext)
                .ok_or_else(|| Status::new(400, "Could not get file content type"))?;
            response::Response::build()
                .header(content_type)
                .sized_body(Cursor::new(d))
                .ok()
        },
    )
}

#[get("/favicon.ico")]
fn favicon<'r>() -> response::Result<'r> {
    let icon = Static::get("favicon.ico").unwrap();
    let content_type = ContentType::Icon;
    response::Response::build()
        .header(content_type)
        .sized_body(Cursor::new(icon))
        .ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes!(index, public, favicon))
        .launch();
}
