use axum::{extract::Json, routing::post, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct WebhookData {
    event: String,
    data: serde_json::Value,
}

async fn handle_webhook(Json(payload): Json<WebhookData>) {
    println!("Received webhook: {:?}", payload);
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/webhook", post(handle_webhook));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
