use std::{
	error::Error,
	io,
	fmt,
	fs::{
		File,
		OpenOptions,
	},
};

use std::result;
type Result<T> = result::Result<T, DocumentServiceError>;

#[derive(Debug)]
pub enum DocumentServiceError {
	RateLimitExceeded,
	Io(io::Error),
}

// Error trait
impl Error for DocumentServiceError {
	// 可以省略
	fn description(&self) -> &str {
		use DocumentServiceError::*;
		match *self {
			RateLimitExceeded => "rate limit exceeded",
			Io(_) => "I/O error",
		}
	}
}

// Display for Error trait
impl fmt::Display for DocumentServiceError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use DocumentServiceError::*;
		match *self {
			RateLimitExceeded => write!(
				f,
				"You have exceeded the allowed number of documents per minute."
			),
			Io(ref io) => write!(f, "I/O error: {}", io),
		}
	}
}

// From for ? operator
impl From<io::Error> for DocumentServiceError {
	fn from(other: io::Error) -> Self {
		DocumentServiceError::Io(other)
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
		.open(filename)?;

	Ok(file)
}

fn main() {}