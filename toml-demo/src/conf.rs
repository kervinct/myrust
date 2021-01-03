pub(crate) mod basic_config;
pub(crate) mod error;
pub(crate) mod poem_config;

use super::*;
use super::environment::{Environment, Environment::*};

use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::fs::{self, File};
use std::io::Read;

pub use toml::value::{Array, Table, Value, Datetime};

const CONFIG_FILENAME: &str = "config/Poem.toml";
pub type Result<T> = ::std::result::Result<T, ConfigError>;

pub use self::error::ConfigError;
pub use self::poem_config::PoemConfig;
pub use self::basic_config::BasicConfig;