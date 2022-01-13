use crate::shortener::Shortener;
use axum::{
    http::StatusCode,
    response::Html,
    response::Redirect,
    extract::{Path, Extension}
};
use std::sync::Arc;

pub async fn root_get_handler() -> Redirect {
    // Redirect to /web
    Redirect::to("web".parse().unwrap())
}

pub async fn web_get_handler() -> Html<&'static str> {
    // TODO: return the web interface (actual HTML)
    Html("<h1>Welcome to the web intarface, it will be implemented
        in the future!</h1>")
}

pub async fn root_post_handler(
        full_url: Option<String>,
        Extension(shorty): Extension<Arc<Shortener>>
    ) -> Result<String, StatusCode> {
    // Generate a shortened url, return it
    // TODO: instead of returning only the shortened path,
    // return domain + shortened path
    match full_url {
        Some(url) => {
            let shortened_url = shorty.shorten_url(&url);
            return Ok(shortened_url);
        },
        None => {
           return Err(StatusCode::BAD_REQUEST);
        }
    }
}

pub async fn short_url_handler(
        Path(short_url): Path<String>,
        Extension(shorty): Extension<Arc<Shortener>>
    ) -> Redirect {
    // Check if the short_url exists in the shortened urls, if so
    // return it, otherwise return 404 not found
    match shorty.get_full_url(&short_url) {
        Some(full_url) => Redirect::to(full_url.parse().unwrap()),
        None => Redirect::to("/".parse().unwrap()),
    }
}
