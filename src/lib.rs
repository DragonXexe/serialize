#[cfg(test)]
mod test;
#[allow(unused_imports)]
#[macro_use]
pub extern crate serialize_derive;

pub use serialize_derive::Serialize;
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

impl Serialize for bool {
    fn serialize(self) -> Bytes {
        Bytes(vec![self as u8], 0)
    }
    fn deserialize(bytes: &Bytes, index: usize) -> Option<Self> {
        if bytes.is_inbound(index) {
            return Some(bytes.read_byte(index) != 0);
        } else {
            return None;
        }
    }
    fn size(&self) -> usize {
        1
    }
}
impl Serialize for u8 {
    fn serialize(self) -> Bytes {
        Bytes(vec![self], 0)
    }

    fn deserialize(bytes: &Bytes, index: usize) -> Option<Self> {
        if bytes.is_inbound(index) {
            return Some(bytes.read_byte(index));
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
            return Some(bytes.read_byte(index) as char);
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
        return Some(res);
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
        return Some(res);
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
        return Some(res);
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
        if !bytes.is_inbound(index + 1) {
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
        for i in 1..len + 1 {
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

impl<T: Serialize> Serialize for Option<T> {
    fn serialize(self) -> Bytes {
        if self.is_none() {
            let mut bytes = Bytes::new();
            bytes.push_byte(0);
            return bytes;
        } else {
            let mut bytes = Bytes::new();
            bytes.push(self.unwrap());
            return bytes;
        }
    }
    fn deserialize(bytes: &Bytes, index: usize) -> Option<Self> {
        let is_some: bool;
        if let Some(discrimenent) = bytes.read::<u8>(index) {
            if discrimenent == 0 {
                is_some = false;
            } else if discrimenent == 1 {
                is_some = true;
            } else {
                return None;
            }
        } else {
            return None;
        }
        if !is_some {
            return Some(None);
        }
        if let Some(item) = bytes.read(index+1) {
            return Some(Some(item));
        } else {
            return None;
        }
    }
    fn size(&self) -> usize {
        1 + match self {
            Some(item) => item.size(),
            None => 0,
        }
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
        if !bytes.is_inbound(index + 1) {
            return None;
        }
        let mut res = String::new();
        let len: u16 = bytes.read(index)?;
        for i in 2..len + 2 {
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


// implemations for floats
impl Serialize for f32 {
    fn serialize(self) -> Bytes {
        use std::mem::transmute;
        let mut bytes = Bytes::new();
        for byte in unsafe { transmute::<f32, [u8; 4]>(self) } { // this unsafe is oke because it correctly gets the bytes
            bytes.push_byte(byte);
        }
        return bytes;
    }

    fn deserialize(bytes: &Bytes, index: usize) -> Option<Self> {
        use std::mem::transmute;
        if !bytes.is_inbound(index + 3) {
            return None;
        }
        let mut slice: [u8; 4] = [0; 4];
        slice[0] = bytes.read_byte(index + 0);
        slice[1] = bytes.read_byte(index + 1);
        slice[2] = bytes.read_byte(index + 2);
        slice[3] = bytes.read_byte(index + 3);
        return Some(unsafe { transmute::<[u8; 4], f32>(slice) }) // again safe for the same reasons
    }

    fn size(&self) -> usize {
        4
    }
}
impl Serialize for f64 {
    fn serialize(self) -> Bytes {
        use std::mem::transmute;
        let mut bytes = Bytes::new();
        for byte in unsafe { transmute::<f64, [u8; 8]>(self) } { // this unsafe is oke because it correctly gets the bytes
            bytes.push_byte(byte);
        }
        return bytes;
    }

    fn deserialize(bytes: &Bytes, index: usize) -> Option<Self> {
        use std::mem::transmute;
        if !bytes.is_inbound(index + 3) {
            return None;
        }
        let mut slice: [u8; 8] = [0; 8];
        slice[0] = bytes.read_byte(index + 0);
        slice[1] = bytes.read_byte(index + 1);
        slice[2] = bytes.read_byte(index + 2);
        slice[3] = bytes.read_byte(index + 3);
        slice[4] = bytes.read_byte(index + 4);
        slice[5] = bytes.read_byte(index + 5);
        slice[6] = bytes.read_byte(index + 6);
        slice[7] = bytes.read_byte(index + 7);
        return Some(unsafe { transmute::<[u8; 8], f64>(slice) }) // again safe for the same reasons
    }

    fn size(&self) -> usize {
        8
    }
}


macro_rules! impl_serialize_copy_for_array {(
    $($N:literal)*
) => (
    $(
        impl<T : Serialize + Default + Copy> Serialize for [T; $N] {
            fn serialize(self) -> Bytes {
                let mut bytes = Bytes::new();
                for item in self {
                    bytes.push(item);
                }
                return bytes;
            }
        
            fn deserialize(bytes: &Bytes, mut index: usize) -> Option<Self> {
                if !bytes.is_inbound(index + 3) {
                    return None;
                }
                let mut res: [T; $N] = [T::default(); $N];
                for i in 0..$N {
                    if let Some(item) = bytes.read::<T>(index) {
                        index += item.size();
                        res[i] = item;
                    }
                }
                return Some(res);
            }
        
            fn size(&self) -> usize {
                let mut res = 0;
                for item in self {
                    res += item.size();
                }
                return res;
            }
        
        }
    )*
)}
impl_serialize_copy_for_array!(
 0x00 0x01 0x02 0x03 0x04 0x05 0x06 0x07 0x08 0x09 0x0A 0x0B 0x0C 0x0D 0x0E 0x0F
 0x10 0x11 0x12 0x13 0x14 0x15 0x16 0x17 0x18 0x19 0x1A 0x1B 0x1C 0x1D 0x1E 0x1F
 0x20 0x21 0x22 0x23 0x24 0x25 0x26 0x27 0x28 0x29 0x2A 0x2B 0x2C 0x2D 0x2E 0x2F
 0x30 0x31 0x32 0x33 0x34 0x35 0x36 0x37 0x38 0x39 0x3A 0x3B 0x3C 0x3D 0x3E 0x3F
 0x40 0x41 0x42 0x43 0x44 0x45 0x46 0x47 0x48 0x49 0x4A 0x4B 0x4C 0x4D 0x4E 0x4F
 0x50 0x51 0x52 0x53 0x54 0x55 0x56 0x57 0x58 0x59 0x5A 0x5B 0x5C 0x5D 0x5E 0x5F
 0x60 0x61 0x62 0x63 0x64 0x65 0x66 0x67 0x68 0x69 0x6A 0x6B 0x6C 0x6D 0x6E 0x6F
 0x70 0x71 0x72 0x73 0x74 0x75 0x76 0x77 0x78 0x79 0x7A 0x7B 0x7C 0x7D 0x7E 0x7F
 0x80 0x81 0x82 0x83 0x84 0x85 0x86 0x87 0x88 0x89 0x8A 0x8B 0x8C 0x8D 0x8E 0x8F
 0x90 0x91 0x92 0x93 0x94 0x95 0x96 0x97 0x98 0x99 0x9A 0x9B 0x9C 0x9D 0x9E 0x9F
 0xA0 0xA1 0xA2 0xA3 0xA4 0xA5 0xA6 0xA7 0xA8 0xA9 0xAA 0xAB 0xAC 0xAD 0xAE 0xAF
 0xB0 0xB1 0xB2 0xB3 0xB4 0xB5 0xB6 0xB7 0xB8 0xB9 0xBA 0xBB 0xBC 0xBD 0xBE 0xBF
 0xC0 0xC1 0xC2 0xC3 0xC4 0xC5 0xC6 0xC7 0xC8 0xC9 0xCA 0xCB 0xCC 0xCD 0xCE 0xCF
 0xD0 0xD1 0xD2 0xD3 0xD4 0xD5 0xD6 0xD7 0xD8 0xD9 0xDA 0xDB 0xDC 0xDD 0xDE 0xDF
 0xE0 0xE1 0xE2 0xE3 0xE4 0xE5 0xE6 0xE7 0xE8 0xE9 0xEA 0xEB 0xEC 0xED 0xEE 0xEF
 0xF0 0xF1 0xF2 0xF3 0xF4 0xF5 0xF6 0xF7 0xF8 0xF9 0xFA 0xFB 0xFC 0xFD 0xFE 0xFF
 0b0000000100000000 0b0000001000000000 0b0000010000000000 0b0000100000000000
 0b0001000000000000 0b0010000000000000 0b0100000000000000 0b1000000000000000
);


impl Serialize for () {
    fn serialize(self) -> Bytes {
        Bytes::new()
    }

    fn deserialize(_bytes: &Bytes, _index: usize) -> Option<Self> {
        Some(())
    }

    fn size(&self) -> usize {
        0
    }
}



macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        #[allow(non_snake_case)]
        impl<$($name: Serialize),+> Serialize for ($($name,)+)
        {
            fn serialize(self) -> Bytes {
                let ($($name,)+) = self;
                let mut bytes = Bytes::new();
                $(bytes.append(&$name.serialize());)*
                return bytes;
            }
        
            #[allow(unused_assignments)]
            fn deserialize(bytes: &Bytes, mut index: usize) -> Option<Self> {
                let ($($name,)+): ($($name,)+);
                $(if let Some(field) = bytes.read::<$name>(index) {
                    index += field.size();
                    $name = field;
                } else {
                    return None;
                })*
                return Some(($($name,)+));
            }
        
            fn size(&self) -> usize {
                let ($($name,)+) = self;
                $($name.size() + )* 0
            }
        }
    };
}

tuple_impls! { A }
tuple_impls! { A B }
tuple_impls! { A B C }
tuple_impls! { A B C D }
tuple_impls! { A B C D E }
tuple_impls! { A B C D E F }
tuple_impls! { A B C D E F G }
tuple_impls! { A B C D E F G H }
tuple_impls! { A B C D E F G H I }
tuple_impls! { A B C D E F G H I J }
tuple_impls! { A B C D E F G H I J K }
tuple_impls! { A B C D E F G H I J K L }
tuple_impls! { A B C D E F G H I J K L M }
tuple_impls! { A B C D E F G H I J K L M N }
tuple_impls! { A B C D E F G H I J K L M N O }
tuple_impls! { A B C D E F G H I J K L M N O P }
tuple_impls! { A B C D E F G H I J K L M N O P Q }
tuple_impls! { A B C D E F G H I J K L M N O P Q R }
tuple_impls! { A B C D E F G H I J K L M N O P Q R S }
tuple_impls! { A B C D E F G H I J K L M N O P Q R S T }
tuple_impls! { A B C D E F G H I J K L M N O P Q R S T U }
tuple_impls! { A B C D E F G H I J K L M N O P Q R S T U V }
tuple_impls! { A B C D E F G H I J K L M N O P Q R S T U V W }
tuple_impls! { A B C D E F G H I J K L M N O P Q R S T U V W X }
tuple_impls! { A B C D E F G H I J K L M N O P Q R S T U V W X Y }
tuple_impls! { A B C D E F G H I J K L M N O P Q R S T U V W X Y Z }




