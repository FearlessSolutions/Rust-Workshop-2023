use anyhow::Context;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub data: Vec<DocumentData>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DocumentData {
    pub id: String,
    pub title: String
}

pub fn list(client: &Client) -> anyhow::Result<Document> {
    // Return a list of documents from the Get Outline API.
    let res = client.post("https://app.getoutline.com/api/documents.list")
        .send().context("Unable to get documents.")?
        .error_for_status().context("Got bad HTTP status")?;

    let docs: Document = res.json().context("Can't find document.")?;

    // Error handling
    Ok(docs)
}