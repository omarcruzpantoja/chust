use core::panic;
use std::{net::{TcpStream, Shutdown}, io::Write, thread, time::Duration};

pub struct Client {
    socket: TcpStream
}

impl Client {
    pub fn new() -> Client { 
        match TcpStream::connect("localhost:6000") {
            Ok(stream) => {
                return Client { socket: stream };
            },
            Err(e) => {
                panic!("Failed to connect: {}", e)
            }
        }
    }

    pub fn write_to_server(&mut self) {
        let mut counter = 1;
            'outer: loop {
    
            let message = format!("Message #{}, from {:?}", counter, self.socket.local_addr());
            match self.socket.write(message.as_bytes()) {
                Ok(_) => {
                    thread::sleep(Duration::from_millis(1000));
                    if counter > 3 {
                        self.socket.shutdown(Shutdown::Both).expect("shutdown call failed");
                        break 'outer;
                    };
                    counter += 1;
                }
                Err(_) => {
                    println!("Connection Closed");
                }
            } 
        }
    }
}
