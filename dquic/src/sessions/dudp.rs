use std::io::Write;

use socket2::SockAddr;



pub struct UdpSession{
    pub id: u32,
    pub buffer: Vec<u8>,
    buffer_size: usize,
    pub address: SockAddr
}

pub trait Session{
    fn get_id(&self) -> u32;
    fn get_address(&self) -> SockAddr;
    fn get_buffer(&self) -> Vec<u8>;
    fn set_buffer(&mut self, buffer: Vec<u8>);
    fn read(&self) -> Vec<u8>;
}

impl Session for UdpSession{
    fn get_id(&self) -> u32{
        self.id
    }

    fn get_address(&self) -> SockAddr{
        self.address
    }

    fn get_buffer(&self) -> Vec<u8>{
        self.buffer.clone()
    }

    fn set_buffer(&mut self, buffer: Vec<u8>){
        self.buffer = buffer;
    }
    
    fn read(&self) -> Vec<u8> {
        self.buffer.clone()
    }
}

pub fn read(session: impl Session, buffer: Vec<u8>){
    println!("Reading from session {}", session.get_id());
}