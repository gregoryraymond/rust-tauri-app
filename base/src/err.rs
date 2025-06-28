use thiserror::Error;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum InvokeError {
    #[error("Permission denied: {0}")]
    PermDenied(String),
    #[error("{0}")]
    Anyhow(String) 
}