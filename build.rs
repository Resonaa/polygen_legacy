use css_minify::optimizations::{Level, Minifier};
use minify_html::{minify as minify_html, Cfg};
use minify_js::minify as minify_js;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir("private/js")? {
        let dir = entry?;
        let file_name = dir.file_name().into_string().unwrap();

        let mut path = "public/js/".to_string();
        path.push_str(&file_name.replace(".js", ".min.js"));

        let source = fs::read_to_string(dir.path())?;
        let mut out = Vec::new();
        minify_js(source.into_bytes(), &mut out).unwrap();

        fs::write(path, out)?;
    }

    for entry in fs::read_dir("private/css")? {
        let dir = entry?;
        let file_name = dir.file_name().into_string().unwrap();

        let mut path = "public/css/".to_string();
        path.push_str(&file_name.replace(".css", ".min.css"));

        let source = fs::read_to_string(dir.path())?;
        let out = Minifier::default().minify(&source, Level::Two).unwrap();

        fs::write(path, out)?;
    }

    for suffix in ["", "partials/"] {
        for entry in fs::read_dir(format!("private/templates/{suffix}"))? {
            let dir = entry?;
            let file_name = dir.file_name().into_string().unwrap();
            println!("{}", file_name);

            if !file_name.ends_with("hbs") {
                continue;
            }

            let mut path = format!("templates/{suffix}");
            path.push_str(&file_name.replace(".html.hbs", ".min.html.hbs"));

            let source = fs::read_to_string(dir.path())?;
            let cfg = Cfg::default();
            let out = minify_html(&source.into_bytes(), &cfg);

            fs::write(path, out)?;
        }
    }

    Ok(())
}
