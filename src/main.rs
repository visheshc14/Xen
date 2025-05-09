#![feature(proc_macro_hygiene, decl_macro)]
#![allow(non_snake_case)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rust_embed;

pub mod utils;

use std::{ffi::OsStr, path::PathBuf};

use askama::Template;
use chrono::{Datelike, Local};
use comrak::{
    format_html, nodes::NodeValue, parse_document, Arena, ComrakExtensionOptions, ComrakOptions,
};
use lazy_static::lazy_static;
use rocket::{
    http::{ContentType, Status},
    response::content::RawHtml,
};
use rustc_version_runtime::version;

use crate::utils::{highlight_text, iter_nodes};

lazy_static! {
    static ref VERSION: String = version().to_string();
}

#[derive(RustEmbed)]
#[folder = "public/"]
struct Static;

#[derive(RustEmbed)]
#[folder = "posts/"]
struct Posts;

#[derive(Template)]
#[template(path = "index/index.html")]
struct IndexTemplate {
    title: String,
    year: String,
    version: String,
}

struct Post {
    date: String,
    title: String,
    slug: String,
}

#[derive(Template)]
#[template(path = "blog/index.html")]
struct BlogTemplate {
    title: String,
    year: String,
    posts: Vec<Post>,
    version: String,
}

#[derive(Template)]
#[template(path = "blog/post.html")]
struct PostTemplate {
    title: String,
    year: String,
    post: String,
    version: String,
}

#[get("/")]
fn index() -> RawHtml<String> {
    let template = IndexTemplate {
        year: Local::now().date_naive().year().to_string(),
        version: VERSION.to_string(),
        title: "Vishesh Choudhary".to_owned(),
    };
    RawHtml(template.render().unwrap())
}

#[get("/blog")]
fn blog() -> RawHtml<String> {
    let post_list: Vec<_> = Posts::iter()
        .map(|f| {
            let slug = f.as_ref();
            let split: Vec<_> = slug.splitn(2, '_').collect();
            Post {
                date: split[0].to_owned(),
                title: split[1].replace("-", " ").replace(".md", ""),
                slug: slug.to_owned().replace(".md", ""),
            }
        })
        .collect();

    let template = BlogTemplate {
        year: Local::now().date_naive().year().to_string(),
        posts: post_list,
        version: VERSION.to_string(),
        title: "Blog - Vishesh Choudhary".to_owned(),
    };
    RawHtml(template.render().unwrap())
}

#[get("/blog/<file>")]
fn get_blog(file: String) -> Result<RawHtml<String>, Status> {
    let filename = format!("{}.md", file);
    Posts::get(&filename).map_or_else(
        || Err(Status::NotFound),
        |d| {
            let post_text = String::from_utf8(d.as_ref().to_vec()).unwrap();
            let mut opts = ComrakOptions::default();
            opts.extension = ComrakExtensionOptions {
                strikethrough: true,
                tagfilter: false,
                table: true,
                autolink: true,
                tasklist: true,
                superscript: false,
                header_ids: Some("#".to_string()),
                footnotes: false,
                description_lists: false,
                front_matter_delimiter: None,
            };
            opts.render.unsafe_ = true;

            let arena = Arena::new();
            let root = parse_document(&arena, &post_text, &opts);
            iter_nodes(root, &|node| match &mut node.data.borrow_mut().value {
                NodeValue::CodeBlock(ref mut block) => {
                    let lang = String::from_utf8(block.info.clone()).unwrap();
                    let code = String::from_utf8(block.literal.clone()).unwrap();
                    block.literal = highlight_text(code, lang).as_bytes().to_vec();
                }
                _ => (),
            });

            let mut html = vec![];
            format_html(root, &opts, &mut html).unwrap();
            let template = PostTemplate {
                year: Local::now().date_naive().year().to_string(),
                post: String::from_utf8(html).unwrap(),
                version: VERSION.to_string(),
                title: file.splitn(2, '_').collect::<Vec<_>>()[1]
                    .to_owned()
                    .replace('-', " "),
            };
            Ok(RawHtml(template.render().unwrap()))
        },
    )
}

#[get("/static/<file..>")]
fn public(file: PathBuf) -> Result<(ContentType, Vec<u8>), Status> {
    let filename = file.display().to_string();
    Static::get(&filename).map_or_else(
        || Err(Status::NotFound),
        |d| {
            let ext = file
                .as_path()
                .extension()
                .and_then(OsStr::to_str)
                .ok_or(Status::BadRequest)?;
            let content_type = ContentType::from_extension(ext).ok_or(Status::BadRequest)?;
            Ok((content_type, d.to_vec()))
        },
    )
}

#[get("/favicon.ico")]
fn favicon() -> Result<(ContentType, Vec<u8>), Status> {
    let icon = Static::get("favicon.ico").ok_or(Status::NotFound)?;
    Ok((ContentType::Icon, icon.to_vec()))
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000);

    let config = rocket::Config {
        port,
        address: std::net::Ipv4Addr::new(0, 0, 0, 0).into(),
        ..rocket::Config::default()
    };

    let _rocket = rocket::custom(config)
        .mount("/", routes!(index, public, blog, get_blog, favicon))
        .ignite().await?
        .launch().await?;

    Ok(())
}
