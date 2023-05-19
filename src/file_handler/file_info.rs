use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileInfo {
    pub path: String,
    pub info: Vec<u8>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DirInfo {
    pub path: String,
    pub info: Vec<FileInfo>,
    pub sub_dirs: Vec<Box<DirInfo>>
}