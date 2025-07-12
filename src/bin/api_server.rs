use junkyardTracker::api::create_app;
use junkyardTracker::firecrawl_client::FirecrawlClient;
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv::dotenv().ok();
    
    // Get API key from environment variable
    let api_key = env::var("FIRECRAWL_API_KEY")
        .expect("FIRECRAWL_API_KEY environment variable not set");

    // Create FirecrawlClient
    let firecrawl_client = FirecrawlClient::new(api_key);

    // Create the app with routes
    let app = create_app(firecrawl_client);

    // Get port from environment or default to 3000
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    println!("🚗 Junkyard Tracker API starting on http://{}", addr);
    println!("📋 Available endpoints:");
    println!("  POST /search - Search for vehicles");
    println!("  GET  /search?make=<make>&model=<model>&year_min=<year>&year_max=<year>&zip_code=<zip> - Search for vehicles (GET)");
    println!("  GET  /health - Health check");
    println!("  GET  /supported-makes - Get supported makes");
    println!("  GET  /supported-models?make=<make> - Get supported models for a make");

    // Create listener and serve the app
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
