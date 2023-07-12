#[cfg(test)]
mod test;

use std::fs;


pub trait SetGetBytes {
    fn set_byte(&mut self, byte: usize, data: u8);
    fn get_byte(&self, byte: usize) -> u8;
}
impl SetGetBytes for u64 {
    fn set_byte(&mut self, byte: usize, data: u8) {
        let mask: u64 = 0xff << byte * 8;
        let temp: u64 = *self & !mask;
        *self = temp | ((data as u64) << byte * 8);
    }

    fn get_byte(&self, byte: usize) -> u8 {
        return ((*self >> 8 * byte) & 0xff) as u8;
    }
}
impl SetGetBytes for u32 {
    fn set_byte(&mut self, byte: usize, data: u8) {
        let mask: u32 = 0xff << byte * 8;
        let temp: u32 = *self & !mask;
        *self = temp | ((data as u32) << byte * 8);
    }

    fn get_byte(&self, byte: usize) -> u8 {
        return ((*self >> 8 * byte) & 0xff) as u8;
    }
}
impl SetGetBytes for u16 {
    fn set_byte(&mut self, byte: usize, data: u8) {
        let mask: u16 = 0xff << byte * 8;
        let temp: u16 = *self & !mask;
        *self = temp | ((data as u16) << byte * 8);
    }

    fn get_byte(&self, byte: usize) -> u8 {
        return ((*self >> 8 * byte) & 0xff) as u8;
    }
}


/// This is the main part of this crate
/// implement this trait for the things you want to serialize and deserialize
/// 
/// this is meant to be used for saving things in files or transmitting things through buffers
/// # Warning
/// the normal String has maximum length of 255
/// if you want longer strings use the U16String wich allows
/// for strings with a length 65535
/// 
/// # Example
/// ```
/// use serialr::Serialize;
/// // string implements serialize
/// let string = "Hello World".to_string();
/// // serialize the string
/// let bytes = string.clone().serialize();
/// // deserialize from the beginning of the bytes
/// let found_string = String::deserialize(&bytes, 0).unwrap();
/// // the result will be the same
/// assert_eq!(string, found_string);
/// ```
/// 
pub trait Serialize: Sized {
    fn serialize(self) -> Bytes;
    fn deserialize(bytes: &Bytes, index: usize) -> Option<Self>;
    fn size(&self) -> usize;
}

/// This is the struct returned by serialize
/// 
/// Bytes can be used to save and load from files
/// and you are able to read and write to it
/// the main way you can write to it is by the read, push and write
/// functions they can use the turbo fish notation to write and read and push
/// any type that implements the Serialize trait
/// 
/// # Example
/// ```
/// use serialr::Bytes;
/// let mut bytes = Bytes::new();
/// let string = "Hello World".to_string();
/// bytes.push(string.clone());
/// let found_string = bytes.read::<String>(0).unwrap();
/// assert_eq!(string, found_string);
/// ```
#[derive(Debug, Clone, Default)]
pub struct Bytes(Vec<u8>, usize);
impl Bytes {
    pub fn new() -> Self {
        Self(vec![], 0)
    }
    pub fn to_vec(self) -> Vec<u8> {
        self.0
    }
    pub fn set_len(&mut self, len: usize) {
        while len > self.0.len() {
            self.0.push(0)
        }
        while len < self.0.len() {
            self.0.pop();
        }
    } 
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_inbound(&self, index: usize) -> bool {
        index < self.0.len()
    }

    pub fn read_byte(&self, index: usize) -> u8 {
        self.0[index]
    }
    pub fn write_byte(&mut self, index: usize, byte: u8) {
        self.0[index] = byte;
    }
    pub fn push_byte(&mut self, byte: u8) {
        self.0.push(byte);
    }
    
    pub fn write<T: Serialize>(&mut self, index: usize, val: T) {
        let bytes = val.serialize();
        self.insert(index, &bytes);
    }
    pub fn read<T: Serialize>(&self, index: usize) -> Option<T> {
        T::deserialize(self, index)
    }
    pub fn push<T: Serialize>(&mut self, val: T) {
        let bytes = val.serialize();
        self.append(&bytes);
    }
    
    pub fn insert(&mut self, index: usize, bytes: &Bytes) {
        for i in 0..bytes.len() {
            self.0[index + i] = bytes.read_byte(i);
        }
    }
    pub fn append(&mut self, bytes: &Bytes) {
        for byte in &bytes.0 {
            self.0.push(*byte)
        }
    }

    pub fn write_to_file(self, path: String) -> Result<(), std::io::Error> {
        fs::write::<String, Vec<u8>>(path, self.into())
    }
    pub fn read_from_file(path: String) -> Result<Bytes, std::io::Error> {
        match fs::read(path) {
            Ok(ok) => Ok(Bytes(ok, 0)),
            Err(err) => Err(err),
        }
        
    }
}
impl From<Bytes> for Vec<u8> {
    fn from(value: Bytes) -> Self {
        value.0
    }
}
impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Self(value, 0)
    }
}
impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        Self(value.to_vec(), 0)
    }
}
impl Iterator for Bytes {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(byte) = self.0.get(self.1) {
            self.1 += 1;
            return Some(*byte);

        } else {
            return None;
        }
    }
}

impl Serialize for u8 {
    fn serialize(self) -> Bytes {
        Bytes(vec![self], 0)
    }

    fn deserialize(bytes: &Bytes, index: usize) -> Option<Self> {
        if bytes.is_inbound(index) {
            return Some(bytes.read_byte(index))
        } else {
            return None;
        }
    }

