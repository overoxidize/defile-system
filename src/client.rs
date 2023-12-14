#![allow(unused_imports)]
#![allow(dead_code)]

use crate::{
    file::{File, FileError, NetworkFile},
    file_system::FileSystem,
};
use file_format::FileFormat;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::{fs, fs::File as FsFile};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc,
};
use tokio_util::codec::{Framed, LinesCodec};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
type Rx = mpsc::UnboundedReceiver<String>;
struct Client<'a> {
    username: String,
    file_system: FileSystem<'a>,
    rx: Rx,
    socket: SocketAddr,
    lines: Framed<TcpStream, LinesCodec>,
    state_checksum: 
}

impl<'a> Client<'a> {
    pub fn upload_file<T>(
        &mut self,
        file_name: &'a String,
        directory: Option<T>,
    ) -> Result<(), FileError> {
        let mut target_dir: PathBuf = PathBuf::new();
        let mut network_file: NetworkFile;
        if directory.is_none() {
            target_dir = PathBuf::from("/");
            Ok(())
        } else {
            let local_file = FsFile::open(file_name).expect("Unable to open file.");
            let metadata = fs::metadata(file_name).expect("Unable to retrieve file metadata.");
            let mut file_buffer =
                fs::read(file_name).expect("Unable to read data into file buffer.");
            let file_extension = Path::new(file_name)
                .extension()
                .expect("Unable to retrieve file extension.");

            let format =
                FileFormat::from_file(file_name).expect("Unable to retrieve file format.");
            let mut temp_file = File::new(
                file_name,
                format,
                file_extension.into(),
                self.username.clone(),
            );

            temp_file.buffer = file_buffer;

            if let Ok(time) = metadata.created() {
                temp_file.created_at = time;
            } else {
                let message = String::from("Couldn't retrieve creation time for file.");
                return Err(FileError::MetadataError { metadata: message });
            }

            temp_file.set_checksum();

            network_file = NetworkFile { inner: temp_file };

            self.file_system.directory.insert(file_name, network_file);

            Ok(())
        }
    }

    fn generate_state_checksum(&mut self, data: &BtreeMap<String, NetworkFile>) -> Result<(), std::io::Error> {
        
        let serialized = serde_json::to_string(&data).expect("Serialization failed.");
        let hasher = DefaultHasher::new();
        hasher.input_str(&serialized_data);
        let hash_result = hasher.result_str();

        hash_result
        // hasher.write(self.file_system.directory.as_bytes());

        Ok(())
    }
}
