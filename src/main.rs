use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use serde::Deserialize;
use std::env::consts::OS;
use std::process::Command;
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

    open_browser(&format!("http://localhost:{}", PORT));

    axum::serve(listener, app).await.unwrap();
}

fn open_browser(url: &str) {
    let result = match OS {
        "linux" | "macos" => Command::new("xdg-open").arg(url).spawn(),
        "windows" => Command::new("cmd").args(["/C", "start", url]).spawn(),
        _ => {
            eprintln!("Cannot automatically open the browser on this OS.");
            return;
        }
    };
    if let Err(e) = result {
        eprintln!("Failed to open the browser: {}", e);
    }
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
</body>
</html>"#,
        articles
            .iter()
            .map(|article| {
                let title = &article.title;
                let url = article.url.as_deref().unwrap_or("#");
                format!(
                    "<li><a href=\"{}\" target=\"_blank\">{}</a></li>",
                    url, title
                )
            })
            .collect::<Vec<_>>()
            .join("")
    );

    Html(html)
}
