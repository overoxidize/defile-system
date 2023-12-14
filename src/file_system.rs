use crate::file::NetworkFile;
use std::collections::BTreeMap;
pub struct FileSystem<'a> {
    pub directory: BTreeMap<&'a String, NetworkFile<'a>>,
}

impl<'a> FileSystem<'a> {
    pub fn new() -> Self {
        Self {
            directory: BTreeMap::new()
        }
    }

    // pub fn synchronize
}