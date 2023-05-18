use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileInfo {
    pub path: String,
    pub info: Vec<u8>,
}