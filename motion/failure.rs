extern crate failure;
#[macro_use]
extern crate failure_derive;

use failure::{Backtrace, Fail};
use std::{io, result};
use std::fs::{ File, OpenOptions };

type Result<T> = result::Result<T, DocumentServiceError>;

#[derive(Debug, Fail)]
pub enum DocumentServiceError {
    #[fail(display = "You have exceeded the allowed number of documents per minute.")]
    RateLimitExceeded(Backtrace),
    #[fail(display = "I/O error: {}", _0)]
    Io(io::Error, Backtrace),
}

// From for ? operator
impl From<io::Error> for DocumentServiceError {
    fn from(other: io::Error) -> Self {
        DocumentServiceError::Io(other, Backtrace::new())
    }
}

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 100;

fn num_docs_created_in_last_minute() -> u8 {
    2
}

pub fn create_document(filename: &str) -> Result<File> {
    if num_docs_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        return Err(DocumentServiceError::RateLimitExceeded(Backtrace::new()));
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;

    Ok(file)
}

fn main() {}