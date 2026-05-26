// ExampleUsage.java

import java.util.HashMap;
import java.util.Map;

public class ExampleUsage {
    public static void main(String[] args) {
        try {
            // Initialize the configuration with your API key
            ApiTubeConfig config = new ApiTubeConfig("YOUR_API_KEY");

            // Initialize the client
            ApiTubeClient client = new ApiTubeClient(config);

            // Initialize the service
            ApiTubeNewsService newsService = new ApiTubeNewsService(client);

            // Define parameters
            Map<String, String> params = new HashMap<>();
            params.put("published_at.start", "NOW-1HOUR/HOUR");
            params.put("language.code", "en");
            params.put("category.id", "medtop:13000000,medtop:04000000");
            params.put("per_page", "5");

            // Fetch articles
            String articles = newsService.getArticles(params);

            // Output the articles
            System.out.println(articles);

        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
