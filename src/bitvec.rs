use std::{fmt::Display, ops::Index};
use std::ops::{
    Div, DivAssign, Rem, RemAssign, Mul, MulAssign, BitXor, BitXorAssign, ShlAssign, ShrAssign
};
use std::cmp::PartialEq;

const BIT_NUM: usize = 8;

pub fn bitvec(value: &[u8]) -> BitVec{
    BitVec::from_u8_slice(value)
}

// Коллекция, позволяющая хранить биты и иметь привычные методы управления ими
// Collection of bits with common methods for vectors
#[derive(Clone, Debug)]
pub struct BitVec{
    bytes: Vec<u8>,
    index_bit_false: bool,
    index_bit_true: bool,
    len: usize,
    cap: usize,
}

impl BitVec{
    pub fn one(len: usize) -> Self{
        let cap = (len as f32/BIT_NUM as f32).ceil() as usize;
        let mut bytes = Vec::with_capacity(cap);
        for _ in 0..cap{
            bytes.push(255);
        }
        Self{
            bytes,
            len,
            cap,
            ..Default::default()
        }
    }

    pub fn new() -> Self{
        Self::default()
    }

    pub fn with(value: bool) -> Self{
        let mut res = Self::new();
        res.push(value);
        res
    }

    pub fn with_capacity(cap: usize) -> Self{
        let vec_capacity = (cap as f32/BIT_NUM as f32).ceil() as usize;

        let bytes = Vec::with_capacity(vec_capacity);

        Self{
            bytes,
            len: 0,
            cap,
            ..Default::default()
        }
    }

    pub fn from_u8_slice(value: &[u8]) -> Self{
        let mut bit_vec = BitVec::with_capacity(value.len());
        bit_vec.push_u8_slice(value);

        bit_vec
    }

    pub fn from_bool_slice(value: &[bool]) -> Self{
        let mut bit_vec = BitVec::with_capacity(value.len());
        bit_vec.push_bool_slice(value);

        bit_vec
    }

    pub fn from_str(str: &str) -> Self{
        Self::from_bit_slice(&str.as_bytes())
    }

    pub fn from_bit_slice(value: &[u8]) -> Self{
        let bytes = value.to_vec();
        let cap = value.len() * BIT_NUM;
        let len = cap;

        BitVec{
            bytes,
            cap,
            len,
            ..Default::default()
        }
    }

    pub fn identity(len: usize) -> Self{
        let vec_capacity = (len as f32/BIT_NUM as f32).ceil() as usize;
        let mut bytes = Vec::with_capacity(vec_capacity);

        bytes.push(128);
        for _ in 1..vec_capacity{
            bytes.push(0);
        }

        Self{
            bytes,
            len,
            cap: vec_capacity,
            ..Default::default()
        }
    }

    pub fn push(&mut self, value: bool){
        if self.size_check_add(1){
            self.resize_add(1);
            self.index_mut(self.len-1, value);
        } else{
            self.len+=1;
            self.index_mut(self.len-1, value);
        }
    }

    pub fn push_u8(&mut self, value: u8){
        let bits = Self::split_u8(value);
        self.resize_add(BIT_NUM);
        for i in 0..BIT_NUM{
            self.index_mut(self.len - (BIT_NUM-i), bits[i])
        }
    }

    pub fn push_u8_slice(&mut self, bytes: &[u8]){
        let len = bytes.len();
        if self.size_check_add(len){
            self.resize_add(len);
        }else{
            self.len+=len;
        }
        for i in 0..len{
            let value = match bytes[i]{
                1 => true,
                0 => false,
                _ => false,
            };
            self.index_mut(self.len() - len + i, value);
        }
    }

    pub fn push_bool_slice(&mut self, bytes: &[bool]){
        let len = bytes.len();
        if self.size_check_add(len){
            self.resize_add(len);
        }else{
            self.len+=len;
        }
        for i in 0..len{
            self.index_mut(self.len() - len + i, bytes[i]);
        }
    }

    pub fn push_bit_slice(&mut self, bytes: &[u8]){
        for byte in bytes{
            self.push_u8(*byte)
        }
    }

    fn resize_add(&mut self, added_bits: usize){
        let new_len_bits = added_bits + self.len;
        let new_len_bytes = (new_len_bits as f32 / BIT_NUM as f32).ceil() as usize;

        if new_len_bytes > self.bytes.len(){
            self.cap += BIT_NUM;
        }
        self.bytes.resize(new_len_bytes, 0u8);
        self.len = new_len_bits;
    }

