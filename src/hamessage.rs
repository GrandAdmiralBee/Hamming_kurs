use crate::block::hamming::{repair, deconstruct, generate, repair_and_deconstruct};
use crate::cyclic::sevenfour::{generate_7_4, reduce_7_4, repair_7_4, repair_and_reduce_7_4};
use crate::cyclic::fifteeneleven::{generate_15_11, reduce_15_11, repair_15_11, repair_and_reduce_15_11};
use crate::cyclic::thirtyonetwentysix::{generate_31_26, reduce_31_26, repair_31_26, repair_and_reduce_31_26};
use crate::bitvec::BitVec;
use crate::HammingType;

use std::fmt::Display;

use rand::Rng;
use colored::*;

// Сообщение с добавочными битами хэмминга
#[derive(Debug, Clone)]
pub struct HammingMessage{
    pub data: Vec<BitVec>,
    //Если это циклический код, то записывается и полином
    typ: HammingType,
    len: usize
}

impl HammingMessage{
    pub fn new(bits: &BitVec, typ: HammingType) -> Self{
        match typ{
            HammingType::Block => {
                let mut data = Vec::with_capacity(1);
                data.push(generate(bits));
                return Self{
                    typ: HammingType::Block,
                    len: data[0].len(),
                    data,
                }
            },
            HammingType::Code_7_4(polynom) =>{
                let fours = bits.split_into(4);
                let mut segments = Vec::with_capacity(fours.len());
                for segment in &fours{
                    segments.push(generate_7_4(segment, &polynom).unwrap());
                }
                let mut len = 7 * (fours.len() - 1);
                len += segments[segments.len()-1].len();
                let typ = HammingType::Code_7_4(polynom);

                return Self { 
                    data: segments,
                    len,
                    typ,
                }
            },
            HammingType::Code_15_11(polynom) =>{
                let elevens = bits.split_into(11);
                let mut segments = Vec::with_capacity(elevens.len());
                for segment in &elevens{
                    segments.push(generate_15_11(segment, &polynom).unwrap());
                }
                let mut len = 15 * (elevens.len() - 1);
                len += segments[segments.len()-1].len();
                let typ = HammingType::Code_15_11(polynom);

                return Self { 
                    data: segments,
                    len,
                    typ,
                }
            },
            HammingType::Code_31_26(polynom) =>{
                let elevens = bits.split_into(26);
                let mut segments = Vec::with_capacity(elevens.len());
                for segment in &elevens{
                    segments.push(generate_31_26(segment, &polynom).unwrap());
                }
                let mut len = 31 * (elevens.len() - 1);
                len += segments[segments.len()-1].len();
                let typ = HammingType::Code_31_26(polynom);

                return Self { 
                    data: segments,
                    len,
                    typ,
                }
            },
        }
    }

    #[inline]
    pub fn len(&self) -> usize{
        self.len
    }

    // Создает ошибку в случайном бите
    pub fn make_error(&mut self) -> &mut Self{
        match &self.typ{
            HammingType::Block => {
                let error_index = rand::thread_rng().gen_range(0..self.data[0].len() as usize);
                println!("Внесена ошибка по индексу: {error_index}");
                let error = !self.data[0][error_index];
                self.data[0].index_mut(error_index, error);
            },
            HammingType::Code_7_4(_) => {
                let error_segment = rand::thread_rng().gen_range(0..self.data.len() as usize);
                print!("Внесена ошибка в сегмент: {error_segment} ");
                let max = self.data[error_segment].len();
                let error_index = rand::thread_rng().gen_range(0..max as usize);
                println!("в бит: {error_index}");
                let new_value = !self.data[error_segment][error_index];
                self.data[error_segment].index_mut(error_index, new_value);
            },
            HammingType::Code_15_11(_) => {
                let error_segment = rand::thread_rng().gen_range(0..self.data.len() as usize);
                print!("Внесена ошибка в сегмент: {error_segment} ");
                let max = self.data[error_segment].len();
                let error_index = rand::thread_rng().gen_range(0..max as usize);
                println!("в бит: {error_index}");
                let new_value = !self.data[error_segment][error_index];
                self.data[error_segment].index_mut(error_index, new_value);
            },
            HammingType::Code_31_26(_) => {
                let error_segment = rand::thread_rng().gen_range(0..self.data.len() as usize);
                print!("Внесена ошибка в сегмент: {error_segment} ");
                let max = self.data[error_segment].len();
                let error_index = rand::thread_rng().gen_range(0..max as usize);
                println!("в бит: {error_index}");
                let new_value = !self.data[error_segment][error_index];
                self.data[error_segment].index_mut(error_index, new_value);
            },
        }
        self
    }

    // Создает ошибку в заданном бите
    pub fn make_error_at(&mut self, index: usize) -> &mut Self{
        let mut segment_index = 0;
        let mut inner_index = 0;
        match &self.typ{
            HammingType::Block => {
                println!("Внесена ошибка по индексу: {index}");
                let error = !self.data[0][index];
                self.data[0].index_mut(index, error);
                return self;
            },
            HammingType::Code_7_4(_) => {
                segment_index = index/7;
                inner_index = index%7;
            }
            HammingType::Code_15_11(_) => {
                segment_index = index/15;
                inner_index = index%15;
            }
            HammingType::Code_31_26(_) => {
                segment_index = index/31;
                inner_index = index%31;
            }
        }
        println!("Внесена ошибка в сегмент {} по индексу {}", segment_index, inner_index);
        let error = !self.data[segment_index][inner_index];
        self.data[segment_index].index_mut(inner_index, error);
        self
    }

