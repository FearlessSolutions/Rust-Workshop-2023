
// create an https func
    // to request blocking client

pub mod documents;

use std::error::Error;
use std::fmt::{Display, Formatter};
use anyhow::Context;
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, HeaderMap};

#[derive(Debug)]
struct ExampleError {
    description: String,
}

impl Display for ExampleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Some example error happened! {}", self.description)
    }
}

impl Error for ExampleError {}

pub fn blocking_client (auth_token:&str) -> anyhow::Result<Client> {
    let mut headers = HeaderMap::new();

    // format!() macro
    // Need "Bearer ___"
    // println!("Some text {}", variable_name);
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", auth_token).parse().context("Couldn't make the bearer token string")?
    );

    let client = Client::builder()
        .default_headers(headers)
        .build();
    let actual_client = client.context("Failed to build the client!")?;

    Ok(actual_client)
}