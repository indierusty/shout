use dotenvy::dotenv;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

type Db = Arc<RwLock<Database>>;

#[derive(Debug)]
struct Database {
    urls: HashMap<String, Details>,
    counter: usize,
}

#[derive(Deserialize, Debug, Clone)]
struct Details {
    url: String,
    // TODO: use strict type for this
    email: String,
}

impl Database {
    fn add_detail(&mut self, details: Details) -> String {
        let short_url = self.new_url();
        self.urls.insert(short_url.clone(), details);
        short_url
    }

    fn get_detail(&mut self, short_url: &String) -> Option<Details> {
        self.urls.get(short_url).cloned()
    }

    fn new_id(&mut self) -> usize {
        self.counter += 1;
        self.counter
    }

    fn new_url(&mut self) -> String {
        let id = self.new_id();
        id_to_short_url(id)
    }
}

// TODO: test this f()
fn id_to_short_url(mut id: usize) -> String {
    const BASE_LEN: usize = 62;
    const MAP: [char; BASE_LEN] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9',
    ];

    let mut short_url = "".to_string();

    while id > 0 {
        short_url.push(MAP[id % BASE_LEN]);
        id /= BASE_LEN;
    }

    short_url.chars().rev().collect()
}

impl Default for Database {
    fn default() -> Self {
        Self {
            urls: HashMap::default(),
            counter: 1_000_000,
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let db = Db::default();
    println!("{:#?}", db);
    // build our application with a route
    let app = Router::new()
        .route("/", post(make_url))
        .route("/:url", get(root))
        .with_state(db);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn make_url(State(db): State<Db>, Json(details): Json<Details>) -> String {
    db.write().unwrap().add_detail(details)
}

// basic handler that responds with a static string
async fn root(State(db): State<Db>, Path(short_url): Path<String>) {
    // let details = db.write().unwrap().get_detail(&short_url);
    if let Ok(mut urls) = db.write() {
        if let Some(d) = urls.get_detail(&short_url) {
            send_email(d);
        } else {
            println!("There is no url {:?}", short_url);
        }
    } else {
        println!("Failed to get write accces to db");
    }
}

fn send_email(details: Details) {
    let smtp_user_name = env::var("SMTP_USERNAME").expect("SMTP_USERNAME env var to be set");
    let smtp_key = env::var("SMTP_KEY").expect("SMTP_KEY env var to be set");

    let from_address = &smtp_user_name;
    let to_address = details.email;

    let email = Message::builder()
        .from(from_address.parse().unwrap())
        .to(to_address.parse().unwrap())
        .subject("Your Link".to_string())
        .body(details.url)
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
