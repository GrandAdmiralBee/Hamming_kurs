use crate::bitvec::BitVec;

pub fn generate(sequence: &BitVec) -> BitVec{


    let mut result = sequence.clone();
    let len = sequence.len() as u32;

    let mut indices = Vec::new();

    let mut index = 0u32;

    let mut pow = 0;
    while 2u32.pow(pow) < len{
        index = 2u32.pow(pow);
        pow += 1;

        result.insert((index-1) as usize, false);
        indices.push(index-1);
    }

    let len = result.len() as u32;
    
    if 2u32.pow(indices.len() as u32) < len{
        indices.push(index*2-1);
        result.insert((index*2-1) as usize, false);
    }

    for index in &indices{
        let mut skip = true;
        let mut count = *index;
        let mut temp = false;
        while count < len{
            for _ in 0..index+1{
                if skip{
                    skip = false;
                    count += 1;
                    continue;
                }
                if count >= len{
                    break;
                }
                temp ^= result[count as usize];
                count += 1;
            }
            count+=index+1;
        }
        result.index_mut(*index as usize, temp)
    }

    result
}


pub fn repair_and_deconstruct(sequence: &BitVec) -> BitVec{


    let mut result = sequence.clone();

    let len = sequence.len() as u32;

    let mut indices = Vec::new();

    let mut pow = 0;
    while 2u32.pow(pow) < len{
        let index = 2u32.pow(pow);
        pow+=1;
        indices.push(index-1);
    }

    for index in &indices{
        let mut skip = true;
        let mut count = *index;
        let mut temp = false;
        while count < len{
            for _ in 0..index+1{
                if skip{
                    skip = false;
                    count+=1;
                    continue;
                }
                if count >= len{
                    break;
                }
                temp ^= result[count as usize];
                count +=1;
            }
            count+=index+1;
        }
        result.index_mut(*index as usize, temp)
    }

    let mut tmp = 0usize;

    for index in &indices{
        if result[*index as usize] != sequence[*index as usize]{
            tmp += *index as usize + 1;
        }
    }

    if tmp > 0{
        println!("Исправлена ошибка по индексу {}", tmp-1);
        result.index_mut(tmp-1, !result[tmp-1]);
    }

    let mut count = 0u32;
    for index in &indices{
        if *index > result.len() as u32{
            break
        }
        result.remove((index-count) as usize);
        count += 1;
    }   

    result
}

pub fn deconstruct(sequence: &BitVec) -> BitVec{

    let mut result = sequence.clone();

    let len = sequence.len() as u32;

    let mut indices = Vec::new();

    let mut pow = 0;
    while 2u32.pow(pow) < len{
        let index = 2u32.pow(pow);
        pow+=1;
        indices.push(index-1);
    }


    let mut count = 0u32;
    for index in &indices{
        if *index > result.len() as u32{
            break
        }
        result.remove((index-count) as usize);
        count += 1;
    }    

    result
}

pub fn repair(sequence: &BitVec) -> BitVec{

    let mut result = sequence.clone();

    let len = sequence.len() as u32;

    let mut indices = Vec::new();

    let mut pow = 0;
    while 2u32.pow(pow) < len{
        let index = 2u32.pow(pow);
        pow+=1;
        indices.push(index-1);
    }

    for index in &indices{
        let mut skip = true;
        let mut count = *index;
        let mut temp = false;
        while count < len{
            for _ in 0..index+1{
                if skip{
                    skip = false;
                    count+=1;
                    continue;
                }
                if count >= len{
                    break;
                }
                temp ^= result[count as usize];
                count +=1;
            }
            count+=index+1;
        }
        result.index_mut(*index as usize, temp)
    }

    let mut tmp = 0usize;

    for index in &indices{
        if result[*index as usize] != sequence[*index as usize]{
            tmp += *index as usize + 1;
        }
    }

    if tmp > 0{
        println!("Исправлена ошибка по индексу {}", tmp-1);
        result.index_mut(tmp-1, !result[tmp-1]);
    }

    for index in &indices{
        let mut skip = true;
        let mut count = *index;
        let mut temp = false;
        while count < len{
            for _ in 0..index+1{
                if skip{
                    skip = false;
                    count+=1;
                    continue;
                }
                if count >= len{
                    break;
                }
                temp ^= result[count as usize];
                count +=1;
            }
            count+=index+1;
        }
        result.index_mut(*index as usize, temp)
    }

    result
}