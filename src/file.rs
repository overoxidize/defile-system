#![allow(unused_imports)]
#![allow(dead_code)]

use file_format::FileFormat;
use std::fmt;
use std::hash::Hasher;
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    ffi::OsString,
    path::PathBuf,
    time::SystemTime,
};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct File<'a> {
    pub name: &'a String,
    pub size: usize,
    pub buffer: Vec<u8>,
    file_format: FileFormat,
    file_extension: OsString,
    pub created_at: SystemTime,
    // replace this with DateTime or something similar
    pub modifications: Vec<SystemTime>,
    // replace with a collection to store modifications
    // over time
    owner: String,
    pub permissions: Vec<FilePermission>,
    pub checksum: Option<Vec<u8>>,
    pub directory: PathBuf,
}

impl<'a> File<'a> {
    pub fn new(
        name: &'a String,
        file_format: FileFormat,
        file_extension: OsString,
        owner: String,
    ) -> Self {
        Self {
            name,
            size: 0,
            buffer: Vec::new(),
            file_format,
            file_extension,
            created_at: SystemTime::now(),
            modifications: Vec::new(),
            owner,
            permissions: vec![FilePermission::new()],
            checksum: Some(Vec::new()),
            directory: PathBuf::from("/"),
        }
    }

    pub fn set_checksum(&mut self) {
        let checksum = calculate_checksum(&self.buffer);

        self.checksum = Some(checksum);
    }

    pub fn get_checksum(&self) -> &Option<Vec<u8>> {
        &self.checksum
    }

    fn read_into_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, FileError> {
        for (i, ele) in self.buffer.iter().enumerate() {
            buffer[i] = *ele;
        }

        Ok(buffer.len())
    }

    //  fn write_to_file(&self, &mut buffer: &[u8])
}
#[derive(Debug, Clone, Copy)]
pub enum Permission {
    Read,
    Write,
    Execute,
    Delete,
}

#[derive(Debug, Clone)]
pub struct FilePermission {
    permissions: HashMap<String, Permission>,
}

impl Default for FilePermission {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for FilePermission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FilePermission {
    pub fn new() -> Self {
        Self {
            permissions: HashMap::new(),
        }
    }

    pub fn add_permission(&mut self, permission: Permission, permission_type: &str) {
        self.permissions.insert(permission_type.into(), permission);
    }

    pub fn has_permission(&self, permission_type: &str) -> Option<Permission> {
        self.permissions.get(permission_type).cloned()
    }
}

impl<'a> fmt::Display for File<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({},{},{:#?},{},{:?},{:?}, {:#?}, {},{:#?}, {:#?})",
            self.name,
            self.size,
            self.buffer,
            self.file_format,
            self.file_extension,
            self.created_at,
            self.modifications,
            self.owner,
            self.permissions,
            self.checksum
        )
    }
}

pub struct LocalFile<'a> {
    inner: File<'a>,
}

// impl From<LocalFile> for NetworkFile {}

pub struct NetworkFile<'a> {
    pub inner: File<'a>,
}

#[derive(Error, Debug)]
pub enum FileError {
    #[error("File not found.")]
    FileNotFound { file_name: String },
    #[error("Unable to retrieve metadata.")]
    MetadataError { metadata: String },
}

fn calculate_checksum(data: &[u8]) -> Vec<u8> {
    let mut hasher = DefaultHasher::new();
    hasher.write(data);

    hasher.finish().to_be_bytes().to_vec()

}
