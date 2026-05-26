import { ApiTubeClient } from './client';

export interface Article {
    id: number;
    title: string | null;
    description: string | null;
    href: string | null;
    source: {
        id: number | null;
        domain: string;
        home_page_url: string;
    };
    published_at: string | null;
    language: string;
    categories: Array<{ id: number; name: string; score: number; taxonomy: string }>;
}

export interface NewsResponse {
    status: string;
    page: number;
    limit: number;
    results: Article[];
}

export interface NewsQueryParams {
    per_page?: number;
    page?: number;
    'language.code'?: string;
    'category.id'?: string;
    published_at?: string;
    'published_at.start'?: string;
    'published_at.end'?: string;
    'sort.by'?: 'published_at' | 'relevance' | 'engagement' | 'quality' | 'controversy' | 'trust';
    'sort.order'?: 'asc' | 'desc';
}

export class ApiTubeNewsService {
    private client: ApiTubeClient;

    constructor(client: ApiTubeClient) {
        this.client = client;
    }

    async getEverything(params: NewsQueryParams = {}): Promise<NewsResponse> {
        return this.client.request<NewsResponse>('/everything', { params });
    }

    async getTopHeadlines(params: NewsQueryParams = {}): Promise<NewsResponse> {
        return this.client.request<NewsResponse>('/top-headlines', { params });
    }

    async getArticlesByCategory(category: string, params: NewsQueryParams = {}): Promise<NewsResponse> {
        return this.client.request<NewsResponse>('/everything', {
            params: { ...params, 'category.id': category },
        });
    }

    async getArticlesByLanguage(language: string, params: NewsQueryParams = {}): Promise<NewsResponse> {
        return this.client.request<NewsResponse>('/everything', {
            params: { ...params, 'language.code': language },
        });
    }
}
