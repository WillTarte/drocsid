extern crate crossbeam;
#[macro_use]
extern crate log;

mod drocsid_client;
mod drocsid_server;

use drocsid_client::client::Client;
use crossbeam::channel::{unbounded};
use crate::drocsid_server::server::{Server, Message, MessageType, MessageEmitter};
use std::borrow::BorrowMut;
use threadpool::ThreadPool;
use std::net::{TcpListener};
use std::io::{Read, Write};
use std::sync::mpsc;
use log::{info, warn};
use std::error::Error;
use std::io;

const ADDRESS: &str = "127.0.0.1:9001";

fn process_conn(client: &mut Client) {
    let mut buf: [u8; 1000] = [0 as u8; 1000];

    match client.stream.read(buf.borrow_mut()) {
        Ok(ref size) => {
            info!("read {} bytes", size);
            if *size <= 0 as usize {
                return;
            }
        }
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => { /*do nothing*/ }
        Err(err) => {
            warn!("[process_conn - 33] {:?}", err);
            return;
        }
    }
    match serde_json::from_slice(buf.borrow_mut()) {
        Ok(message) => {
            info!("{:?}", &message);
            client.sender.send(message);
        }
        Err(err) => warn!("[process_conn - 42] {:?}", err)
    }
}

fn main() -> () {
    env_logger::init();

    let listener: TcpListener = TcpListener::bind(ADDRESS).expect("failed to listen");
    listener.set_nonblocking(true).expect("Cannot set non-blocking mode");
    let (server_sender, client_receiver) = unbounded::<String>();
    let (client_sender, server_receiver) = mpsc::channel::<Message>();
    let mut server: Server = Server::new(server_sender, server_receiver);
    let pool: threadpool::ThreadPool = ThreadPool::new(8);

    loop
    {
        match listener.accept() {
            Ok((socket, addr)) => {
                socket.set_nonblocking(true).expect("Could not set remote connection to non-blocking");
                info!("New client!");
                server.new_client(Client::new(client_receiver.clone(), client_sender.clone(), socket));
            }
            Err(err) => warn!("Could not accept incoming connection")
        }

        for client in server.clients.iter_mut() {
            let mut client_clone = client.clone();
            pool.execute(move || process_conn(&mut client_clone))
        }

        while let result = server.server_receiver.try_recv() {
            match result {
                Ok(msg) => {
                    match msg.msg_type {
                        MessageType::ChatMessage => {
                            for client in server.clients.iter_mut() {
                                match msg.emitter {
                                    MessageEmitter::Client(ref id) => {
                                        if *id == client.identity.username {
                                            //does not send message to itself
                                            ()
                                        } else {
                                            client.stream.write(serde_json::to_string(&msg).unwrap().as_bytes());
                                        };
                                    }
                                    MessageEmitter::Server(ref id) => { /*TODO*/ }
                                }
                            }
                        }
                        MessageType::AuthenticateIdentity => {
                            match msg.emitter {
                                MessageEmitter::Server(uuid) => { /*TODO*/ }
                                MessageEmitter::Client(uuid) => { /*TODO*/ }
                            }
                        }
                    }
                }
                Err(err) => {
                    warn!("[main - 99] {:?}", err);
                    break;
                }
            }
        }
    }
}