use std::{collections:: VecDeque};

use super::file_info::{FileInfo};

#[derive(Clone)]
pub struct Cache {
    pub queue: VecDeque<FileInfo>,
    max_size: usize
}

impl Cache {
    pub fn new() -> Self {
        return Cache { queue: VecDeque::new(), max_size: 10}
    }

    pub fn edit_elem(&mut self, f: FileInfo) -> Option<FileInfo> {
        for (idx, file) in self.queue.iter().enumerate() {
            if f.path == file.path {
                self.queue.remove(idx);
                break;
            }
        }

        self.queue.push_front(f);

        if self.queue.len() > self.max_size {
            return self.queue.pop_back();
        }

        return None;
    }

    pub fn read_elem(&self, path: &str) -> Option<FileInfo> {
        for elem in self.queue.iter() {
            if elem.path == path {
                return Some(elem.clone());
            }
        }
        return None;
    }

    pub fn add_to_cache(&mut self, f: FileInfo) -> Option<FileInfo> {
        self.queue.push_front(f);
        if self.queue.len() > self.max_size {
            return self.queue.pop_back();
        }

        return None;
    }
}