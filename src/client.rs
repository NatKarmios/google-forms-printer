use anyhow::{Context, Result};
use colorize::AnsiColor;
use serde_json::Value as JSON;

pub struct FormsClient {
    client: reqwest::Client,
    form_id: String,
}

impl FormsClient {
    pub async fn new(client: &reqwest::Client, form_id: &str) -> Self {
        Self {
            client: client.clone(),
            form_id: form_id.into(),
        }
    }

    pub async fn get_form(&self) -> Result<JSON> {
        get_json(&self.client, &get_form_url(&self.form_id)).await
    }

    pub async fn get_responses(&self, min_timestamp: &str) -> Result<JSON> {
        get_json(
            &self.client,
            &get_responses_url(&self.form_id, min_timestamp),
        )
        .await
    }
}

const ENDPOINT: &str = "https://forms.googleapis.com/v1/forms";

fn get_form_url(form_id: &str) -> String {
    format!("{}/{}", ENDPOINT, form_id)
}

fn get_responses_url(form_id: &str, min_timestamp: &str) -> String {
    let query = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("filter", &format!("timestamp > {}", min_timestamp))
        .finish();
    format!("{}/{}/responses?{}", ENDPOINT, form_id, query)
}

async fn get_json(client: &reqwest::Client, url: &str) -> Result<JSON> {
    let response = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("Failed to GET {}\nIf this keeps happening, check your internet connection!", url).red())?;
    let json = response
        .json::<JSON>()
        .await
        .with_context(|| format!("Failed to parse JSON from {}\nSomething is very wrong!", url).red())?;
    Ok(json)
}
