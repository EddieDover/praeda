use thiserror::Error;

#[derive(Error, Debug)]
pub enum PraedaError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("TOML serialization error: {0}")]
    TomlError(#[from] toml::ser::Error),

    #[error("TOML deserialization error: {0}")]
    TomlDeError(#[from] toml::de::Error),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Missing quality: {0}")]
    MissingQuality(String),

    #[error("Missing item type: {0}")]
    MissingItemType(String),

    #[error("Missing item subtype: type={0}, subtype={1}")]
    MissingItemSubtype(String, String),
}

pub type Result<T> = std::result::Result<T, PraedaError>;
