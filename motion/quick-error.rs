#[macro_use]
extern crate quick_error;

use quick_error::ResultExt;
use std::{ io, result, fs::{File, OpenOptions} };

type Result<T> = result::Result<T, DocumentServiceError>;

/// 自动实现Error trait
/// 简化Display、From实现
/// 方便添加上下文
quick_error! {
    #[derive(Debug)]
    pub enum DocumentServiceError {
        RateLimitExceeded {
            display("You have exceeded the allowed number of documents per minute.")
        }
        Io(filename: String, cause: io::Error) {
            display("I/O error: {} for filename {}", cause, filename)
            // from()
            context(filename: &'a str, cause: io::Error)
                -> (filename.to_string(), cause)
        }
    }
}

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 100;

fn num_docs_created_in_last_minute() -> u8 {
    2
}

pub fn create_document(filename: &str) -> Result<File> {
    if num_docs_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        return Err(DocumentServiceError::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)
        .context(filename)?;

    Ok(file)
}

fn main() {}