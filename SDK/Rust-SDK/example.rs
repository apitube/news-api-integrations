use apitube::{ApiTubeConfig, ApiTubeClient, ApiTubeNewsService, NewsQueryParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize configuration
    let config = ApiTubeConfig::new("YOUR_API_KEY");

    // Create client and service
    let client = ApiTubeClient::new(config)?;
    let news_service = ApiTubeNewsService::new(&client);

    // Fetch latest news
    println!("Fetching latest news...");
    let params = NewsQueryParams::new().per_page(10);
    let latest_news = news_service.get_everything(params).await?;

    println!("Found {} articles", latest_news.results.len());
    for article in &latest_news.results {
        println!("- {}", article.title.as_deref().unwrap_or("(untitled)"));
    }

    // Fetch news by category
    println!("\nFetching technology news...");
    let params = NewsQueryParams::new().per_page(5);
    let tech_news = news_service.get_by_category("medtop:13000000", params).await?;

    for article in &tech_news.results {
        println!("- {}", article.title.as_deref().unwrap_or("(untitled)"));
    }

    // Fetch news by language
    println!("\nFetching English news...");
    let params = NewsQueryParams::new().per_page(5);
    let english_news = news_service.get_by_language("en", params).await?;

    for article in &english_news.results {
        println!("- {}", article.title.as_deref().unwrap_or("(untitled)"));
    }

    Ok(())
}
