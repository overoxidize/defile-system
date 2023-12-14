#![allow(unused_imports)]
#![allow(dead_code)]

use crate::file_system::FileSystem;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use std::thread::scope;
use serde::{Serialize, Deserialize};
use crypto::digest::Digest;

type Tx = mpsc::UnboundedSender<String>;

struct Network<'a> {
    state: Arc<Mutex<NetworkState<'a>>>,
}

impl<'a> Network<'a> {
    pub fn new() -> Self {
        Self { state: Arc::new(Mutex::new(NetworkState::new())) }
    }

    pub fn broadcast(&mut self) -> Result<(), NetworkError> {
        let mut shared_state = self.state.clone();
        tokio::spawn(async move {
            let node_keys = shared_state.lock().await.nodes.keys();

            for node_addr in node_keys.into_iter() {
                
                let node_addr = *node_addr;
                let port = node_addr.port();

                // We want to connect to the address
                    // If the node is not connected to the network
                    // we probably want to remove them from the 
                    // nodes list
                // perform a state check to see if they're identical
                // if not, we want to update state
                // otherwise continue looping through
            
            }
        });

        Ok(())
    }
}

struct NetworkState<'a> {
    nodes: HashMap<SocketAddr, Tx>,
    file_system: FileSystem<'a>,
}

impl<'a> NetworkState<'a> {
    pub fn new() -> Self {
        Self { 
            nodes: HashMap::new(),
            file_system: FileSystem::new()
        }
    }
}

#[derive(Error, Debug)]
pub enum NetworkError {
    CorruptedState,
    NodeNotFound,
    SynchronizationError,
}