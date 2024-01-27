use std::env;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use dotenvy::dotenv;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use shout::{Db, FullUrl};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // TODO: compile time checkk for .env using dotenvy_macro crate
    dotenv().expect(".env file not found");

    let db = Db::default();
    let app = Router::new()
        .route("/:url", get(visit_url))
        .route("/api/url", post(create_url))
        .with_state(db)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4444").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn visit_url(State(db): State<Db>, Path(short_url): Path<String>) -> StatusCode {
    println!("path {:?}", short_url); // DEBUG:

    if let Ok(d) = db.read() {
        if let Some(url) = d.get_url(&short_url) {
            send_email(url);
            return StatusCode::OK;
        }
    }

    StatusCode::NOT_FOUND
}

async fn create_url(
    State(db): State<Db>,
    Json(full_url): Json<FullUrl>,
) -> Result<(StatusCode, Json<ShortUrl>), ()> {
    println!("{:#?}", full_url); //DEBUG:

    if let Ok(mut d) = db.write() {
        let short_url = d.add_url(full_url);
        return Ok((StatusCode::CREATED, Json(ShortUrl { url: short_url })));
    } else {
        Err(())
    }
}

#[derive(Deserialize, Serialize)]
struct ShortUrl {
    url: String,
}

fn send_email(details: FullUrl) {
    let smtp_user_name = env::var("SMTP_USERNAME").expect("SMTP_USERNAME env var to be set");
    let smtp_key = env::var("SMTP_KEY").expect("SMTP_KEY env var to be set");

    let from_address = &smtp_user_name;
    let to_address = details.email();

    let email = Message::builder()
        .from(from_address.parse().unwrap())
        .to(to_address.parse().unwrap())
        .subject("Your Link".to_string())
        .body(details.url())
        .unwrap();

    let creds = Credentials::new(smtp_user_name, smtp_key);

    // Open a remote connection to SMTP server
    let host = env::var("SMTP_HOST").expect("SMTP_HOST is not set");

    let mailer = SmtpTransport::relay(&host)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
