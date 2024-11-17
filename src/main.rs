mod handlers;
mod memory_database;
mod url_manager;
mod util;
use clap::Parser;

use axum::{routing::get, AddExtensionLayer, Router};
use handlers::{not_found, root_get, root_post, short_url, web_get};
use memory_database::MemoryDatabase;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(long, default_value_t = String::from("0.0.0.0"))]
    host: String,

    /// Number of times to greet
    #[arg(long, default_value_t = 3000)]
    port: u16,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();
    let db = Arc::new(MemoryDatabase::new());

    // Build the application with routes
    let app = Router::new()
        .route("/", get(root_get).post(root_post::<MemoryDatabase>))
        .route("/web", get(web_get))
        .route("/:short_url", get(short_url::<MemoryDatabase>))
        .route("/404", get(not_found))
        .layer(AddExtensionLayer::new(db));

    // Start the server
    let addr = SocketAddr::from_str(&format!("{}:{}", args.host, args.port).to_string())
        .expect("Failed to parse adress with given host and port");

    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
