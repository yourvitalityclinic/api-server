use reqwest::{header::CONTENT_TYPE, Client, Response};
use serde::Serialize;

const API_URL: &str = "https://api.mailjet.com/v3.1/send";

#[derive(Clone)]
pub struct EmailClient {
    api_key: String,
    secret_key: String,
    client: Client,
}

#[derive(Debug, Serialize)]
struct Person {
    #[serde(rename = "Email")]
    email: String,
    #[serde(rename = "Name")]
    name: String,
}
#[derive(Debug, Serialize)]
struct Attachment {
    #[serde(rename = "ContentType")]
    content_type: String,
    #[serde(rename = "Filename")]
    filename: String,
    #[serde(rename = "ContentID")]
    content_id: String,
    #[serde(rename = "Base64Content")]
    base64_content: String,
}
#[derive(Debug, Serialize)]
struct RequestMessage {
    #[serde(rename = "From")]
    from: Person,
    #[serde(rename = "To")]
    to: Vec<Person>,
    #[serde(rename = "Subject")]
    subject: String,
    #[serde(rename = "HTMLPart")]
    html_part: String,
    #[serde(rename = "InlinedAttachments")]
    attachments: Vec<Attachment>,
}
#[derive(Debug, Serialize)]
struct RequestFormat {
    #[serde(rename = "Messages")]
    messages: Vec<RequestMessage>,
}

impl EmailClient {
    pub fn new(api_key: String, secret_key: String) -> EmailClient {
        EmailClient {
            api_key,
            secret_key,
            client: Client::new(),
        }
    }

    pub async fn send_email(
        &self,
        email: String,
        name: String,
        subject: String,
        body: String,
        img_data: String,
    ) -> Result<Response, reqwest::Error> {
        let email_body = serde_json::to_string(&RequestFormat {
            messages: vec![RequestMessage {
                from: Person {
                    name: "Kat Wawer Dziedziak".into(),
                    email: "yourvitalityclinic@gmail.com".into(),
                },
                to: vec![Person { name, email }],
                subject,
                html_part: body,
                attachments: vec![Attachment {
                    content_type: "image/png".into(),
                    filename: "chart.png".into(),
                    base64_content: img_data,
                    content_id: "id1".into(),
                }],
            }],
        })
        .unwrap();

        self.client
            .post(API_URL)
            .header(CONTENT_TYPE, "application/json")
            .basic_auth(&self.api_key, Some(&self.secret_key))
            .body(email_body)
            .send()
            .await
    }
}
