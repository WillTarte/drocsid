use crate::drocsid_client::client::Client;
use crossbeam::channel;
use serde::{Serialize, Deserialize};
use std::sync::mpsc;
use uuid::Uuid;

pub struct Server {
    server_sender: channel::Sender<String>, // mpmc sender
    pub(crate) server_receiver: mpsc::Receiver<Message>, // mpsc receiver
    pub(crate) clients: Vec<Client>
}

impl Server {
    pub fn new(server_sender: channel::Sender<String>, server_receiver: mpsc::Receiver<Message>) -> Self {
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum MessageType {
    ChatMessage,
    AuthenticateIdentity
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageEmitter {
    Client(Uuid),
    Server(Uuid)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub(crate) msg_type: MessageType,
    pub(crate) content: String,
    pub(crate) emitter: MessageEmitter
}