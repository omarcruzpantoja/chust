use std::{net::TcpStream, io::Read, vec};


#[derive(Debug)]
pub struct Header {
    // TODO: Implement generic usize for it to be 64 or 32
    pub data_size: usize, 
}

impl Header {
    pub const HEADER_LENGTH: usize = 8;

    pub fn new(header_in_bytes: [u8; Header::HEADER_LENGTH]) -> Header {
        Header { data_size: Header::from_be_bytes(header_in_bytes) }
    }

    fn from_be_bytes(byte_array: [u8; Header::HEADER_LENGTH]) -> usize {
        // TODO: IMPLEMENT GENERIC u32 or u64 of this method. For now we're defaulting to u64
        let total_bytes = 8;
        let mut data_size: u64 = 0;
        // The following loops reduces the need to do manually the following line:
        // byte_array[3] as u32 | ((byte_array[2] as u32) << 8) | ((byte_array[1] as u32) << 16 ) | ((byte_array[0] as u32) << 24)
        for i in 0 .. total_bytes {
            data_size |= (byte_array[i] as u64) << (total_bytes - i - 1) * 8;
        }
        data_size as usize
    }

}

pub struct Packet {
    pub header: Header,
    pub data: Vec<u8>
}

impl Packet {

    pub fn new(header_in_bytes: [u8; Header::HEADER_LENGTH]) -> Packet {
        let header = Header::new(header_in_bytes);
        Packet { header: Header::new(header_in_bytes), data: vec![0 as u8; header.data_size] }
    }

    pub fn prepare_packet(mut message: Vec<u8>) -> Vec<u8> {
        // Function prepares the packet to be sent. Adds the header and appends the content of the message into a single vec.
        let mut len_message: Vec<u8> = message.len().to_be_bytes().to_vec();
        let mut packet_data: Vec<u8> = Vec::new();
        println!("Packet Size: {:?}", message.len());
        packet_data.append(&mut len_message);
        packet_data.append(&mut message);
        packet_data
    }

    pub fn get_message(&mut self, mut socket: &TcpStream) {
        // Function uses the header info to figure out how much data from the stream we have to read in order to get the entire packet.
        let mut data_size = self.header.data_size;
        let packet_data = &mut self.data;
        
        while data_size > 0 {
            let mut message_buffer = if data_size > 1024 { vec![0 as u8; 1024] } else { vec![0 as u8; data_size] };
            
            match socket.read(&mut message_buffer) {
                Ok(n) => {
                    packet_data.append(&mut message_buffer);
                    data_size -= n;
                },
                Err(e) => {
                    println!("SOME ERROR HAPPENED WHILE READING {e:?}");

                }
            }
        }
    }
}