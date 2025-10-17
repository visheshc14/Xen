use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use askama::Template;
use chrono::{Datelike, Local, NaiveDate};
use comrak::{
    format_html, nodes::NodeValue, parse_document, Arena, ComrakExtensionOptions, ComrakOptions,
};
use lazy_static::lazy_static;
use markdown_meta_parser::MetaData;
use rocket::{
    get,
    fs::FileServer,
    http::ContentType,
    http::Status,
    Build, Rocket,
};
use rustc_version_runtime::version;

pub mod utils;
use crate::utils::{highlight_text, iter_nodes};


lazy_static! {
    static ref EXE: String = std::env::current_exe()
        .unwrap()
        .as_path()
        .to_string_lossy()
        .to_string();
    static ref VERSION: String = version().to_string();
    // Read posts directly from the repo's posts/ folder (no rebuild needed).
    static ref POSTS_DIR: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("posts");
}


#[derive(rust_embed::RustEmbed)]
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

#[derive(Clone)]
struct Post {
    date: String,
    title: String,
    slug: String,
    blurb: String,
    tags: Vec<String>,
}

#[derive(Template)]
#[template(path = "blog/index.html")]
struct BlogTemplate {
    title: String,
    year: String,
    posts: Vec<Post>,
    path: String,
    version: String,
}

#[derive(Template)]
#[template(path = "blog/post.html")]
struct PostTemplate {
    title: String,
    year: String,
    post: String,
    path: String,
    version: String,
}



fn comrak_opts() -> ComrakOptions {
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
    opts
}

fn read_to_string(p: &Path) -> Result<String, Status> {
    fs::read_to_string(p).map_err(|_| Status::InternalServerError)
}

fn list_markdown_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(rd) = fs::read_dir(dir) {
        for entry in rd.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                files.push(path);
            }
        }
    }
    files
}

fn slug_from_path(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_string()
}


fn normalize_lang(lang: &str) -> String {
    match lang.trim().to_ascii_lowercase().as_str() {
        "" => "".to_string(),
        "rust" | "rs" => "rs".to_string(),
        "js" | "javascript" => "javascript".to_string(),
        "ts" | "typescript" => "typescript".to_string(),
        "py" | "python" => "python".to_string(),
        "sh" | "bash" | "zsh" => "bash".to_string(),
        "c++" | "cpp" => "cpp".to_string(),
        "c" => "c".to_string(),
        "html" => "html".to_string(),
        "css" => "css".to_string(),
        other => other.to_string(),
    }
}



#[get("/")]
fn index() -> IndexTemplate {
    IndexTemplate {
        year: Local::now().date_naive().year().to_string(),
        path: EXE.to_string(),
        version: VERSION.to_string(),
        title: "Vishesh Choudhary".to_owned(),
    }
}

#[get("/blog")]
fn blog() -> BlogTemplate {
    let opts = comrak_opts();

    let mut posts: Vec<Post> = Vec::new();

    for file in list_markdown_files(&POSTS_DIR) {
        let Ok(content) = read_to_string(&file) else { continue };

        
        let mut type_mark = HashMap::new();
        type_mark.insert("tags".into(), "array");

        let meta = MetaData {
            content,
            required: vec![
                "title".to_owned(),
                "tags".to_owned(),
                "date".to_owned(),
                "blurb".to_owned(),
            ],
            type_mark,
        };

        let Ok((parsed_meta, body_or_blurb_md)) = meta.parse() else { continue };

        let title = match parsed_meta["title"].clone() {
            markdown_meta_parser::Value::String(t) => t.replace('\'', ""),
            _ => "".to_owned(),
        };

        let date = match parsed_meta["date"].clone() {
            markdown_meta_parser::Value::String(d) => d,
            _ => "".to_owned(),
        };

        let blurb_front = match parsed_meta["blurb"].clone() {
            markdown_meta_parser::Value::String(b) => b.replace('"', ""),
            _ => "".to_owned(),
        };

        let tags = match parsed_meta["tags"].clone() {
            markdown_meta_parser::Value::Array(t) => t,
            _ => vec![],
        };

        
        let blurb_src = if blurb_front.trim().is_empty() {
            body_or_blurb_md
        } else {
            blurb_front
        };

       
        let arena = Arena::new();
        let root = parse_document(&arena, &blurb_src, &opts);
        let mut blurb_html = Vec::new();
        format_html(root, &opts, &mut blurb_html).unwrap();

        posts.push(Post {
            date,
            title,
            slug: slug_from_path(&file),
            blurb: String::from_utf8(blurb_html).unwrap(),
            tags,
        });
    }

    
    posts.sort_by(|a, b| {
        let da = NaiveDate::parse_from_str(&a.date, "%Y-%m-%d")
            .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
        let db = NaiveDate::parse_from_str(&b.date, "%Y-%m-%d")
            .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
        db.cmp(&da)
    });

    BlogTemplate {
        year: Local::now().date_naive().year().to_string(),
        posts,
        path: EXE.to_string(),
        version: VERSION.to_string(),
        title: "Blog - Vishesh Choudhary".to_owned(),
    }
}

