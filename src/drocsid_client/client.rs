use std::net::{TcpStream};
use crossbeam::channel;
use tokio::sync::mpsc;

pub struct Client{
    receiver: channel::Receiver<String>,
    sender: mpsc::Sender<String>,
    pub(crate) stream: TcpStream
}

impl Client {
    pub fn new(receiver: channel::Receiver<String>, sender: mpsc::Sender<String>, stream: TcpStream) -> Self {
        Client {
            receiver,
            sender,
            stream
        }
    }
}