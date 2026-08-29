use std::env;

use axum::{extract::State, routing::post, Json, Router};
use email::EmailClient;
use serde::Deserialize;
use tower_http::cors::CorsLayer;

mod email;

const EMAIL_TEMPLATE: &str = r#"
<body>
    <h2>Congratulations!</h2>
    <p>By completing the Wheel of Vitality assessment, you’ve taken a valuable step towards understanding how much you already do to enhance your vitality, as well as which areas may benefit from more attention.</p>
    <p>Thank you for subscribing to the Vitality Clinic newsletter.</p>
    <p>But that’s not all! You can also take advantage of a <b>30% discount</b> on your first 60-minute Psychological Vitality Consultation with me. This session can help you clarify your goals and identify the barriers that may be holding you back from a more fulfilling life.</p>
    <p>To enquire about my availability, get in touch by forwarding your results to my address <span>yourvitalityclinic</span><span>@gmail</span><span>.com</span></p>
    <footer style="font-size: 12px; color: #888888; margin-top: 30px; padding-top: 15px; border-top: 1px solid #dddddd;">
        <p>© 2025 by Kat Wawer Dziedziak CPsychol. All rights reserved.</p>
        <p>Developer May Dziedziak</p>
    </footer>
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
    let port = env::var("PORT").unwrap();
    let client = EmailClient::new(api_key, secret_key);

    let app = Router::new()
        .route("/", post(root))
        .with_state(client)
        .layer(CorsLayer::permissive());

    let ip = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&ip).await.unwrap();
    println!("[INFO] Listening on `{}`", ip);
    axum::serve(listener, app).await.unwrap();
}
