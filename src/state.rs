use anyhow::{Context, Result};
use printers::printer::Printer;
use yup_oauth2 as oauth2;
use colorize::AnsiColor;

use crate::{choose_printer, parse::*, Cfg, FormsClient, ParsedResponse};
use crate::util::*;

type Auth = oauth2::authenticator::Authenticator<oauth2::hyper_rustls::HttpsConnector<oauth2::hyper::client::HttpConnector>>;

pub struct State {
  pub client: FormsClient,
  pub cfg: Cfg,
  pub last_handled: String,
  pub printer: Printer,
}

impl State {
  pub async fn new() -> Result<Self> {
    let auth: Auth = get_auth().await?;
    let client = get_client(auth).await;
    let (cfg, client) = match Cfg::from_file(&client).await? {
        Some(c) => c,
        None => {
            let (cfg, client) = Cfg::make(&client).await?;
            cfg.save()?;
            (cfg, client)
        }
    };
    let last_handled = chrono::offset::Utc::now().to_rfc3339();
    let printer = choose_printer();
    Ok(State { client, cfg, last_handled, printer })
  }

  pub async fn get_responses(&self) -> Result<Vec<ParsedResponse>> {
    let raw_responses = self.client.get_responses(&self.last_handled).await?;
    parse_responses(&raw_responses, self)
  }
}

async fn get_headers(auth: Auth) -> reqwest::header::HeaderMap {
  let mut headers = reqwest::header::HeaderMap::new();
  let token = auth.token(SCOPES).await.unwrap().token().unwrap().to_owned();
  headers.insert(
      reqwest::header::AUTHORIZATION,
      format!("Bearer {}", token).parse().unwrap(),
  );
  headers
}

async fn get_client(auth: Auth) -> reqwest::Client {
  reqwest::Client::builder()
      .default_headers(get_headers(auth).await)
      .build()
      .unwrap()
}

async fn get_auth() -> Result<Auth> {
  let secret = oauth2::read_application_secret(SECRETS_FILE).await
    .with_context(|| format!("Couldn't read secret file - have you put your secrets in '{}'?", SECRETS_FILE).red())?;
  let auth = oauth2::InstalledFlowAuthenticator::builder(
      secret,
      oauth2::InstalledFlowReturnMethod::HTTPRedirect,
  ).persist_tokens_to_disk(TOKENS_FILE).build().await
    .with_context(|| format!("Couldn't read/write '{}'", TOKENS_FILE).red())?;
  Ok(auth)
}
