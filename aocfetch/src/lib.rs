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

mod session;
use session::SessionError;

mod request;
use request::RequestError;

/// app cli arg config
#[derive(Parser)]
struct Args {
    /// the session cookie directly
    #[arg(group = "session", short, long)]
    session_cookie: Option<String>,
    /// a file containing the session cookie
    #[arg(group = "session", short, long)]
    session_file: Option<PathBuf>,
    /// the location of the firefox dotfiles (defaults to ~/.mozilla/firefox)
    // because of the mutual exclusivity with the other session args, we'll handle the default in Config::make
    #[arg(group = "session", short, long)]
    firefox_folder: Option<PathBuf>,

    /// the day to get the input for
    /// if this isn't given, the default is the current day if UTC-5 (AOC timezone) is in december otherwise december 1st
    #[arg(value_parser = clap::value_parser!(u8).range(1..=31), short, long)]
    day: Option<u8>,
    /// the year to get the input for
    /// if this isn't given, the default is the current year if UTC-5 (AOC timezone) is in December otherwise the previous year
    /// if you're using this in the year 65,536 or later, this will fail. Send me an email and I'll fix it right away.
    #[arg(value_parser = clap::value_parser!(u16).range(2015..), short, long)]
    year: Option<u16>, // we'll validate this as a year that isn't in the future in the make function

    /// file to save the input.txt data to
    /// if no output file is provided, the data will be piped to stdout
    #[arg(short, long)]
    output: Option<PathBuf>,
}

/// configuration options for the app created based on cli args
pub struct Config {
    session_cfg: SessionConfig,
    output_cfg: OutputConfig,
    day: u8,
    year: u16,
}

/// keep track of how the application will get the session cookie, inferred from the cli args
enum SessionConfig {
    Direct(String),
    File(PathBuf),
    Firefox(PathBuf),
}

/// keep track of how the application will output the data received
enum OutputConfig {
    File(PathBuf),
    Stdout,
}

/// construct app config from arguments
impl Config {
    pub fn make() -> Self {
        let args = Args::parse();

        // how will we get the session cookie?
        let session_cfg = if let Some(session_string) = args.session_cookie {
            // the user passed it directly
            SessionConfig::Direct(session_string.clone())
        } else if let Some(session_file) = args.session_file {
            // the user stored it in a file
            SessionConfig::File(session_file.clone())
        } else if let Some(firefox_folder) = args.firefox_folder {
            // the user wants to grab it from firefox and provided the config folder
            SessionConfig::Firefox(firefox_folder.clone())
        } else {
            // we default to grabbing it from where we assume the firefox config folder is
            SessionConfig::Firefox(PathBuf::from("~/.mozilla/firefox"))
        };

        // where will we store the output of the request if we get a 200 response
        let output_cfg = if let Some(out_file) = args.output {
            OutputConfig::File(out_file)
        } else {
            OutputConfig::Stdout
        };

        // time sensitive config
        let dt = get_aoc_time();

        // figure out the day
        let day = if let Some(arg_day) = args.day {
            arg_day
        } else if dt.month() == 12 {
            dt.day() as u8
        } else {
            1
        };

        // figure out the year
        let year = if let Some(arg_year) = args.year {
            if arg_year <= dt.year() as u16 {
                arg_year
            // custom clap validation for a user-provided invalid year
            } else {
                let mut cmd = Args::command();
                cmd.error(
                    ErrorKind::ArgumentConflict,
                    "The year provided is in the future for UTC-5",
                )
                .exit();
            }
        } else {
            dt.year() as u16
        };

        Config {
            session_cfg,
            output_cfg,
            day,
            year,
        }
    }
}

/// return a DateTime struct representing the current time for AOC
fn get_aoc_time() -> DateTime<Utc> {
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
    #[error("error retrieving session cookie")]
    SessionError(#[from] SessionError),
    #[error("error occurred while requesting input from adventofcode.com")]
    RequestError(#[from] RequestError),
    #[error("error occured while attempting to write to stdout")]
    StdoutError(io::Error),
    #[error("error occured while attempting to create file {0}")]
    FileCreationError(PathBuf, io::Error),
    #[error("error occured while attempting to write to file {0}")]
    FileWriteError(PathBuf, io::Error),
}

/// run the application according to the provided config
pub fn run(cfg: Config) -> Result<(), RunError> {
    // figure out the session cookie
    let session_cookie = match cfg.session_cfg {
        SessionConfig::Direct(session_string) => session_string,
        SessionConfig::File(file) => session::from_file(file)?,
        SessionConfig::Firefox(folder) => session::from_firefox(folder)?,
    };

    let recv = request::request_input(cfg.year, cfg.day, &session_cookie)?;

    // write to output as determined by the config
    match cfg.output_cfg {
        OutputConfig::Stdout => {
            io::stdout()
                .write_all(recv.as_bytes())
                .map_err(RunError::StdoutError)?;
        }
        OutputConfig::File(file) => {
            let mut out =
                File::create(&file).map_err(|e| RunError::FileCreationError(file.clone(), e))?;
            out.write_all(recv.as_bytes())
                .map_err(|e| RunError::FileWriteError(file.clone(), e))?;
        }
    }

    Ok(())
}
