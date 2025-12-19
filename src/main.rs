use axum::{
    Router,
    extract::{Path, State},
    http::{StatusCode, header},
    response::IntoResponse,
    routing::get,
};
use dotenvy::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
struct CoolifyResponse {
    deployments: Vec<Deployment>,
}

#[derive(Deserialize)]
struct Deployment {
    status: String,
}

struct AppState {
    client: Client,
    coolify_url: String,
    api_token: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let state = Arc::new(AppState {
        client: Client::new(),
        coolify_url: std::env::var("COOLIFY_URL").expect("COOLIFY_URL must be set"),
        api_token: std::env::var("API_TOKEN").expect("API_TOKEN must be set"),
    });

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let app = Router::new()
        .route("/", get(|| async { "Badge service is alive!" }))
        .route("/health", get(|| async { (StatusCode::OK, "OK") }))
        .route("/badge/:app_uuid", get(get_badge))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Badge service listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_badge(
    Path(app_uuid): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let url = format!(
        "{}/api/v1/deployments/applications/{}",
        state.coolify_url, app_uuid
    );

    let response = state
        .client
        .get(&url)
        .bearer_auth(&state.api_token)
        .send()
        .await;

    let status = match response {
        Ok(resp) => {
            if resp.status() == 401 {
                "unauthorized".to_string()
            } else {
                // Parse into our new wrapper structure
                match resp.json::<CoolifyResponse>().await {
                    Ok(data) => data
                        .deployments
                        .first()
                        .map(|d| d.status.clone())
                        .unwrap_or_else(|| "no_history".to_string()),
                    Err(e) => {
                        eprintln!("JSON Parsing error: {}", e);
                        "parse_error".to_string()
                    }
                }
            }
        }
        Err(_) => "offline".to_string(),
    };

    // Mapping logic remains the same
    let color = match status.as_str() {
        "finished" => "#3fb950",
        "failed" => "#f85149",
        "in_progress" => "#2188ff",
        "queued" => "#6e7681",
        "no_history" => "#d1d5da",
        _ => "#cea61b",
    };

    let status_text_width = (status.len() * 8).max(40);
    let total_width = 50 + status_text_width;
    let status_center_x = 50 + (status_text_width / 2);

    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{total_width}" height="20">
            <linearGradient id="b" x2="0" y2="100%">
                <stop offset="0" stop-color="#bbb" stop-opacity=".1"/>
                <stop offset="1" stop-opacity=".1"/>
            </linearGradient>
            <mask id="a">
                <rect width="{total_width}" height="20" rx="3" fill="#fff"/>
            </mask>
            <g mask="url(#a)">
                <path fill="#555" d="M0 0h50v20H0z"/>
                <path fill="{color}" d="M50 0h{status_text_width}v20H50z"/>
                <path fill="url(#b)" d="M0 0h{total_width}v20H0z"/>
            </g>
            <g fill="#fff" text-anchor="middle" font-family="Verdana,DejaVu Sans,sans-serif" font-size="11">
                <text x="25" y="15" fill="#010101" fill-opacity=".3">deploy</text>
                <text x="25" y="14">deploy</text>
                <text x="{status_center_x}" y="15" fill="#010101" fill-opacity=".3">{status}</text>
                <text x="{status_center_x}" y="14">{status}</text>
            </g>
        </svg>"##,
        total_width = total_width,
        status_text_width = status_text_width,
        status_center_x = status_center_x,
        color = color,
        status = status
    );

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "image/svg+xml"),
            (header::CACHE_CONTROL, "no-cache, no-store, must-revalidate"),
        ],
        svg,
    )
}
