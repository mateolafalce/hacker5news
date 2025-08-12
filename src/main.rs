use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::net::TcpListener;

const PORT: u16 = 3000;
const HACKER_NEWS_API_URL: &str = "https://hacker-news.firebaseio.com";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index_handler))
        .with_state(Arc::new(AppState));
    let listener = TcpListener::bind(&format!("0.0.0.0:{}", PORT))
        .await
        .unwrap();

    println!("Listening on http://localhost:{}", PORT);
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone, Default)]
struct AppState;

#[derive(Deserialize)]
struct HNItem {
    title: String,
    url: Option<String>,
}

async fn index_handler(State(_): State<Arc<AppState>>) -> impl IntoResponse {
    let ids: Vec<u64> =
        match reqwest::get(HACKER_NEWS_API_URL.to_owned() + "/v0/topstories.json").await {
            Ok(resp) => resp.json().await.unwrap_or_default(),
            Err(_) => vec![],
        };

    let mut articles = Vec::new();
    for id in ids.into_iter().take(5) {
        let url = format!("{}/v0/item/{}.json", HACKER_NEWS_API_URL, id);
        if let Ok(resp) = reqwest::get(&url).await
            && let Ok(item) = resp.json::<HNItem>().await
        {
            articles.push(item);
        }
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Top 5 Hacker News</title>
    <style>
        body {{ 
            font-family: Arial, sans-serif; 
            max-width: 800px; 
            margin: 0 auto; 
            padding: 20px; 
        }}
        .header {{ 
            display: flex; 
            align-items: center; 
            margin-bottom: 30px; 
        }}
        .logo {{ 
            width: 40px; 
            height: 40px; 
            margin-right: 15px; 
        }}
        h1 {{ 
            margin: 0; 
            color: #ff6600; 
        }}
        ul {{ 
            list-style-type: none; 
            padding: 0; 
        }}
        li {{ 
            margin: 15px 0; 
            padding: 10px; 
            border-left: 3px solid #ff6600; 
            background-color: #f6f6ef; 
        }}
        a {{ 
            text-decoration: none; 
            color: #000; 
            font-weight: bold; 
        }}
        a:visited {{ 
            color: #828282; 
        }}
        a:hover {{ 
            color: #ff6600; 
        }}
        footer {{
            margin-top: 40px;
            text-align: center;
            color: #888;
            font-size: 0.95em;
        }}
        .github-link {{
            color: #24292f;
            text-decoration: none;
            display: inline-flex;
            align-items: center;
            gap: 6px;
        }}
        .github-link:hover {{
            color: #ff6600;
        }}
        .github-icon {{
            width: 18px;
            height: 18px;
            vertical-align: middle;
        }}
    </style>
    <link rel="icon" href="https://news.ycombinator.com/y18.svg">
</head>
<body>
    <div class="header">
        <img src="https://news.ycombinator.com/y18.svg" alt="Y Combinator" class="logo">
        <h1>Top 5 Hacker News Articles</h1>
    </div>
    <ul>
        {}
    </ul>
    <footer>
        <a class="github-link" href="https://github.com/mateolafalce/hacker5news" target="_blank" rel="noopener">
            <svg class="github-icon" viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
                <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38
                0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52
                -.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2
                -3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82a7.65
                7.65 0 0 1 2-.27c.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08
                2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01
                1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0 0 16 8c0-4.42-3.58-8-8-8z"/>
            </svg>
            GitHub
        </a>
    </footer>
</body>
</html>"#,
        articles
            .iter()
            .map(|article| {
                let title = &article.title;
                let url = article.url.as_deref().unwrap_or("#");
                format!("<li><a href=\"{}\">{}</a></li>", url, title)
            })
            .collect::<Vec<_>>()
            .join("")
    );

    Html(html)
}
