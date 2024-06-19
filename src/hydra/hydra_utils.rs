use reqwest::{header::ACCEPT,blocking::{Client,Response}};

use super::hydra_api_schema::Result;

pub fn get_wrapper(url: &str) -> Result<Response> {
    Client::new()
        .get(url)
        .header(ACCEPT, "application/json")
        .send()
}
