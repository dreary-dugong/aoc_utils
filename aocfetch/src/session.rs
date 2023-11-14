use std::fs;
use std::io;
use std::path::PathBuf;

extern crate thiserror;
use thiserror::Error;

extern crate ini;
use ini::Ini;

extern crate rusqlite;
use rusqlite::Connection;

/// error encountered while retreiving session cookie
#[derive(Error, Debug)]
pub enum SessionError {
    #[error("unable to read session cookie from {0}: {1}")]
    FileReadError(PathBuf, io::Error),
    #[error("unable to load Firefox's profile.ini file: {0}")]
    IniLoadError(ini::Error),
    #[error("unable to find the correct profile in Firefox's profile.ini file")]
    IniMissingProfile,
    #[error("profile in Firefox's profile.ini was found but is missing the Path attribute")]
    ProfileMissingPath,
    #[error("unable to open cookies database: {0}")]
    CantOpenDb(rusqlite::Error),
    #[error("error preparing statement for cookies database: {0}")]
    StatementPrepError(rusqlite::Error),
    #[error("error executing query on cookies database: {0}")]
    QueryError(rusqlite::Error),
    #[error("error getting next row of query results from cookies database: {0}")]
    RowsError(rusqlite::Error),
    #[error("can't find adventofcode.com session cookie in firefox profile's cookie database")]
    MissingCookie,
}

/// pull session cookie from file containing only that
pub fn from_file(file: PathBuf) -> Result<String, SessionError> {
    Ok(fs::read_to_string(&file)
        .map_err(|e| SessionError::FileReadError(file, e))?
        .trim()
        .to_string())
}

/// pull session cookie from user's firefox profile
pub fn from_firefox(folder: PathBuf) -> Result<String, SessionError> {
    // get path to profile of interest
    let profile_path = get_profile_path(folder)?;
    // construct the path to the cookies database
    let mut cookie_db_path = profile_path;
    cookie_db_path.push("cookies.sqlite");

    extract_cookie(cookie_db_path)
}

/// given the location of the firefox dotfiles, get the full path to the profile from which we'll extract the cookie
fn get_profile_path(firefox_path: PathBuf) -> Result<PathBuf, SessionError> {
    const PROFILE_NAME: &str = "default-release"; // this probably shouldn't be a constant but I'm not sure how firefox determines which profile is in use. Maybe this can be an arg.

    // parse profiles.ini so we can find the folder name of the profile we want
    let mut profiles_ini_path = firefox_path.clone();
    profiles_ini_path.push("profiles.ini");
    let profile_data =
        Ini::load_from_file(profiles_ini_path).map_err(SessionError::IniLoadError)?;

    // iterate over every section and check if it has a name and if so whether that's the name of the profile we want and if so take the path
    let mut profile_folder = None;
    for section in profile_data.sections() {
        if let Some(name) = profile_data.get_from(section, "Name") {
            if name == PROFILE_NAME {
                profile_folder = Some(
                    profile_data
                        .get_from(section, "Path")
                        .ok_or(SessionError::ProfileMissingPath)?,
                );
            }
        }
    }

    let profile_folder = profile_folder.ok_or(SessionError::IniMissingProfile)?;

    let mut profile_path = firefox_path.clone();
    profile_path.push(profile_folder);
    Ok(profile_path)
}

/// given the path to the cookies database, extract the session cookie if it exists
fn extract_cookie(dbpath: PathBuf) -> Result<String, SessionError> {
    const QUERY: &str =
        "SELECT value FROM moz_cookies WHERE host LIKE '%.adventofcode.com' AND name='session' LIMIT 1;";
    let con = Connection::open(dbpath).map_err(SessionError::CantOpenDb)?;

    let mut stmt = con
        .prepare(QUERY)
        .map_err(SessionError::StatementPrepError)?;

    let cookie: String = stmt
        .query([])
        .map_err(SessionError::QueryError)?
        .next()
        .map_err(SessionError::RowsError)?
        .ok_or(SessionError::MissingCookie)?
        .get_unwrap(0);

    Ok(cookie)
}
