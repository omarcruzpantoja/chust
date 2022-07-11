use core::panic;
use std::{net::{TcpStream, Shutdown}, io::{Write, Read}, thread, time::Duration, sync::{Arc, Mutex}, str::from_utf8, ops::Deref};

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

    pub fn listener(&mut self) {
        let socket = self.socket.clone();

        let handle = thread::spawn(move || {
            let mut data = [0 as u8; 1024];
            let mut socket = socket.deref();
            'reading_data: while match socket.read(&mut data) {
                Ok(n) => {
                    if n == 0 { 
                        break 'reading_data; 
                    }
                    let stringified_data = from_utf8(&data).unwrap();
                    println!("{stringified_data}");
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
}
