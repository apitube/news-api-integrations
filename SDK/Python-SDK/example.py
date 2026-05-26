# example_usage.py

from config import ApiTubeConfig
from client import ApiTubeClient
from news_service import ApiTubeNewsService

# Initialize the configuration with your API key
config = ApiTubeConfig(api_key='YOUR_API_KEY')

# Initialize the client
client = ApiTubeClient(config)

# Initialize the service
news_service = ApiTubeNewsService(client)

# Define parameters
params = {
    'published_at.start': 'NOW-1HOUR/HOUR',
    'language.code': 'en',
    'category.id': 'medtop:13000000,medtop:04000000',
    'per_page': 5,
}

# Fetch articles
articles = news_service.get_articles(params)

# Output the articles
print(articles)
