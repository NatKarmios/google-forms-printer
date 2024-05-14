use anyhow::{Context, Result};
use bimap::BiBTreeMap;
use colorize::AnsiColor;
use do_notation::m;
use serde::{Deserialize, Serialize};
use std::{
    fs::File, io::{stdin, BufReader}, path::Path
};

use crate::{util::*, FormsClient};

#[derive(Serialize, Deserialize)]
pub struct Cfg {
    pub form_id: String,
    pub name_question_id: String,
    pub company_question_id: String,
}

impl Cfg {
    pub async fn make(client: &reqwest::Client) -> Result<(Self, FormsClient)> {
        println!("Enter form ID:");
        let form_id = {
            let mut s = String::new();
            stdin().read_line(&mut s).unwrap();
            s.trim().to_owned()
        };

        let client = FormsClient::new(client, &form_id).await;

        let form = client.get_form().await?;
        let ids = get_question_map(&form);

        println!("Choose the question for the name:");
        let name_question_id = choose_question(&ids);

        println!("Choose the question for the company name:");
        let company_question_id = choose_question(&ids);

        let cfg = Self {
            form_id,
            name_question_id,
            company_question_id,
        };

        Ok((cfg, client))
    }

    pub async fn from_file(client: &reqwest::Client) -> Result<Option<(Self, FormsClient)>> {
        if !Path::exists(Path::new(CONFIG_FILE)) {
            return Ok(None);
        }
        let file = File::open(CONFIG_FILE)
            .with_context(|| format!("Couldn't open config file '{}'", CONFIG_FILE).red())?;
        let reader = BufReader::new(file);
        let form = serde_json::from_reader(reader).ok();
        match form {
            Some(form) => check_form(client, form).await,
            None => Ok(None),
        }
    }

    pub fn save(&self) -> Result<()> {
        let e = || format!("Couldn't save config to '{}'", CONFIG_FILE).red();
        let file = File::create(CONFIG_FILE).with_context(e)?;
        serde_json::to_writer(file, self).with_context(e)?;
        Ok(())
    }
}

fn choose_question(ids: &BiBTreeMap<String, String>) -> String {
    let keys: Vec<&String> = ids.left_values().collect();
    let key = choose(&keys);
    ids.get_by_left(key).unwrap().to_owned()
}

fn get_question_map(form: &serde_json::Value) -> BiBTreeMap<String, String> {
    let mut questions = BiBTreeMap::new();
    for item in form["items"].as_array().unwrap() {
        if let Some(id) = m! {
          q_item <- item.get("questionItem");
          q <- q_item.get("question");
          q_id <- q.get("questionId");
          q_id.as_str()
        } {
            let title = item["title"].as_str().unwrap();
            questions.insert(title.into(), id.into());
        }
    }
    questions
}

async fn check_form(client: &reqwest::Client, cfg: Cfg) -> Result<Option<(Cfg, FormsClient)>> {
    let client = FormsClient::new(client, &cfg.form_id).await;
    let form = client.get_form().await?;
    let title = form["info"]["documentTitle"].as_str().unwrap();
    let ids = get_question_map(&form);
    let (name_question, company_question) = match (
        ids.get_by_right(&cfg.name_question_id),
        ids.get_by_right(&cfg.company_question_id),
    ) {
        (None, _) | (_, None) => return Ok(None),
        (Some(name), Some(company)) => (name, company),
    };
    println!("Config found! Please confirm that the following information is correct:");
    println!(
        " Title: {}\n 'Name' question: {}\n 'Company' question: {}",
        title, name_question, company_question
    );
    println!("Is this correct? (Y/n)");
    let r = {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        s.trim().to_lowercase().to_owned()
    };
    if r != "n" {
        Ok(Some((cfg, client)))
    } else {
        Ok(None)
    }
}
