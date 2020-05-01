use crossbeam::channel;
use std::net::TcpStream;
use std::sync::mpsc;
use crate::drocsid_server::server::Message;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

pub struct Client<'a>{
    receiver: channel::Receiver<String>,
    pub(crate) sender: mpsc::Sender<Message>,
    pub(crate) stream: TcpStream,
    pub(crate) identity: ClientIdentity<'a>
}

impl Client<'_> {
    pub fn new(receiver: channel::Receiver<String>, sender: mpsc::Sender<Message>, stream: TcpStream) -> Self {
        Client {
            receiver,
            sender,
            stream,
            identity: ClientIdentity::new_anonymous()
        }
    }

    pub fn set_identity(&mut self, auth_msg: Message) {
        self.identity = ClientIdentity::from_string(auth_msg.content);
    }
}

impl Clone for Client<'_> {
    fn clone(&self) -> Self {
        Client {
            receiver: self.receiver.clone(),
            sender: self.sender.clone(),
            stream: self.stream.try_clone().unwrap(),
            identity: self.identity
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct ClientIdentity<'a> {
    username: Uuid,
    nickname: &'a str
}

impl ClientIdentity<'_> {
    pub fn new_anonymous() -> Self {
        let uid = Uuid::new_v4();
        ClientIdentity {
            username: uid,
            nickname: "anonymous"
        }
    }

    pub fn from_string(string: String) -> Self {
        serde_json::from_str(string.as_str()).unwrap()
    }
}