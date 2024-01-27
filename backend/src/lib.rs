use serde::Deserialize;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Deserialize, Debug, Clone)]
pub struct FullUrl {
    url: String,
    // TODO: use strict type for this
    email: String,
}

impl FullUrl {
    pub fn new(url: String, email: String) -> Self {
        Self { url, email }
    }

    pub fn email(&self) -> String {
        self.email.to_string()
    }

    pub fn url(&self) -> String {
        self.url.to_string()
    }
}

pub type Db = Arc<RwLock<Database>>;

#[derive(Debug)]
pub struct Database {
    urls: HashMap<String, FullUrl>,
    counter: usize,
}

impl Database {
    pub fn add_url(&mut self, url: FullUrl) -> String {
        let short_url = self.new_url();
        self.urls.insert(short_url.clone(), url);
        short_url
    }

    pub fn get_url(&self, short_url: &String) -> Option<FullUrl> {
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
