use crate::bitvec::{BitVec};
use super::CyclicError;

static POLYNOM_BIT_LEN: usize = 4;
static MESSAGE_BIT_LEN: usize = 4;

pub fn generate_7_4(message: &BitVec, polynom: &BitVec) -> Result<BitVec, CyclicError>{
    if message.len() > MESSAGE_BIT_LEN {
        return Err(CyclicError::SevenFourError(String::from("len of segment is greater than 4")));
    } else{
        let polynom = match polynom.len(){
            4 => polynom.clone(),
            // Если был передан полином неверной длины, то возьмем полином 1011
            _ => BitVec::from_u8_slice(&[1, 0, 1, 1]),
        };
        let mut m = message.clone();
        let x = BitVec::identity(polynom.len());
        m*=x;

        let (_, rem) = m.div_and_rem(&polynom);
        m ^= rem.clone();       
        return Ok(m)
    }
}

pub fn repair_7_4(message: &BitVec, polynom: &BitVec) -> Result<BitVec, CyclicError>{
    if message.len() > MESSAGE_BIT_LEN + POLYNOM_BIT_LEN - 1 || polynom.len() != POLYNOM_BIT_LEN {
        return Err(CyclicError::SevenFourError(String::from("Len of message is greater than 7 or len of polynom is not equal 4")));
    } else{
        let mut m = message.clone();

        let (_, mut rem) = m.div_and_rem(&polynom);
        let mut i = 0;
        if rem.count_zeros() == 0{
            return Ok(m);
        }
        while rem.count_ones() > 1{
            m >>= 1;
            rem = m.clone() % polynom.clone();
            i+=1;
        }

        for _ in 0..(POLYNOM_BIT_LEN - 1 - rem.len()){
            rem.shift_at_r(0);
        }

        if i == 0{
            for j in 0..rem.len(){
                if rem[j] {
                    println!("Исправлена ошибка по индексу {}", MESSAGE_BIT_LEN + j);
                }
            }
        }else{
            println!("Исправлена ошибка по индексу {}", MESSAGE_BIT_LEN-i);
        }

        m^=rem;
        m <<= i;

        return Ok(m)
    }
}

pub fn reduce_7_4(message: &BitVec) -> Result<BitVec, CyclicError>{
    if message.len() > MESSAGE_BIT_LEN + POLYNOM_BIT_LEN - 1 {
        return Err(CyclicError::SevenFourError(String::from("Len of message is greater than 7")));
    } else{
        let mut m = message.clone();

        for _ in 0..(POLYNOM_BIT_LEN - 1){
            m.remove(m.len()-1);
        }
        return Ok(m)
    }
}

pub fn repair_and_reduce_7_4(message: &BitVec, polynom: &BitVec) -> Result<BitVec, CyclicError>{
    let message = repair_7_4(message, polynom)?;
    reduce_7_4(&message)
}