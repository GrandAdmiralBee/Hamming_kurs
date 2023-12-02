use hamming_code_labs::bitvec::{bitvec, BitVec};
use hamming_code_labs::channel::Channel;
use hamming_code_labs::hamessage::HammingMessage;
use hamming_code_labs::HammingType;
use std::collections::VecDeque;

fn main() {
    let mut channel = Channel::new();

    //Кодирование блочным кодом хэмминга
    let data = BitVec::from_str(
        "And dreadfully distinct. Against the dark, a tall white fountain played.",
    );
    let message = HammingMessage::new(&data, HammingType::Block);
    channel.add_packet(message);

    //Кодирование с помощью кода 7-4
    let polynom = bitvec(&[1, 1, 0, 1]);
    let message = HammingMessage::new(&data, HammingType::Code_7_4(polynom));
    channel.add_packet(message);

    //Кодирование с помощью кода 15-11
    let polynom = bitvec(&[1, 0, 0, 1, 1]);
    let message = HammingMessage::new(&data, HammingType::Code_15_11(polynom));
    channel.add_packet(message);

    //Кодирование с помощью кода 31-26
    let polynom = bitvec(&[1, 0, 0, 1, 0, 1]);
    let mut message = HammingMessage::new(&data, HammingType::Code_31_26(polynom));
    message.make_error_at(5).make_error_at(10).repair();
    println!("{}", message.decode().try_to_string());
    //channel.add_packet(message);

    /* channel.make_some_errors();

    let messages = channel.repair_and_decode();

    for message in messages{
        println!("{}", message.try_to_string());
    } */
}
