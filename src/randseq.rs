use crate::bitvec::BitVec;

pub fn generate_sequence(key: &BitVec, polynom:  &BitVec, size: usize) -> BitVec{
    
    let len = (2u32.pow(key.len() as u32) - 1) as usize;
    let period = key.len();

    let mut result = BitVec::with_capacity(len);

    for i in 0..key.len(){
        result.push(key[i]);
    }

    for i in period..size{
        let mut temp = false;
        let mut count = 0;
        for j in i-period..i-1{
            if polynom[count] == true{
                temp^=result[j];
            }
            count+=1;
        }
        result.push(temp);        
    }

    result
}