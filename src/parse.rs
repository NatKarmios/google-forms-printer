use anyhow::{Context, Result};
use colored::Colorize;
use tryvial::try_block;

use crate::State;

#[derive(Clone)]
pub struct ParsedResponse {
    pub name: String,
    pub company: String,
    pub date: String,
}

pub fn parse_answer(val: &serde_json::Value) -> Result<String> {
    let answer: Option<String> = try_block! {
      val.get("textAnswers")?
        .get("answers")?
        .get(0)?
        .get("value")?
        .as_str()?
        .to_owned()
    };
    answer.context("Failed to parse answer".red())
}

fn parse_response(raw_response: &serde_json::Value, state: &State) -> Result<ParsedResponse> {
    let response: Result<ParsedResponse> = try_block! {
      let name = parse_answer(&raw_response["answers"][&state.cfg.name_question_id])?;
      let company = parse_answer(&raw_response["answers"][&state.cfg.company_question_id])?;
      let date = raw_response["lastSubmittedTime"].as_str()
        .context("Date doesn't exist on response".red())?
        .into();
      ParsedResponse { name, company, date }
    };
    response.with_context(|| {
        format!(
            "Failed to parse response\n{}",
            serde_json::to_string_pretty(&raw_response).unwrap()
        )
        .red()
    })
}

pub fn parse_responses(
    raw_responses: &serde_json::Value,
    state: &State,
) -> Result<Vec<ParsedResponse>> {
    let mut responses = Vec::new();
    if let Some(raw_responses) = raw_responses["responses"].as_array() {
        for raw_response in raw_responses {
            let response = parse_response(raw_response, state)?;
            responses.push(response);
        }
        Ok(responses)
    } else {
        Ok(Vec::new())
    }
}