#[get("/blog/<slug>")]
fn get_blog(slug: String) -> Result<PostTemplate, Status> {
    let path = POSTS_DIR.join(format!("{slug}.md"));
    let content = read_to_string(&path)?;

    
    let mut type_mark = HashMap::new();
    type_mark.insert("tags".into(), "array");

    let meta = MetaData {
        content,
        required: vec![
            "title".to_owned(),
            "tags".to_owned(),
            "date".to_owned(),
            "blurb".to_owned(),
        ],
        type_mark,
    };

    let (parsed_meta, body_md) = meta.parse().map_err(|_| Status::InternalServerError)?;

    let title = match parsed_meta["title"].clone() {
        markdown_meta_parser::Value::String(t) => t,
        _ => "".to_owned(),
    };

  
    let opts = comrak_opts();
    let arena = Arena::new();
    let root = parse_document(&arena, &body_md, &opts);

    
    iter_nodes(root, &|node| {
      
        let (lang_raw, code_opt): (String, Option<String>) = {
            let borrowed = node.data.borrow(); // immutable borrow
            if let NodeValue::CodeBlock(block) = &borrowed.value {
                let lang = String::from_utf8(block.info.clone()).unwrap_or_default();
                let code = String::from_utf8(block.literal.clone()).unwrap_or_default();
                (lang, Some(code))
            } else {
                (String::new(), None)
            }
        }; 

        if let Some(code) = code_opt {
            let lang_norm = normalize_lang(&lang_raw);
            let inner = highlight_text(code, lang_norm.clone()); // returns HTML spans etc.
            let html = format!(
                r#"<pre class="code-toolbar"><code class="language-{}">{}</code></pre>"#,
                lang_norm, inner
            );
            node.data.borrow_mut().value = NodeValue::HtmlInline(html.into_bytes());
        }
    });

    
    let mut html = Vec::new();
    format_html(root, &opts, &mut html).map_err(|_| Status::InternalServerError)?;

    Ok(PostTemplate {
        title,
        year: Local::now().date_naive().year().to_string(),
        post: String::from_utf8(html).unwrap(),
        path: EXE.to_string(),
        version: VERSION.to_string(),
    })
}

#[get("/favicon.ico")]
fn favicon() -> Option<(ContentType, Vec<u8>)> {
    Static::get("favicon.ico").map(|icon| (ContentType::Icon, icon.data.into_owned()))
}

#[rocket::launch]
fn rocket() -> Rocket<Build> {
    // Bind correctly for Render (0.0.0.0 and PORT)
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000);

    let config = rocket::Config {
        address: std::net::Ipv4Addr::UNSPECIFIED.into(), // 0.0.0.0
        port,
        ..rocket::Config::default()
    };

    rocket::custom(config)
        .mount("/", rocket::routes![index, blog, get_blog, favicon])
        .mount("/static", FileServer::from("public"))
}