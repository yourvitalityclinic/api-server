use std::env;

use axum::{extract::State, routing::post, Json, Router};
use email::EmailClient;
use serde::Deserialize;
use tower_http::cors::CorsLayer;

mod email;

const EMAIL_TEMPLATE: &str = r#"
<body>
    <h1>Your Vitality Wheel</h1>
    <img style="width: 100vw;" src="cid:id1" alt="Your Vitality Wheel"  />
    <p>Thank you for using my vitality wheel.<p>
</body>
"#;

#[derive(Debug, Deserialize)]
struct FormData {
    name: String,
    email: String,
    img_string: String,
}

async fn root(State(client): State<EmailClient>, Json(payload): Json<FormData>) -> String {
    println!(
        "[INFO] Received request from {}|{}",
        payload.name, payload.email
    );

    client
        .send_email(
            payload.email,
            payload.name,
            "Your Vitality Wheel".into(),
            EMAIL_TEMPLATE.into(),
            payload.img_string,
        )
        .await
        .unwrap();

    println!("[INFO] Email sent successfuly");
    String::new()
}

#[tokio::main]
async fn main() {
    let api_key = env::var("MAILJET_API_KEY").unwrap();
    let secret_key = env::var("MAILJET_SECRET_KEY").unwrap();
    let client = EmailClient::new(api_key, secret_key);

    let app = Router::new()
        .route("/", post(root))
        .with_state(client)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("[INFO] Listening on `{}`", "0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
