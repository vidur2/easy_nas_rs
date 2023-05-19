use super::error::Error;
use super::file_info::DirInfo;
use super::{cache::Cache, file_info::FileInfo};
use std::fs::{File, FileType};
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

    pub fn write_dir(dir_info: DirInfo) -> Result<(), Error> {
        std::fs::create_dir(&dir_info.path);
        for file in dir_info.info {
            if let Err(err) = Self::write_file(file.info.clone(), Some(file)) {
                return Err(err)
            }
        }

        for dir in dir_info.sub_dirs {
            if let Err(err) = Self::write_dir(*dir) {
                return Err(err);
            }
        }
        return Ok(())
    }

    pub fn read_dir(&mut self, path: String) -> Result<DirInfo, Error> {
        match std::fs::read_dir(&path) {
            Ok(dir) => {
                let mut inf = DirInfo {
                    path: path.clone(),
                    info: Vec::new(),
                    sub_dirs: Vec::new(),
                };
                for elem in dir.into_iter() {
                    let elem = elem.unwrap();
                    let metadata = elem.metadata().unwrap();
                    if metadata.is_file() {
                        inf.info.push(self.read_file(path.clone()).unwrap());
                    } else {
                        inf.sub_dirs.push(Box::new(self.read_dir(String::from(elem.path().to_str().unwrap())).unwrap()));
                    }
                }
            },
            Err(_) => return Err(Error::DirReadError(path)),
        }

        return Err(Error::DirReadError(path));
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