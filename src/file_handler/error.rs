use serde::Serialize;

use super::file_info::FileInfo;

#[derive(Debug, Serialize)]
pub enum Error {
    FileWriteError(FileInfo),
    FileReadError(String),
    DirReadError(String)
}