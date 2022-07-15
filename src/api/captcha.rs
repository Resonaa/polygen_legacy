use ::captcha::{filters::Wave, Captcha};
use rocket::http::{Cookie, CookieJar};

#[derive(Responder)]
#[response(status = 200, content_type = "image/png")]
struct Png(Vec<u8>);

#[get("/captcha")]
async fn captcha(jar: &CookieJar<'_>) -> Option<Png> {
    let mut captcha = Captcha::new();
    captcha
        .add_chars(4)
        .apply_filter(Wave::new(2.0, 20.0).horizontal())
        .apply_filter(Wave::new(2.0, 20.0).vertical())
        .view(220, 120);
    jar.add_private(Cookie::new("captcha", captcha.chars_as_string()));

    captcha.as_png().map(Png)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![captcha]
}