    fn resize_remove(&mut self, removed_bits: usize){
        let new_len_bits = self.len - removed_bits;
        let new_len_bytes = (new_len_bits as f32 / BIT_NUM as f32).ceil() as usize;

        if new_len_bytes < self.bytes.len(){
            self.cap -= BIT_NUM;
        }
        self.bytes.resize(new_len_bytes, 0u8);
        self.len = new_len_bits;
    }

    pub fn size_check_remove(&self, removed_bits: usize) -> bool {
        let new_len_bits = self.len - removed_bits;
        let new_len_bytes = (new_len_bits as f32 / BIT_NUM as f32).ceil() as usize;
        
        let diff = self.bytes.len() - new_len_bytes;

        if diff == 0 {
            false
        } else{
            true
        }
    }

    fn size_check_add(&self, added_bits: usize) -> bool{
        let new_len_bits = added_bits + self.len;
        let new_len_bytes = (new_len_bits as f32 / BIT_NUM as f32).ceil() as usize;

        let diff = new_len_bytes - self.bytes.len();

        if diff == 0 {
            false
        } else{
            true
        }
    }

    pub fn shift_at_r(&mut self, index: usize){
        let index = index;
        let bit_location = ((index as f32 + 1.0)/BIT_NUM as f32).ceil() as usize;
        let bit_offset = (index)%BIT_NUM;

        let mut lost_bits: Vec<u8> = Vec::with_capacity(bit_offset+1);

        let mut prev_bit = 0;
        #[allow(unused)]// It IS actually used, rust bug
        let mut next_bit = 0;

        if self.size_check_add(1){
            self.resize_add(1);
        }else{
            self.len+=1;
        }

        let mut first_iter = true;

        for i in bit_location-1..self.bytes.len(){
            if first_iter{
                for j in 0..bit_offset{
                    lost_bits.push((self.bytes[i] >> BIT_NUM-1-j) & 1);
                }

                next_bit = self.bytes[i] & 1;
                self.bytes[i] >>= 1;
            
                let mut index = (bit_location - 1)*BIT_NUM;
                for bit in &lost_bits{
                    let value = match bit{
                        1 => true,
                        0 => false,
                        _ => false,
                    };
                    self.index_mut(index, value);
                    index+=1;
                }
                first_iter = false;
            }else{
                next_bit = self.bytes[i] & 1;
                self.bytes[i] >>= 1;
                self.bytes[i] += prev_bit<<(BIT_NUM-1);
            }
            prev_bit = next_bit;
        }
    }

    pub fn shift_at_l(&mut self, index: usize){
        let index = index;
        let bit_location = ((index as f32 + 1.0)/BIT_NUM as f32).ceil() as usize;
        let bit_offset = (index)%BIT_NUM;

        let mut lost_bits: Vec<u8> = Vec::with_capacity(bit_offset+1);

        let mut prev_bit = 0;
        #[allow(unused)]// It IS actually used, rust bug
        let mut next_bit = 0;

        for i in (bit_location-1..self.bytes.len()).rev(){
            if i == bit_location-1{
                for j in 0..bit_offset{
                    lost_bits.push((self.bytes[i] >> BIT_NUM-1-j) & 1);
                }

                next_bit = self.bytes[i] & 128;
                self.bytes[i] <<= 1;
                self.bytes[i] += prev_bit >> BIT_NUM-1;
                let mut index = (bit_location - 1)*BIT_NUM;
                for bit in &lost_bits{
                    let value = match bit{
                        1 => true,
                        0 => false,
                        _ => false,
                    };
                    self.index_mut(index, value);
                    index+=1;
                }        
            } else{
                next_bit = self.bytes[i] & 128;
                self.bytes[i] <<= 1;
                self.bytes[i] += prev_bit >> (BIT_NUM-1);
            }
            prev_bit = next_bit;
        }

        if self.size_check_remove(1){
            self.resize_remove(1);
        }else{
            self.len-=1;
        }
    }

    pub fn insert(&mut self, index: usize, value: bool){
        self.shift_at_r(index);
        self.index_mut(index, value);
    }

    pub fn remove(&mut self, index: usize){
        self.shift_at_l(index)
    }

    #[inline]
    pub fn len(&self) -> usize{
        self.len
    }

    #[inline]
    pub fn capacity(&self) -> usize{
        self.cap
    }

