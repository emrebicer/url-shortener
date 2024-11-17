use crate::url_manager::UrlManager;
use axum::{
    body::{Body, HttpBody},
    extract::{Extension, Path},
    http::{Request, StatusCode},
    response::Redirect,
    response::{Html, IntoResponse},
};
use std::str;
use std::sync::Arc;

pub async fn root_get() -> Redirect {
    // Redirect to /web
    Redirect::to("web".parse().unwrap())
}

pub async fn web_get() -> Html<&'static str> {
    // Return the web interface
    Html(include_str!("../public/web.html"))
}

pub async fn root_post<T: UrlManager>(
    Extension(manager): Extension<Arc<T>>,
    mut req: Request<Body>,
) -> Result<String, StatusCode> {
    // Generate a shortened url, return it
    let req_body = req.body_mut().data().await;
    match req_body {
        Some(some_url) => {
            let url_bytes = some_url.expect("body should consist of bytes");

            let mut url_str = str::from_utf8(&url_bytes)
                .expect("bytes should be convertable into a string")
                .to_string();

            // First check if origin exists, it is better because the protocol will be included
            let origin = req.headers().get("Origin");

            let host = req
                .headers()
                .get("host")
                .expect("HOST header must be present")
                .to_str()
                .expect("host header must be a valid string")
                .to_string();

            // If origin is not here, fallback to http + host
            let addr = match origin {
                Some(orig) => orig
                    .to_str()
                    .expect("origin header must be a valid string")
                    .to_string(),
                None => format!("{}/{}", "http://", host),
            };

            let shortened_url = format!("{}/{}", addr, manager.shorten_url(&mut url_str));
            return Ok(shortened_url);
        }
        None => {
            return Err(StatusCode::BAD_REQUEST);
        }
    }
}

pub async fn short_url<T: UrlManager>(
    Path(short_url_path): Path<String>,
    Extension(manager): Extension<Arc<T>>,
) -> Redirect {
    // Check if the short_url exists in the shortened urls, if so
    // return it, otherwise return 404 not found
    match manager.get_full_url(&short_url_path) {
        Some(full_url) => Redirect::to(full_url.parse().unwrap()),
        None => Redirect::to("/404".parse().unwrap()),
    }
}

pub async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
