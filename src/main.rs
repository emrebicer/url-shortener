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
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let shorty = Arc::new(Shortener::new());

    // Build the application with routes
    let app = Router::new()
        .route("/", get(root_get_handler).post(root_post_handler))
        .route("/web", get(web_get_handler))
        .route("/:short_url", get(short_url_handler))
        .route("/404", get(not_found_handler))
        .layer(AddExtensionLayer::new(shorty));

    // Try to get the port from environment variables
    let port = match env::var("PORT") {
        Ok(port) => port.parse::<u16>()?,
        Err(_) => 3000,
    };

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
