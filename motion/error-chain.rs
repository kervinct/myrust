#[macro_use]
extern crate error_chain;

use errors::*;
use std::{ fs::{ File, OpenOptions }, error::Error };

pub mod errors {
    error_chain! {
        errors {
            RateLimitExceeded {
                display("You have exceeded the allowed number of documents per minute.")
            }
        }
        foreign_links {
            Io(::std::io::Error);
        }
    }
}


const MAX_DOCS_CREATED_PER_MINUTE: u8 = 100;

fn num_docs_created_in_last_minute() -> u8 {
    2
}

pub fn create_document(filename: &str) -> Result<File> {
    if num_docs_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        bail!(ErrorKind::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)
        .chain_err(|| format!("could not open {}", filename))?;

    Ok(file)
}

fn main() {}