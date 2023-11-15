use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

extern crate clap;
use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};

extern crate chrono;
use chrono::{DateTime, Datelike, FixedOffset, Utc};

extern crate thiserror;
use thiserror::Error;

extern crate reqwest;
use reqwest::blocking;
use reqwest::StatusCode;

extern crate regex;
use regex::Regex;

#[derive(Parser)]
struct Args {
    /// optional file to output to (defaults to stdout)
    #[arg(short, long)]
    outfile: Option<PathBuf>,

    /// day to download the example for (defaults to current day if it's December in UTC-5, otherwise 1)
    #[arg(value_parser = clap::value_parser!(u8).range(1..=31),short, long)]
    day: Option<u8>,

    /// year to download the example for (defaults to current year if it's Decembe in UTC-5, otherwise last year)
    #[arg(value_parser = clap::value_parser!(u16).range(2015..), short, long)]
    year: Option<u16>,
}

/// output configration options
enum OutputCfg {
    File(PathBuf),
    Stdout,
}
/// configuration settings for the application
pub struct Config {
    out: OutputCfg,
    day: u8,
    year: u16,
}

impl Config {
    /// construct the application configuration based on cli args
    pub fn make() -> Self {
        let args = Args::parse();

        // output setting
        let out = match args.outfile {
            Some(f) => OutputCfg::File(f),
            None => OutputCfg::Stdout,
        };

        // time sensitive config
        let aoc_dt = get_aoc_dt();

        // year
        let year = if let Some(arg_year) = args.year {
            if arg_year < aoc_dt.year() as u16 {
                arg_year
            } else {
                // validate inputted year to make sure it's not in the future
                let mut cmd = Args::command();
                cmd.error(
                    ErrorKind::InvalidValue,
                    "the year provided is in the future for UTC-5",
                )
                .exit()
            }
        // if there is no year arg, choose a default
        } else if aoc_dt.month() == 12 {
            aoc_dt.year() as u16
        } else {
            aoc_dt.year() as u16 - 1
        };

        // day
        let day = if let Some(arg_day) = args.day {
            // validate inputted day to make sure it's not in the future
            if year != aoc_dt.year() as u16 || arg_day <= aoc_dt.day() as u8 {
                arg_day
            } else {
                let mut cmd = Args::command();
                cmd.error(
                    ErrorKind::InvalidValue,
                    "the and year provided is in the future for UTC-5",
                )
                .exit()
            }
        // if there is no day arg, choose a default
        } else if aoc_dt.month() == 12 {
            aoc_dt.day() as u8
        } else {
            1
        };

        Config { out, day, year }
    }
}

/// return the current datetime in UTC-5 timezone (AOC timezone)
fn get_aoc_dt() -> DateTime<Utc> {
    const HOUR: i32 = 3600;
    let offset = FixedOffset::east_opt(-5 * HOUR).unwrap();
    let utc_now = Utc::now();

    utc_now + offset
}

#[derive(Error, Debug)]
pub enum RunError {
    #[error("request failed: {0}")]
    RequestFailed(reqwest::Error),
    #[error("bad response from AOC: code {0}")]
    BadRequest(u16),
    #[error("failed to find example on page")]
    RegexFailed,
    #[error("failed to write example to {0}: {1}")]
    FileWriteFailed(PathBuf, io::Error),
    #[error("failed to write example to stdout: {0}")]
    StdoutWriteFailed(io::Error),
}
pub fn run(cfg: Config) -> Result<(), RunError> {
    let html = get_html(cfg.year, cfg.day)?;
    let example = retrieve_example(html)?;

    match cfg.out {
        OutputCfg::File(f) => {
            fs::write(&f, example).map_err(|e| RunError::FileWriteFailed(f, e))?;
        }
        OutputCfg::Stdout => {
            io::stdout()
                .write_all(example.as_bytes())
                .map_err(RunError::StdoutWriteFailed)?;
        }
    };

    Ok(())
}

fn get_html(year: u16, day: u8) -> Result<String, RunError> {
    let url = format!("https://adventofcode.com/{year}/day/{day}");
    let response = blocking::get(url).map_err(RunError::RequestFailed)?;

    match response.status() {
        StatusCode::OK => Ok(response.text().unwrap()),
        other => Err(RunError::BadRequest(other.as_u16())),
    }
}

fn retrieve_example(html: String) -> Result<String, RunError> {
    const PATTERN: &str = r"<code>.*</code>";
    let reg = Regex::new(PATTERN).unwrap();
    if let Some(m) = reg.find(&html) {
        Ok(m.as_str()
            .strip_prefix("<code>")
            .unwrap()
            .strip_suffix("</code>")
            .unwrap()
            .to_string())
    } else {
        Err(RunError::RegexFailed)
    }
}