    fn size(&self) -> usize {
        1
    }
}
impl Serialize for char {
    fn serialize(self) -> Bytes {
        Bytes(vec![self as u8], 0)
    }

    fn deserialize(bytes: &Bytes, index: usize) -> Option<Self> {
        if bytes.is_inbound(index) {
            return Some(bytes.read_byte(index) as char)
        } else {
            return None;
        }
    }

    fn size(&self) -> usize {
        1
    }
}
impl Serialize for u16 {
    fn serialize(self) -> Bytes {
        let mut res = Bytes::new();
        res.push(self.get_byte(1));
        res.push(self.get_byte(0));
        return res;
    }
    fn deserialize(bytes: &Bytes, index: usize) -> Option<Self> {
        if !bytes.is_inbound(index + 1) {
            return None;
        }
        let mut res = 0;
        res.set_byte(0, bytes.read_byte(index + 1));
        res.set_byte(1, bytes.read_byte(index + 0));
        return Some(res)
    }
    fn size(&self) -> usize {
        2
    }
}
impl Serialize for u32 {
    fn serialize(self) -> Bytes {
        let mut res = Bytes::new();
        res.push(self.get_byte(3));
        res.push(self.get_byte(2));
        res.push(self.get_byte(1));
        res.push(self.get_byte(0));
        return res;
    }
    fn deserialize(bytes: &Bytes, index: usize) -> Option<Self> {
        if !bytes.is_inbound(index + 3) {
            return None;
        }
        let mut res = 0;
        res.set_byte(0, bytes.read_byte(index + 3));
        res.set_byte(1, bytes.read_byte(index + 2));
        res.set_byte(2, bytes.read_byte(index + 1));
        res.set_byte(3, bytes.read_byte(index + 0));
        return Some(res)
    }
    fn size(&self) -> usize {
        4
    }
}
impl Serialize for u64 {
    fn serialize(self) -> Bytes {
        let mut res = Bytes::new();
        res.push(self.get_byte(7));
        res.push(self.get_byte(6));
        res.push(self.get_byte(5));
        res.push(self.get_byte(4));
        res.push(self.get_byte(3));
        res.push(self.get_byte(2));
        res.push(self.get_byte(1));
        res.push(self.get_byte(0));
        return res;
    }
    fn deserialize(bytes: &Bytes, index: usize) -> Option<Self> {
        if !bytes.is_inbound(index + 3) {
            return None;
        }
        let mut res = 0;
        res.set_byte(0, bytes.read_byte(index + 7));
        res.set_byte(1, bytes.read_byte(index + 6));
        res.set_byte(2, bytes.read_byte(index + 5));
        res.set_byte(3, bytes.read_byte(index + 4));
        res.set_byte(4, bytes.read_byte(index + 3));
        res.set_byte(5, bytes.read_byte(index + 2));
        res.set_byte(6, bytes.read_byte(index + 1));
        res.set_byte(7, bytes.read_byte(index + 0));
        return Some(res)
    }
    fn size(&self) -> usize {
        8
    }
}
impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(self) -> Bytes {
        let mut bytes = Bytes::new();
        bytes.push(self.len() as u16);
        for item in self {
            bytes.push(item)
        }
        bytes
    }
    fn deserialize(bytes: &Bytes, index: usize) -> Option<Self> {
        if !bytes.is_inbound(index+1) {
            return None;
        }
        let mut res = vec![];
        let len: u16 = bytes.read(index).unwrap();
        let mut offset = 2;
        for _ in 0..len {
            if let Some(item) = bytes.read::<T>(index + offset as usize) {
                offset += item.size();
                res.push(item);
            } else {
                return None;
            }
        }
        return Some(res);
    }
    fn size(&self) -> usize {
        let mut res = 2;
        for item in self {
            res += item.size();
        }
        return res;
    }
}
impl Serialize for String {
    fn serialize(self) -> Bytes {
        let mut bytes = Bytes::new();
        bytes.push_byte(self.len() as u8);
        for ch in self.chars() {
            bytes.push_byte(ch as u8);
        }
        return bytes;
    }
    fn deserialize(bytes: &Bytes, index: usize) -> Option<Self> {
        if !bytes.is_inbound(index) {
            return None;
        }
        let mut res = String::new();
        let len: u8 = bytes.read_byte(index);
        for i in 1..len+1 {
            if let Some(item) = bytes.read::<char>(index + i as usize) {
                res.push(item);
            } else {
                return None;
            }
        }
        return Some(res);
    }
    fn size(&self) -> usize {
        self.len() + 1
    }
}

/// a special type that can be used to make strings
/// with longer lenghts as the u16 indicates this has a len
/// that is defined by a u16 instead of a u8 like for a normal string
/// 
pub struct U16String(String);
impl From<String> for U16String {
    fn from(value: String) -> Self {
        U16String(value)
    }
}
impl From<U16String> for String {
    fn from(value: U16String) -> Self {
        value.0
    }
}
impl Serialize for U16String {
    fn serialize(self) -> Bytes {
        let mut bytes = Bytes::new();
        bytes.push(self.0.len() as u16);
        for ch in self.0.chars() {
            bytes.push_byte(ch as u8);
        }
        return bytes;
    }
    fn deserialize(bytes: &Bytes, index: usize) -> Option<Self> {
        if !bytes.is_inbound(index+1) {
            return None;
        }
        let mut res = String::new();
        let len: u16 = bytes.read(index)?;
        for i in 2..len+2 {
            if let Some(item) = bytes.read::<char>(index + i as usize) {
                res.push(item);
            } else {
                return None;
            }
        }
        return Some(U16String(res));
    }
    fn size(&self) -> usize {
        self.0.len() + 2
    }
}

