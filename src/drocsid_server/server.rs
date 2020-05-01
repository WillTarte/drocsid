use crate::drocsid_client::client::Client;
use crossbeam::channel;
use serde::{Serialize, Deserialize};
use std::sync::mpsc;

pub struct Server<'a> {
    //listener: TcpListener,
    server_sender: channel::Sender<String>, // mpmc sender
    pub(crate) server_receiver: mpsc::Receiver<Message>, // mpsc receiver
    pub(crate) clients: Vec<Client<'a>>
}

impl Server<'_> {
    pub fn new(server_sender: channel::Sender<String>, server_receiver: mpsc::Receiver<Message>) -> Self {
        Server {
            //listener,
            server_sender,
            server_receiver,
            clients: Vec::new(),
        }
    }

    pub fn new_client(&mut self, new: Client<'_>) -> bool {
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
    Client(String),
    Server(String)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub(crate) msg_type: MessageType,
    pub(crate) content: String,
    pub(crate) emitter: MessageEmitter
}