use std::net::TcpListener;
use crossbeam::Sender;
use std::sync::mpsc::Receiver;
use crate::drocsid_client::client::Client;

pub struct Server {
    listener: TcpListener,
    client_sender: Sender<String>,
    receiver: Receiver<String>,
    clients: Vec<Client>
}

impl Server {
    pub fn new(listener: TcpListener, client_sender: Sender<String>, receiver: Receiver<String>, clients: Vec<Client>) -> Self {
        Server {
            listener,
            client_sender,
            receiver,
            clients
        }
    }
}