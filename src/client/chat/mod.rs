use core::panic;
use std::{net::{TcpStream}, io::{Read}, thread, sync::{Arc}, str::from_utf8, ops::Deref};

use crate::packet::{self, Packet};

pub struct Client {
    socket: Arc<TcpStream>
}

impl Client {
    pub fn new() -> Client { 
        match TcpStream::connect("localhost:6000") {
            Ok(stream) => {
                return Client { socket: Arc::new(stream) };
            },
            Err(e) => {
                panic!("Failed to connect: {}", e)
            }
        }
    }

    pub fn listener(&mut self) {
        let socket = self.socket.clone();

        let handle = thread::spawn(move || {

            let mut header_buffer = [0 as u8; packet::Header::HEADER_LENGTH];
            let mut socket = socket.deref();
            'reading_data: while match socket.read(&mut header_buffer) {
                Ok(n) => {
                    if n == 0 { 
                        break 'reading_data; 
                    }
                    let mut packet = Packet::new(header_buffer);
                    println!("\nNew message recevied. The data size is: {}", packet.header.data_size);
                    packet.get_message(socket);
                    println!("Message -> {}",  from_utf8(&packet.data).unwrap());
                    //TODO: DO SOMETHING WITH THE packet.data in the future

                    header_buffer.fill(0); // Reset the header buffer
                    true
                },
                Err(e) => {
                    println!("closing {e:?}");
                    false
                }
            } {}
        });
        handle.join().unwrap();
    }

    // FIXME: Fix this entire thing.
    // pub fn write_to_server(&mut self) {
    //     let mut counter = 1;
    //         'outer: loop {
    
    //         let message = format!("Message #{}, from {:?}", counter, self.socket.local_addr());
    //         match self.socket.write(message.as_bytes()) {
    //             Ok(_) => {
    //                 thread::sleep(Duration::from_millis(1000));
    //                 if counter > 3 {
    //                     self.socket.shutdown(Shutdown::Both).expect("shutdown call failed");
    //                     break 'outer;
    //                 };
    //                 counter += 1;
    //             }
    //             Err(_) => {
    //                 println!("Connection Closed");
    //             }
    //         } 
    //     }
    // }
}
