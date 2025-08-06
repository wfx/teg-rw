use ron::de::Error as RonError;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, DataError>;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("I/O error while reading {path:?}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("RON parse error in {path:?}: {source}")]
    ParseRon {
        path: PathBuf,
        #[source]
        source: RonError,
    },

    #[error("Asset not found: {0}")]
    AssetNotFound(PathBuf),

    #[error("Validation error: {0}")]
    Validation(String),
}

// Direkt Zugriff auf die Position im RON-Fehler:
impl DataError {
    /// Hilfsmethode, um direkt auf die Position zuzugreifen
    pub fn ron_error_position(&self) -> Option<(usize, usize)> {
        match self {
            DataError::ParseRon { source, .. } => Some((source.position.line, source.position.col)),
            _ => None,
        }
    }
}
