use crate::drocsid_client::client::Client;
use crossbeam::channel;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::sync::mpsc;

pub struct Server {
    //listener: TcpListener,
    server_sender: channel::Sender<String>, // mpmc sender
    server_receiver: mpsc::Receiver<String>, // mpsc receiver
    pub(crate) clients: Vec<Client>
}

impl Server {
    pub fn new(server_sender: channel::Sender<String>, server_receiver: mpsc::Receiver<String>) -> Self {
        Server {
            //listener,
            server_sender,
            server_receiver,
            clients: Vec::new(),
        }
    }

    pub fn new_client(&mut self, new: Client) -> bool {
        self.clients.push(new);
        true
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    headers: HashMap<String, String>,
    pub(crate) content: String
}