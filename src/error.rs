use std::{error, fmt::Display, io};


#[derive(Debug)]
pub enum PaddleOcrError {
    Io(io::Error),
    Ort(ort::Error),
    Custom(String),
}

impl From<io::Error> for PaddleOcrError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ort::Error> for PaddleOcrError {
    fn from(value: ort::Error) -> Self {
        Self::Ort(value)
    }
}

impl Display for PaddleOcrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Io(e) => e.to_string(),
                Self::Ort(e) => e.to_string(),
                Self::Custom(e) => e.to_string(),
            }
        )
    }
}

impl error::Error for PaddleOcrError  {}

pub type PaddleOcrResult<T> = Result<T, PaddleOcrError>;
