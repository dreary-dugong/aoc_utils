use std::collections::HashMap;

extern crate thiserror;
use thiserror::Error;

extern crate reqwest;
use reqwest::blocking::Client;
use reqwest::StatusCode;

extern crate regex;
use regex::Regex;

/// an error encountered while posting the answer
#[derive(Error, Debug)]
pub enum RequestError {
    #[error("unable to complete request to {0}: {1}")]
    RequestFailed(String, reqwest::Error),
    #[error("bad response from server with status code {0}")]
    BadResponse(u16),
    #[error("unable to find answer approval statement in response: {0}")]
    MissingApproval(String),
}

/// given the url and form params and a cookie, make a post request to submit the answer, return the resulting text or error
pub fn post_answer(
    year: u16,
    day: u8,
    level: u8,
    answer: &str,
    session_cookie: &str,
) -> Result<String, RequestError> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/answer");

    let mut form_params = HashMap::new();
    form_params.insert("level", level.to_string());
    form_params.insert("answer", answer.to_string());

    let client = Client::new();
    let response = client
        .post(&url)
        .header("Cookie", format!("session={session_cookie}"))
        .form(&form_params)
        .send()
        .map_err(|e| RequestError::RequestFailed(url, e))?;

    match response.status() {
        StatusCode::OK => parse_response(&response.text().unwrap()),
        other => Err(RequestError::BadResponse(other.as_u16())),
    }
}

/// given the raw html from an ok response, return the relevant first sentence
fn parse_response(resp: &str) -> Result<String, RequestError> {
    const PATTERN: &str = r"<article>\s*<p>[^\.]*\.";
    let reg = Regex::new(PATTERN).expect("couldn't make regex");
    if let Some(m) = reg.find(resp) {
        Ok(m.as_str()
            .strip_prefix("<article>")
            .unwrap()
            .trim()
            .strip_prefix("<p>")
            .unwrap()
            .trim()
            .to_string())
    } else {
        Err(RequestError::MissingApproval(resp.to_string()))
    }
}
