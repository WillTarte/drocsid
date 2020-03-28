extern crate crossbeam;

mod drocsid_client;
mod drocsid_server;
use drocsid_client::client::Client;
use std::net::{TcpListener, TcpStream};
use std::thread::sleep;
use std::time::Duration;
use std::io::ErrorKind;
use crossbeam::channel::{unbounded, Receiver, Sender};


const ADDRESS: &str = "127.0.0.1:9001";

fn main() -> () {

    let server: TcpListener = TcpListener::bind(ADDRESS).expect("failed to listen");
    server.set_nonblocking(true);
    let mut clients: Vec<Client> = Vec::new();


    for stream in server.incoming() {
        match stream {
            Result::Ok(client) => {
                println!("New client!");
                clients.push(Client::new().clone());
            }
            Result::Err(ref err) if err.kind() == ErrorKind::WouldBlock => {}
            Result::Err(err) => {
                println!("Failed with error {}", err);
            }
        }
    }
}