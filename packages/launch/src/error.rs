use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Missing file/dir at path: {0}")]
	MissingFile(String),
	#[error("Io error: {0:?}")]
	Io(#[from] io::Error),
}
