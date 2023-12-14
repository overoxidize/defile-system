#![allow(unused_imports)]
#![allow(dead_code)]

use crate::connection::Connection;
use crate::file_system::FileSystem;
use std::net::SocketAddr;
use tokio::sync::mpsc;

type Rx = mpsc::UnboundedReceiver<String>;
struct Node<'a> {
    file_system: FileSystem<'a>,
    connections: Vec<Connection>,
    connected: bool,
    socket: SocketAddr,
    rx: Rx,
}
