
// create an https func
    // to request blocking client

use std::error::Error;
use std::fmt::{Display, Formatter};
use anyhow::Context;
use reqwest::blocking::Client;

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

fn blocking_client () -> anyhow::Result<Client> {
    let client = Client::builder().build();
    let actual_client = client.context("Failed to build the client!")?;

    Ok(actual_client)
}