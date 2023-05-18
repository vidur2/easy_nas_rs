#[derive(Clone, Debug)]
pub struct FileInfo {
    pub path: String,
    pub info: Vec<u8>,
}