    fn split_u8(value: u8) -> [bool; BIT_NUM]{
        let mut res = [false; BIT_NUM];
        
        for i in 0..BIT_NUM{
            res[i] = match (value >> (BIT_NUM - 1 - i)) & 1{
                0 => false,
                1 => true,
                _ => false,
            }
        }

        res
    }

    pub fn as_slice(&self) -> &[u8]{
        &self.bytes
    }

    pub fn as_split_slice(&self) -> Vec<bool>{
        let mut res = Vec::new();
        let mut i = 0;
        for byte in &self.bytes{
            let a = Self::split_u8(*byte);
            for b in a{
                i+=1;
                if i > self.len(){
                    break;
                }
                res.push(b);
            }
        }

        res
    }

    pub fn try_to_string(&self) -> String{
        std::str::from_utf8(self.bytes.as_slice()).unwrap_or("Warning, failed message!").to_string()
    }

    pub fn index(&self, index: usize) -> u8 {
        let bit_location = ((index as f32 + 1.0)/BIT_NUM as f32).ceil() as usize;
        let bit_offset = (index)%BIT_NUM;

        let mut byte = self.bytes[bit_location-1].clone();
        byte >>= BIT_NUM-1-bit_offset;

        byte & 1
    }

    pub fn index_mut(&mut self, index: usize, value: bool){

        let bit_location = ((index as f32 + 1.0)/BIT_NUM as f32).ceil() as usize;
        let bit_offset = (index)%BIT_NUM;

        let check_byte = 1 << (BIT_NUM-1-bit_offset);
        let bit = if (self.bytes[bit_location-1]/check_byte)%2 != 0{
            1
        } else{
            0
        };

        match value{
            true => {
                if bit == 0{
                    self.bytes[bit_location-1] += check_byte;
                }
            },
            false => {
                if bit == 1{
                    self.bytes[bit_location-1] -= check_byte;
                }
            }
        }
    }

    pub fn div_and_rem(&self, other: &Self) -> (Self, Self){
        let mut p1 = self.as_split_slice();
        let p2 = other.as_split_slice();

        let mut res = Vec::new();
        let mut i = 0;
        let mut first_iter = !p1[0];
        while (p1.len() - i) >= p2.len()
        {
            let zero_count = sub_poly(&mut p1[i..], &p2, &mut res);
            i += zero_count;

            if (p1.len() - i) >= p2.len(){
                if first_iter {
                    for _ in 0..zero_count{
                        res.push(false);
                    }
                    first_iter = false;
                } else{
                    for _ in 0..zero_count-1{
                        res.push(false);
                    }
                }
            }else if (p1.len() - i) < p2.len(){
                for _ in p2.len()..zero_count{
                    res.push(false);
                }   
            }
        }
        let rem = BitVec::from_bool_slice(&p1[i..]);
        let res = BitVec::from_bool_slice(&res);
        (res, rem)
    }

    pub fn count_ones(&self) -> u32{
        let mut total = 0;
        for byte in &self.bytes{
            total += byte.count_ones();
        }
        total
    }

    pub fn count_zeros(&self) -> u32{
        let mut total = 0;
        for byte in &self.bytes{
            total += byte.count_zeros();
        }
        total
    }
}

impl Default for BitVec{
    fn default() -> Self {
        Self{
            bytes: Vec::new(),
            index_bit_false: false,
            index_bit_true: true,
            len:0,
            cap:0,
        }
    }
}

impl Display for BitVec{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ ").unwrap();
        for i in 0..self.len{
            write!(f, "{} ", self.index(i)).unwrap();
            if (i+1)%8 == 0{
                write!(f, " ").unwrap();
            }
        }
        write!(f, "]")
    }
}

impl Index<usize> for BitVec{
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        match self.index(index){
            1 => &self.index_bit_true,
            0 => &self.index_bit_false,
            _ => &self.index_bit_false,
        }
    }
}

impl Div for BitVec{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let (div, _) = self.div_and_rem(&rhs);
        div
    }
}

impl DivAssign for BitVec{
    fn div_assign(&mut self, rhs: Self) {
        let (div, _) = self.div_and_rem(&rhs);
        self.cap = 0;
        self.len = 0;
        self.bytes = div.bytes;
    }
}

impl Rem for BitVec{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        let (_, rem) = self.div_and_rem(&rhs);
        rem
    }
}

