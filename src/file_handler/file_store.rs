use super::error::Error;
use super::{cache::Cache, file_info::FileInfo};
use std::fs::File;
use std::io::{Write, Read};
use std::ops::Drop;

pub struct FileStore {
    cache: Cache,
}

impl FileStore {
    pub fn new() -> Self {
        return Self {
            cache: Cache::new(),
        }
    }

    pub fn edit_file(&mut self, file_info: FileInfo) -> Result<(), Error> {
        let info = file_info.info.clone();
        let file = self.cache.edit_elem(file_info);

        return Self::write_file(info, file);
    }

    pub fn read_file(&mut self, path: String) -> Result<FileInfo, Error> {
        if let Some(f) = self.cache.clone().read_elem(&path) {
            return Ok(f);
        } else if let Ok(mut ptr) = File::open(&path){
            let mut buff: Vec<u8> = Vec::new();
            ptr.read_to_end(&mut buff).expect("Corrupted");
            let file_info = FileInfo { path, info: buff };
            self.cache.add_to_cache(file_info.clone());
            return Ok(file_info)
        }
        return Err(Error::FileReadError(path));
    }

    fn write_file<'b>(info: Vec<u8>, file: Option<FileInfo>) -> Result<(), Error> {
        if let Some(f) = file {
            match File::create(&f.path) {
                Ok(mut ptr) => {
                    if let Err(_) = ptr.write(&info) {
                        return Err(Error::FileWriteError(f))
                    }
                },
                Err(_) => return Err(Error::FileWriteError(f)),
            }
        }

        return Ok(())
    }
}

impl Drop for FileStore {
    fn drop(&mut self) {
        while let Some(elem) = self.cache.queue.pop_back() {
            Self::write_file(elem.info.clone(), Some(elem)).expect("Could not write file");
        }
    }
}