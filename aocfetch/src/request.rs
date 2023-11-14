extern crate thiserror;
use thiserror::Error;

extern crate reqwest;
use reqwest::blocking::Client;
use reqwest::StatusCode;

/// an error encountered while making the request for input
#[derive(Error, Debug)]
pub enum RequestError {
    #[error("unable to complete request to {0}")]
    RequestFailed(String, reqwest::Error),
    #[error("bad response from server with status code {0}")]
    BadResponse(u16),
}

/// given url params and a cookie, make a request for the day's input and return the text or error
pub fn request_input(year: u16, day: u8, session_cookie: &str) -> Result<String, RequestError> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let client = Client::new();
    let response = client
        .get(&url)
        .header("Cookie", format!("session={session_cookie}"))
        .send()
        .map_err(|e| RequestError::RequestFailed(url, e))?;

    match response.status() {
        StatusCode::OK => Ok(response.text().unwrap()),
        other => Err(RequestError::BadResponse(other.as_u16())),
    }
}
