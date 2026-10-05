use thiserror::Error;

//different error types;
#[derive(Error, Debug)]
pub enum GameEngineError {
    // #[error("Config not found: {0}")]
    // ConfigNotFound(String),

    // #[error("Faction not found: {0}")]
    // FactionNotFound(String),

    // #[error("Game Instance not found: {0}")]
    // GameInstanceNotFound(String),

    // #[error("Unit type not found at index: {0}")]
    // UnitTypeNotFound(usize),

    // #[error("Failed to parse toml: {0}")]
    // TomlParse(#[from] toml::de::Error),

    // #[error("IO error: {0}")]
    // Io(#[from] std::io::Error),

    // #[error("Calculation error: {0}")]
    // Calculation(String),

    // #[error("Missing parameter: {0}")]
    // MissingParameter(String),
}
