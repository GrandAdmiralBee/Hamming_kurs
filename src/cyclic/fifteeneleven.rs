use crate::bitvec::{BitVec};
use super::CyclicError;
use crate::get_error_sindrome_index;

static POLYNOM_BIT_LEN: usize = 5;
static MESSAGE_BIT_LEN: usize = 11;

pub fn generate_15_11(message: &BitVec, polynom: &BitVec) -> Result<BitVec, CyclicError>{
    if message.len() > MESSAGE_BIT_LEN || polynom.len() != POLYNOM_BIT_LEN {
        return Err(CyclicError::FifteenElevenError(String::from("len of segment is greater than 11")));
    } else{
        let mut m = message.clone();
        let x = BitVec::identity(polynom.len());
        m*=x;

        let (_, rem) = m.div_and_rem(&polynom);
        m ^= rem.clone();       
        return Ok(m)
    }
}

pub fn repair_15_11(message: &BitVec, polynom: &BitVec) -> Result<BitVec, CyclicError>{
    if message.len() > MESSAGE_BIT_LEN + POLYNOM_BIT_LEN - 1 || polynom.len() != POLYNOM_BIT_LEN {
        return Err(CyclicError::FifteenElevenError(String::from("Len of message is greater than 11 or len of polynom is not equal 5")));
    } else{
        let mut m = message.clone();

        let (_, mut rem) = m.div_and_rem(&polynom);
        if rem.count_zeros() == 0{
            return Ok(m);
        }

        for _ in 0..(POLYNOM_BIT_LEN - 1 - rem.len()){
            rem.shift_at_r(0);
        }

        let index = get_error_sindrome_index(&rem, crate::HammingType::Code_15_11(polynom.clone())) - (MESSAGE_BIT_LEN + POLYNOM_BIT_LEN - 1 - message.len());
        m.index_mut(index, !m[index]);
        println!("Исправлена ошибка по индексу {}", index);

        return Ok(m)
    }
}

pub fn reduce_15_11(message: &BitVec) -> Result<BitVec, CyclicError>{
    if message.len() > MESSAGE_BIT_LEN + POLYNOM_BIT_LEN - 1 {
        return Err(CyclicError::FifteenElevenError(String::from("Len of message is greater than 11")));
    } else{
        let mut m = message.clone();

        for _ in 0..(POLYNOM_BIT_LEN - 1){
            m.remove(m.len()-1);
        }
        return Ok(m)
    }
}

pub fn repair_and_reduce_15_11(message: &BitVec, polynom: &BitVec) -> Result<BitVec, CyclicError>{
    let message = repair_15_11(message, polynom)?;
    reduce_15_11(&message)
}