impl RemAssign for BitVec{
    fn rem_assign(&mut self, rhs: Self) {
        let (_, rem) = self.div_and_rem(&rhs);
        self.cap = 0;
        self.len = 0;
        self.bytes = rem.bytes;
    }
}

impl Mul for BitVec{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = self.clone();
        for _ in (0..rhs.len()-1).rev(){
            res.push(false);
        }

        for i in 1..rhs.len(){
            if rhs[i] == false{
                continue;
            }
            let mut polynom = self.clone();
            for _ in 0..i{
                polynom.shift_at_r(0);
            }
            for _ in (i..rhs.len()-1).rev(){
                polynom.push(false);
            }
            res ^= polynom;
        }
        res
    }
}

impl MulAssign for BitVec{
    fn mul_assign(&mut self, rhs: Self) {
        let mut res = self.clone();
        for _ in (0..rhs.len()-1).rev(){
            res.push(false);
        }

        for i in 1..rhs.len(){
            if rhs[i] == false{
                continue;
            }
            let mut polynom = self.clone();
            for _ in 0..i{
                polynom.shift_at_r(0);
            }
            for _ in (i..rhs.len()-1).rev(){
                polynom.push(false);
            }
            res ^= polynom;
        }
        
        self.cap+=rhs.len()-1;
        self.len+=rhs.len()-1;

        self.bytes = res.bytes;
    }
}

impl BitXor for BitVec{
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        if self.len() >= rhs.len(){
            let mut res = self.clone();
            for i in 0..rhs.len(){
                let index = self.len() - rhs.len() + i;
                let init_value = res[index];
                res.index_mut(index, rhs[i]^init_value);
            }
            res
        }else{
            let mut res = rhs.clone();
            for i in 0..self.len(){
                let index = rhs.len() - self.len() + i;
                let init_value = res[index];
                res.index_mut(index, self[i]^init_value);
            }
            res
        }
    }
}

impl BitXorAssign for BitVec{
    fn bitxor_assign(&mut self, rhs: Self) {
        if self.len() >= rhs.len(){
            let mut res = self.clone();
            for i in 0..rhs.len(){
                let index = self.len() - rhs.len() + i;
                let init_value = res[index];
                res.index_mut(index, rhs[i]^init_value);
            }
            self.bytes = res.bytes;
        }else{
            let mut res = rhs.clone();
            for i in 0..self.len(){
                let index = rhs.len() - self.len() + i;
                let init_value = res[index];
                res.index_mut(index, self[i]^init_value);
            }
            self.len = res.len;
            self.cap = res.cap;
            self.bytes = res.bytes;
        }
    }
}

impl ShlAssign<usize> for BitVec{
    fn shl_assign(&mut self, rhs: usize) {
        for _ in 0..rhs{
            let lost = self[0];
            self.shift_at_l(0);
            self.push(lost);
        }
    }
}

impl ShrAssign<usize> for BitVec{
    fn shr_assign(&mut self, rhs: usize) {
        for _ in 0..rhs{
            let lost = self[self.len()-1];
            self.shift_at_r(0);
            self.index_mut(0, lost);
            if self.size_check_remove(1){
                self.resize_remove(1);
            }else{
                self.len-=1;
            }
        }
    }
}

fn sub_poly(p: &mut [bool], q: &[bool], res: &mut Vec<bool>) -> usize{
    let coeff = p[0];
    let mut zero_count = 0;
    if p[0] {
        res.push(true);
    }
    p[0] = false;
    for i in 1..q.len(){
        p[i] = p[i] ^ (coeff & q[i]);
    }
    while zero_count < p.len() && !p[zero_count]{
        zero_count+=1;
    }
    zero_count
}

impl PartialEq for BitVec{
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl PartialEq<&Self> for BitVec{
    fn eq(&self, other: &&Self) -> bool {
        self.bytes == *other.bytes
    }
}

//Implementation for splitting for cyclic messages
impl BitVec{
    pub fn split_into(&self, bits_in_segments: usize) -> Vec<BitVec>{
        let cap = (self.len() as f32 / bits_in_segments as f32).ceil() as usize;
        let mut res = Vec::with_capacity(cap);
        let mut c = 0;
        for _ in 0..cap{
            let mut temp = BitVec::with_capacity(bits_in_segments);
            for _ in 0..temp.capacity(){
                if c >= self.len(){
                    break;
                }
                temp.push(self[c]);
                c+=1;
            }
            res.push(temp);
        }
        res
    }
}