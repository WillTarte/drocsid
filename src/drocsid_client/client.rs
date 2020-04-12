use crossbeam::channel;
use std::net::TcpStream;
use std::sync::mpsc;

pub struct Client{
    receiver: channel::Receiver<String>,
    pub(crate) sender: mpsc::Sender<String>,
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

impl Clone for Client {
    fn clone(&self) -> Self {
        Client {
            receiver: self.receiver.clone(),
            sender: self.sender.clone(),
            stream: self.stream.try_clone().unwrap()
        }
    }
}