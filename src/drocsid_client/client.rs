use std::sync::mpsc::{Receiver, Sender};
use std::net::TcpStream;

pub struct Client {
    receiver: Receiver<String>,
    sender: Sender<String>,
    stream: TcpStream
}

impl Client {
    pub fn new(receiver: Receiver<String>, sender: Sender<String>, stream: TcpStream) -> Self {
        Client {
            receiver,
            sender,
            stream
        }
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        unimplemented!()
    }

    fn clone_from(&mut self, source: &Self) {
        unimplemented!()
    }
}