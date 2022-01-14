use crate::shortener::Shortener;
use axum::{
    http::{StatusCode, Request},
    response::{Html, IntoResponse},
    body::{Body, HttpBody},
    response::Redirect,
    extract::{Path, Extension}
};
use std::sync::Arc;
use std::str;

pub async fn root_get_handler() -> Redirect {
    // Redirect to /web
    Redirect::to("web".parse().unwrap())
}

pub async fn web_get_handler() -> Html<&'static str> {
    // Return the web interface
    Html(include_str!("../public/web.html"))
}

pub async fn root_post_handler(
        Extension(shorty): Extension<Arc<Shortener>>,
        mut req: Request<Body>,
    ) -> Result<String, StatusCode> {
    // Generate a shortened url, return it
    let req_body = req.body_mut().data().await;
    match req_body{
        Some(some_url) => {
            let url_bytes = some_url
                .expect("body should consist of bytes");

            let mut url_str = str::from_utf8(&url_bytes)
                .expect("bytes should be convertable into a string")
                .to_string();

            let host = req.headers().get("HOST")
                .expect("host should be included in the request headers")
                .to_str()
                .expect("host should be a string");

            let protocol = if host.contains("localhost:") { "http://" } else { "https://" };

            let shortened_url = 
                format!("{}{}/{}", protocol, host, shorty.shorten_url(&mut url_str));
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
        None => Redirect::to("/404".parse().unwrap()),
    }
}

pub async fn not_found_handler() -> impl IntoResponse {
   (StatusCode::NOT_FOUND, "nothing to see here")
}
