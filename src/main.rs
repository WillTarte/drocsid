#![feature(async_closure)]

extern crate crossbeam;

mod drocsid_client;
mod drocsid_server;
use drocsid_client::client::Client;
use std::net::{TcpListener};
use std::io::{ErrorKind, Read};
use crossbeam::channel::{unbounded};
use crate::drocsid_server::server::Server;
use tokio::sync::mpsc;
use std::thread;

const ADDRESS: &str = "127.0.0.1:9001";

fn process_conn(client: &mut Client) {
    let mut buf: [u8; 1000] = [0 as u8; 1000];
    client.stream.read(&mut buf);

    let s = String::from_utf8_lossy(&buf);
    println!("{}", s)
}

fn main() -> () {

    let listener: TcpListener = TcpListener::bind(ADDRESS).expect("failed to listen");
    listener.set_nonblocking(true).expect("Cannot set non-blocking mode");
    let (server_sender, client_receiver) = unbounded::<String>();
    let (client_sender, server_receiver) = mpsc::channel::<String>(1000);
    let mut server: Server = Server::new(server_sender, server_receiver);

    loop
    {
        if let Ok((mut socket, addr)) = listener.accept() {
            println!("New client!");
            server.new_client(Client::new(client_receiver.clone(),client_sender.clone(), socket.try_clone().expect("Failed to clone conn")));
        }

        for mut client in &server.clients {
            process_conn(&mut client);
        }

    }
}