    // Чинит сообщение, не убирая добавочные биты
    pub fn repair(&mut self) -> &mut Self{
        match &self.typ {
            HammingType::Block => {
                let new_data = repair(&self.data[0]);
                self.data[0] = new_data;
            },
            HammingType::Code_7_4(polynom) => {
                for i in 0..self.data.len(){
                    self.data[i] = repair_7_4(&self.data[i], polynom).unwrap();
                }
            },
            HammingType::Code_15_11(polynom) => {
                for i in 0..self.data.len(){
                    self.data[i] = repair_15_11(&self.data[i], polynom).unwrap();
                }
            },
            HammingType::Code_31_26(polynom) => {
                for i in 0..self.data.len(){
                    self.data[i] = repair_31_26(&self.data[i], polynom).unwrap();
                }
            },
        }
        self
    }

    // Декодирование и починка кода Хэмминга. Возвращает обычное сообщение
    pub fn decode_and_repair(self) -> BitVec{
        match &self.typ{
            HammingType::Block => {
                repair_and_deconstruct(&self.data[0])
            },
            HammingType::Code_7_4(polynom) => {
                let mut m = Vec::with_capacity(self.data.len());
                for i in 0..m.capacity(){
                    m.push(repair_and_reduce_7_4(&self.data[i], polynom).unwrap());
                }
                let cap = m.iter().map(|var| var.len()).collect::<Vec<usize>>().iter().sum();
                let mut bits = BitVec::with_capacity(cap);
                for i in 0..m.len(){
                    bits.push_bool_slice(&m[i].as_split_slice());
                }
                bits
            }
            HammingType::Code_15_11(polynom) => {
                let mut m = Vec::with_capacity(self.data.len());
                for i in 0..m.capacity(){
                    m.push(repair_and_reduce_15_11(&self.data[i], polynom).unwrap());
                }
                let cap = m.iter().map(|var| var.len()).collect::<Vec<usize>>().iter().sum();
                let mut bits = BitVec::with_capacity(cap);
                for i in 0..m.len(){
                    bits.push_bool_slice(&m[i].as_split_slice());
                }
                bits
            }
            HammingType::Code_31_26(polynom) => {
                let mut m = Vec::with_capacity(self.data.len());
                for i in 0..m.capacity(){
                    m.push(repair_and_reduce_31_26(&self.data[i], polynom).unwrap());
                }
                let cap = m.iter().map(|var| var.len()).collect::<Vec<usize>>().iter().sum();
                let mut bits = BitVec::with_capacity(cap);
                for i in 0..m.len(){
                    bits.push_bool_slice(&m[i].as_split_slice());
                }
                bits
            }
        }
    }

    // Декодирование сообщения Хэмминга без починки. Возвращает обычное сообщение
    pub fn decode(self) -> BitVec{
        match self.typ {
            HammingType::Block => {
                deconstruct(&self.data[0])
            },
            HammingType::Code_7_4(_) => {
                let mut m = Vec::with_capacity(self.data.len());
                for i in 0..m.capacity(){
                    m.push(reduce_7_4(&self.data[i]).unwrap());
                }
                let cap = m.iter().map(|var| var.len()).collect::<Vec<usize>>().iter().sum();
                let mut bits = BitVec::with_capacity(cap);
                for i in 0..m.len(){
                    bits.push_bool_slice(&m[i].as_split_slice());
                }
                bits
            },
            HammingType::Code_15_11(_) => {
                let mut m = Vec::with_capacity(self.data.len());
                for i in 0..m.capacity(){
                    m.push(reduce_15_11(&self.data[i]).unwrap());
                }
                let cap = m.iter().map(|var| var.len()).collect::<Vec<usize>>().iter().sum();
                let mut bits = BitVec::with_capacity(cap);
                for i in 0..m.len(){
                    bits.push_bool_slice(&m[i].as_split_slice());
                }
                bits
            },
            HammingType::Code_31_26(_) => {
                let mut m = Vec::with_capacity(self.data.len());
                for i in 0..m.capacity(){
                    m.push(reduce_31_26(&self.data[i]).unwrap());
                }
                let cap = m.iter().map(|var| var.len()).collect::<Vec<usize>>().iter().sum();
                let mut bits = BitVec::with_capacity(cap);
                for i in 0..m.len(){
                    bits.push_bool_slice(&m[i].as_split_slice());
                }
                bits
            },
        }
    }
}

// Выводит биты сообщения, помечая проверочные красным цветом
impl Display for HammingMessage{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.typ{
            HammingType::Block => {
                write!(f, "[ ").unwrap();
                for i in 0..self.len(){
                    if (i+1).is_power_of_two(){
                        let value = self.data[0].index(i).to_string();
                        write!(f, "{} ", value.green()).unwrap();
                    }else{
                        write!(f, "{} ", self.data[0].index(i)).unwrap();
                    }
                    if (i+1)%8 == 0{
                        write!(f, " ").unwrap();
                    }
                }
                write!(f, "]")
            },
            _ => {
                for segment in &self.data{
                    println!("{}", segment);
                }
                writeln!(f, "")
            }
        }
    }
}
