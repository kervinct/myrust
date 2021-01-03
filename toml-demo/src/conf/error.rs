use super::*;
use std::error::Error;

use self::ConfigError::*;

#[derive(Debug)]
pub enum ConfigError {
    NotFound,
    IoError,
    BadFilePath(PathBuf, &'static str),
    BadEnv(String),
    BadEntry(String, PathBuf),
    BadType(String, &'static str, &'static str, Option<PathBuf>),
    ParseError(String, PathBuf, String, Option<(usize, usize)>),
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            NotFound => "Config file was not found",
            IoError => "There was an I/O error while reading the config file",
            BadFilePath(..) => "The config file path is invalid",
            BadEnv(..) => "The environment specified in `POEM_ENV` is invalid",
            BadEntry(..) => "An environment specified as `[environment]` is invalid",
            BadType(..) => "A key was specified with a value of the wrong type",
            ParseError(..) => "The config file contains invalid TOML",
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NotFound => write!(f, "Config file was not found"),
            IoError => write!(f, "There was an I/O error while reading the config file"),
            BadFilePath(ref p, _) => write!(
                f, "{:?} is not a valid config path", p),
            BadEnv(ref e) => write!(
                f, "{:?} is not a valid `POEM_ENV` value", e),
            BadEntry(ref e, _) => write!(
                f, "{:?} is not a valid `[environment]` entry", e),
            BadType(ref n, e, a, _) => write!(
                f, "type mismatch for '{}'. expected {}, found {}", n, e, a),
            ParseError(..) => write!(f, "The config file contains invalid TOML"),
        }
    }
}