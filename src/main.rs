use junkyardTracker::firecrawl_client::FirecrawlClient;
use junkyardTracker::parser::parse_junkyard_page;
use std::env;

mod pick_n_pull;
use pick_n_pull::PicknPullSearch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv::dotenv().ok();
    
    // Get API key from environment variable
    let api_key = env::var("FIRECRAWL_API_KEY")
        .expect("FIRECRAWL_API_KEY environment variable not set");

    // Create FirecrawlClient
    let client = FirecrawlClient::new(api_key);

    // Create PicknPull search URL generator
    let pnp_search = PicknPullSearch::new();

    // Generate URL for Subaru Impreza Wagon search
    let search_url = pnp_search.subaru_impreza_wagon_search("95014", 50);
    println!("Searching URL: {}", search_url);

    // Crawl the webpage
    println!("Crawling webpage...");
    let response = client.crawl_webpage(&search_url).await?;
    println!("Response success: {}", response.success);

    // Parse the response
    let items = if let Some(data) = &response.data {
        if let Some(markdown) = &data.markdown {
            parse_junkyard_page(markdown, &search_url)
        } else {
            println!("No markdown data in response");
            Vec::new()
        }
    } else {
        println!("No data in response");
        Vec::new()
    };

    // Print the results
    println!("Found {} vehicles:", items.len());
    for item in items {
        println!(
            "{} {} {} - Location: {}",
            item.id,
            item.make,
            item.model,
            item.location.as_deref().unwrap_or("Unknown")
        );
    }

    Ok(())
}