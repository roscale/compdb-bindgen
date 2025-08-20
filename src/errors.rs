use std::{io, string};

#[derive(Debug)]
pub enum Error {
    SerdeError(serde_json::Error),
    FileNotFound,
    NotEnoughArguments,
    GetCompilerIncludesError(GetCompilerIncludesError),
}

#[derive(Debug)]
pub enum GetCompilerIncludesError {
    IoError(io::Error),
    FromUtf8Error(string::FromUtf8Error),
    MatchNotFound(String),
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeError(error)
    }
}

impl From<GetCompilerIncludesError> for Error {
    fn from(error: GetCompilerIncludesError) -> Self {
        Self::GetCompilerIncludesError(error)
    }
}

impl From<string::FromUtf8Error> for GetCompilerIncludesError {
    fn from(error: string::FromUtf8Error) -> Self {
        Self::FromUtf8Error(error)
    }
}

impl From<io::Error> for GetCompilerIncludesError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}
