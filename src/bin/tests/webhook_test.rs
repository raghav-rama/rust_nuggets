use axum::extract::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct WebhookData {
    pub event: String,
    pub data: serde_json::Value,
}

pub async fn handle_webhook(Json(payload): Json<WebhookData>) {
    println!("Received webhook: {:?}", payload);
}

#[cfg(test)]
mod webhook_test {
    use super::handle_webhook;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::post,
        Router,
    };
    use serde_json::json;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_webhook() {
        // Create a new router with the webhook handler
        let app = Router::new().route("/webhook", post(handle_webhook));

        // Create a JSON payload
        let payload = json!({
            "event": "test_event",
            "data": {
                "key": "value"
            }
        });

        // Create a request with the JSON payload
        let request = Request::builder()
            .method("POST")
            .uri("/webhook")
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&payload).unwrap()))
            .unwrap();

        // Send the request to the router
        let response = app.oneshot(request).await.unwrap();

        // Assert that the response status is 200 OK
        assert_eq!(response.status(), StatusCode::OK);

        // You can add more assertions here if your webhook handler returns a specific response
    }
}
