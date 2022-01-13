mod shortener;
mod handlers;

use shortener::Shortener;
use handlers::{
    root_get_handler,
    web_get_handler,
    root_post_handler,
    short_url_handler,
    not_found_handler
};
use axum::{
    routing::get,
    Router,
    AddExtensionLayer
};
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let shorty = Arc::new(Shortener::new());

    // TODO: do I have to create an extension layer for each
    // parameter I want to pass into my handlers?
    // In my case, I only pass <shorty>, but what if
    // I had more variables that I want to use in my handlers?
    // maybe there is a more elegant way (single struct that holds several variables?)

    // Build the application with routes
    let app = Router::new()
        .route("/", get(root_get_handler).post(root_post_handler))
        .route("/web", get(web_get_handler))
        .route("/:short_url", get(short_url_handler))
        .route("/404", get(not_found_handler))
        .layer(AddExtensionLayer::new(shorty));

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
