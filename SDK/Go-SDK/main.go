// main.go

package main

import (
    "fmt"
    "log"
    "apitube/config"
    "apitube/client"
    "apitube/news"
)

func main() {
    // Initialize the configuration with your API key
    cfg := config.NewConfig("YOUR_API_KEY")

    // Initialize the client
    c := client.NewClient(cfg)

    // Initialize the news service
    newsService := news.NewNewsService(c)

    // Define parameters
    params := map[string]string{
        "published_at.start": "NOW-1HOUR/HOUR",
        "language.code": "en",
        "category.id": "medtop:13000000,medtop:04000000",
        "per_page":     "5",
    }

    // Fetch articles
    articles, err := newsService.GetArticles(params)
    if err != nil {
        log.Fatalf("Error fetching articles: %v", err)
    }

    // Output the articles
    fmt.Println(articles)
}
