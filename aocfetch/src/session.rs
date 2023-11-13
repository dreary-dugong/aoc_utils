use std::fs;
use std::io;
use std::path::PathBuf;

extern crate thiserror;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("unable to read session cookie from file {0}")]
    FileReadError(PathBuf, io::Error),
}

pub fn from_file(file: PathBuf) -> Result<String, SessionError> {
    Ok(fs::read_to_string(file).map_err(|e| SessionError::FileReadError(file, e))?)
}
