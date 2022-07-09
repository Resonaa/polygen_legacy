use css_minify::optimizations::{Level, Minifier};
use minify_html::{minify as minify_html, Cfg};
use minify_js::minify as minify_js;
use std::fs;

fn main() {
    for entry in fs::read_dir("private/js").unwrap() {
        let dir = entry.unwrap();
        let file_name = dir.file_name().into_string().unwrap();

        let mut path = "public/js/".to_string();
        path.push_str(&file_name.replace(".js", ".min.js"));

        let source = fs::read_to_string(dir.path()).unwrap();
        let mut out = Vec::new();
        minify_js(source.into_bytes(), &mut out).unwrap();

        fs::write(path, out).unwrap();
    }

    for entry in fs::read_dir("private/css").unwrap() {
        let dir = entry.unwrap();
        let file_name = dir.file_name().into_string().unwrap();

        let mut path = "public/css/".to_string();
        path.push_str(&file_name.replace(".css", ".min.css"));

        let source = fs::read_to_string(dir.path()).unwrap();
        let out = Minifier::default().minify(&source, Level::Two).unwrap();

        fs::write(path, out).unwrap();
    }

    for entry in fs::read_dir("private/templates/").unwrap() {
        let dir = entry.unwrap();
        let file_name = dir.file_name().into_string().unwrap();

        let mut path = "templates/".to_string();
        path.push_str(&file_name.replace(".html.hbs", ".min.html.hbs"));

        let source = fs::read_to_string(dir.path()).unwrap();
        let cfg = Cfg::default();
        let out = minify_html(&source.into_bytes(), &cfg);

        fs::write(path, out).unwrap();
    }
}
