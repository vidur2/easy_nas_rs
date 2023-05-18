use super::file_info::FileInfo;

#[derive(Debug)]
pub enum Error {
    FileWriteError(FileInfo),
    FileReadError(String)
}