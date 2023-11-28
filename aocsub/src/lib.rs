use std::io::{self, Write};
use std::path::PathBuf;

extern crate clap;
use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};

extern crate chrono;
use chrono::{DateTime, Datelike, FixedOffset, Utc};

extern crate thiserror;
use thiserror::Error;

extern crate dirs;

extern crate aocfetch;
use aocfetch::session::{self, SessionError};

mod request;
use request::RequestError;

#[derive(Parser)]
#[command(name = "aocsub")]
#[command(author = "Daniel Gysi <danielgysi@protonmail.com>")]
#[command(version = "0.2.0")]
#[command(
    about = "A command line utility to submit answers for Advent of Code <https://adventofcode.com> puzzles"
)]
struct Args {
    /// the answer to submit (defaults to stdin)
    #[arg(short, long)]
    answer: Option<String>,

    /// your adventofcode.com session cookie
    #[arg(group = "session", short, long)]
    cookie: Option<String>,
    /// a file containing your adventofcode.com session cookie
    #[arg(group = "session", short, long)]
    file: Option<PathBuf>,
    /// the location of your firefox dotfiles (defaults to ~/.mozilla/firefox)
    // because of the mutual exclusivity with the other session args, we'll handle the default in Config::make
    #[arg(group = "session", short, long)]
    browser_folder: Option<PathBuf>,

    /// the day to submit the answer for
    /// (defaults to current day if UTC-5 is December, otherwise 1)
    #[arg(value_parser = clap::value_parser!(u8).range(1..=31), short, long)]
    day: Option<u8>,
    /// the year to submit the answer for
    /// (defaults to current year if UTC-5 is December, otherwise the previous year)
    /// NOTE: this will break in the year 65,536. File a github issue if you encounter this.
    #[arg(value_parser = clap::value_parser!(u16).range(2015..), short, long)]
    year: Option<u16>, // we'll validate this as a year that isn't in the future in the make function

    /// the level to submit the answer for (1 or 2, defaults to 1)
    #[arg(default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=2), short, long)]
    level: u8,
}

/// configuration options for the app created based on cli args
pub struct Config {
    session_cfg: SessionConfig,
    day: u8,
    year: u16,
    level: u8,
    answer: String,
}

/// keep track of how the application will get the session cookie, inferred from the cli args
enum SessionConfig {
    Direct(String),
    File(PathBuf),
    Firefox(PathBuf),
}

/// construct app config from arguments
impl Config {
    pub fn make() -> Self {
        let args = Args::parse();

        // parse and store the answer
        let answer = if let Some(ans) = args.answer {
            ans
        } else {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap_or_else(|_| {
                let mut cmd = Args::command();
                cmd.error(
                    ErrorKind::InvalidValue,
                    "no answer provided and it could not be parsed from stdin",
                )
                .exit();
            });
            buf
        };

        // how will we get the session cookie?
        let session_cfg = if let Some(session_string) = args.cookie {
            // the user passed it directly
            SessionConfig::Direct(session_string.clone())
        } else if let Some(session_file) = args.file {
            // the user stored it in a file
            SessionConfig::File(session_file.clone())
        } else if let Some(firefox_folder) = args.browser_folder {
            // the user wants to grab it from firefox and provided the config folder
            SessionConfig::Firefox(firefox_folder.clone())
        } else {
            // we default to grabbing it from where we assume the firefox config folder is
            let mut firefox_folder = dirs::home_dir().unwrap();
            firefox_folder.push(".mozilla/firefox");
            SessionConfig::Firefox(firefox_folder)
        };

        // time sensitive config
        let dt = get_aoc_time();

        // figure out the year
        let year = if let Some(arg_year) = args.year {
            if arg_year <= dt.year() as u16 {
                arg_year
            // custom clap validation for a user-provided invalid year
            } else {
                let mut cmd = Args::command();
                cmd.error(
                    ErrorKind::InvalidValue,
                    "the year provided is in the future for UTC-5",
                )
                .exit();
            }
        } else if dt.month() == 12 {
            dt.year() as u16
        } else {
            dt.year() as u16 - 1
        };

        // figure out the day
        let day = if let Some(arg_day) = args.day {
            // custom clap validation for a user-provided invalid year
            if year != dt.year() as u16 || (dt.month() == 12 && arg_day <= dt.day() as u8) {
                arg_day
            } else {
                let mut cmd = Args::command();
                cmd.error(
                    ErrorKind::InvalidValue,
                    "the day provided is in the future for UTC-5",
                )
                .exit()
            }
        } else if dt.month() == 12 {
            dt.day() as u8
        } else {
            1
        };

        let level = args.level;

        Config {
            session_cfg,
            day,
            year,
            level,
            answer,
        }
    }
}

/// return a DateTime struct representing the current time for AOC
pub fn get_aoc_time() -> DateTime<Utc> {
    // seconds in an hour
    const HOUR: i32 = 3600;

    // aoc time is UTC-5
    let utc_now = Utc::now();
    let offset = FixedOffset::east_opt(-5 * HOUR).unwrap();

    utc_now + offset
}

/// an error encountered while running the application
#[derive(Error, Debug)]
pub enum RunError {
    #[error("error retrieving session cookie: {0}")]
    SessionError(#[from] SessionError),
    #[error("error occurred while posting answer to adventofcode.com: {0}")]
    RequestError(#[from] RequestError),
    #[error("error occured while attempting to write to stdout: {0}")]
    StdoutError(io::Error),
}

/// run the application according to the provided config
pub fn run(cfg: Config) -> Result<(), RunError> {
    // figure out the session cookie
    let session_cookie = match cfg.session_cfg {
        SessionConfig::Direct(session_string) => session_string,
        SessionConfig::File(file) => session::from_file(file)?,
        SessionConfig::Firefox(folder) => session::from_firefox(folder)?,
    };

    let mut recv =
        request::post_answer(cfg.year, cfg.day, cfg.level, &cfg.answer, &session_cookie)?;
    recv.push('\n');

    io::stdout()
        .write_all(recv.as_bytes())
        .map_err(RunError::StdoutError)?;

    Ok(())
}
