use std::{net::{TcpListener, TcpStream, Shutdown, SocketAddr}, io::{Write, Read}, sync::{Arc, Mutex}, collections::HashMap, ops::Deref, str::from_utf8};
// use std::str::from_utf8;
use std::{thread, time::Duration};

mod server;

fn main() -> std::io::Result<()> {
    let mut server = server::Server::new();
    server.start();
    Ok(())
}