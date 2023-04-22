use crate::bitvec::BitVec;
use crate::hamessage::HammingMessage;
use rand::Rng;

pub struct Channel{
    packets: Vec<HammingMessage>
}

impl Channel{
    pub fn new() -> Self{
        Self { packets: Vec::new() }
    }

    pub fn add_packet(&mut self, packet: HammingMessage){
        self.packets.push(packet);
    }

    pub fn get_packets(&self) -> Vec<HammingMessage>{
        self.packets.clone()
    }

    pub fn make_some_errors(&mut self){
        for packet in &mut self.packets{
            let temp = rand::thread_rng().gen_range(0..packet.len()/4 as usize);
            for _ in 0..2{
                packet.make_error();
            }
        }
    }

    pub fn repair_and_decode(self) -> Vec<BitVec>{
        let mut new_packets = Vec::with_capacity(self.packets.len());
        for packet in self.packets{
            new_packets.push(packet.decode_and_repair());
        }
        new_packets
    }
}