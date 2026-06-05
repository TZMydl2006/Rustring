use std::io;
use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, MiniZensicalError>;

#[derive(Debug, Error)]
pub enum MiniZensicalError {
    #[error("failed to {action} {path}: {source}")]
    Io {
        action: &'static str,
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to walk {path}: {source}")]
    WalkDir {
        path: PathBuf,
        #[source]
        source: walkdir::Error,
    },
    #[error("failed to parse config at {path}: {source}")]
    ParseConfig {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
    #[error("failed to parse front matter in {path}: {message}")]
    FrontMatter { path: PathBuf, message: String },
    #[error("failed to serialize search index at {path}: {source}")]
    SerializeSearch {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("failed to serialize knowledge graph at {path}: {source}")]
    SerializeGraph {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("invalid config: {0}")]
    InvalidConfig(String),
    #[error("template rendering failed: {0}")]
    Template(#[from] minijinja::Error),
}

impl MiniZensicalError {
    pub fn io(action: &'static str, path: impl Into<PathBuf>, source: io::Error) -> Self {
        Self::Io {
            action,
            path: path.into(),
            source,
        }
    }

    pub fn walk(path: impl Into<PathBuf>, source: walkdir::Error) -> Self {
        Self::WalkDir {
            path: path.into(),
            source,
        }
    }
}
