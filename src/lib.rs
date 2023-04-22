pub mod block;
pub mod bitvec;
pub mod cyclic;
pub mod hamessage;
pub mod channel;
mod randseq;
//Порождающий полиномы степеней 3, 4, 5
//3:
//  1, 1, 0, 1  ;   1, 0, 1, 1
//4:
//  1, 0, 0, 1, 1   ;   1, 1, 0, 0, 1
//5:
//  1, 0, 0, 1, 0, 1    ;   1, 0, 1, 0, 0, 1    ;   


use bitvec::BitVec;
#[derive(Debug, Clone)]
pub enum HammingType{
    //block hamming code
    Block,
    //7-4 hamming code
    Code_7_4(BitVec),
    //15-11 hamming code
    Code_15_11(BitVec),
    //31-26 hamming code
    Code_31_26(BitVec),
}

pub fn get_error_sindrome_index(rem: &BitVec, typ: HammingType) -> usize{
    let (check_len, polynom) = match typ{
        HammingType::Code_7_4(polynom) => (4,polynom),
        HammingType::Code_15_11(polynom) => (11,polynom),
        HammingType::Code_31_26(polynom) => (26, polynom),
        HammingType::Block => (0, BitVec::new()),
    };

    let mut check_message = BitVec::one(check_len);
    let x = BitVec::identity(polynom.len());
    check_message *= x;
    
    let (_, rem_1) = check_message.div_and_rem(&polynom);

    check_message^= rem_1;
    let mut res = 0;
    for i in 0..check_message.len(){
        check_message.index_mut(i, !check_message[i]);
        let (_,mut rem_1) = check_message.div_and_rem(&polynom);
        for _ in 0..(polynom.len() - 1 - rem_1.len()){
            rem_1.shift_at_r(0);
        }
        if rem_1 == rem{
            res = i;
            //break;
        }
        check_message.index_mut(i, !check_message[i]);
    };
    res
}