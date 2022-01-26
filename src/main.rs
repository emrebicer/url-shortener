mod handlers;
mod memory_database;
mod url_manager;
mod util;

use axum::{routing::get, AddExtensionLayer, Router};
use handlers::{not_found, root_get, root_post, short_url, web_get};
use memory_database::MemoryDatabase;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Arc::new(MemoryDatabase::new());

    // Build the application with routes
    let app = Router::new()
        .route("/", get(root_get).post(root_post::<MemoryDatabase>))
        .route("/web", get(web_get))
        .route("/:short_url", get(short_url::<MemoryDatabase>))
        .route("/404", get(not_found))
        .layer(AddExtensionLayer::new(db));

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
