
use std::{collections::HashMap, error::Error, vec};


#[cfg(test)]
mod test;
#[allow(unused_imports)]
#[macro_use]
pub extern crate serialize_derive;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bytes {
    data: Vec<u8>,
    current: usize,
}
impl Bytes {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            current: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn reset(&mut self) {
        self.current = 0;
    }
    pub fn push(&mut self, byte: u8) {
        self.data.push(byte)
    }
}
impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Self {
            data: value,
            current: 0,
        }
    }
}
impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        Self {
            data: value.to_vec(),
            current: 0,
        }
    }
}
impl Into<Vec<u8>> for Bytes {
    fn into(self) -> Vec<u8> {
        self.data
    }
}
impl std::io::Write for Bytes {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for byte in buf {
            self.data.push(*byte);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
impl std::io::Read for Bytes {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        for i in 0..buf.len() {
            let byte= match self.data.get(self.current) {
                Some(byte) => *byte,
                None => return Ok(i),
            };
            buf[i] = byte;
            self.current += 1;
        }
        Ok(buf.len())
    }
}
#[derive(Debug)]
pub enum SerializeError {
    IOError(std::io::Error),
    InvalidData,
}
impl std::fmt::Display for SerializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerializeError::IOError(error) => write!(f, "{}", error),
            SerializeError::InvalidData => write!(f, "got invalid data while deserializing"),
        }
    }
}
impl Error for SerializeError {}
impl From<std::io::Error> for SerializeError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}


pub trait Serialize: 'static {
    fn serialize<B: std::io::Write>(self, buf: &mut B) -> Result<(), SerializeError>;
}

pub trait Deserialize: Sized + 'static {
    fn deserialize<B: std::io::Read>(buf: &mut B) -> Result<Self, SerializeError>;
}


pub trait SerialWrite: std::io::Write {
    fn write_serialized<S: Serialize>(&mut self, val: S) -> Result<(), SerializeError>;
}
impl<B: std::io::Write> SerialWrite for B {
    fn write_serialized<S: Serialize>(&mut self, val: S) -> Result<(), SerializeError> {
        val.serialize(self)
    }
}
pub trait SerialRead: std::io::Read {
    fn read_serialized<S: Deserialize>(&mut self) -> Result<S, SerializeError>;
}
impl<B: std::io::Read> SerialRead for B {
    fn read_serialized<S: Deserialize>(&mut self) -> Result<S, SerializeError> {
        S::deserialize(self)
    }
}

macro_rules! impl_num {
    ($num:ty) => {
        impl Serialize for $num {
            fn serialize<B: std::io::Write>(self, buf: &mut B) -> Result<(), SerializeError>{
                let bytes = self.to_be_bytes();
                buf.write_all(&bytes)?;
                Ok(())
            }
        }
        impl Deserialize for $num {
            fn deserialize<B: std::io::Read>(buf: &mut B) -> Result<Self, SerializeError> {
                let bytes = &mut [0; size_of::<Self>()];
                buf.read_exact(bytes)?;
                Ok(Self::from_be_bytes(*bytes))
            }
        }
    };
    ($($num:ty),*) => {
        $(
            impl_num!($num);
        )*
    }
}
impl_num!(
    u8, u16, u32, u64, usize,
    i8, i16, i32, i64, isize,
    f32, f64
);

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize<B: std::io::Write>(self, buf: &mut B) -> Result<(), SerializeError> {
        buf.write_serialized(self.len())?;
        for item in self {
            buf.write_serialized(item)?;
        }
        Ok(())
    }
}
impl<T: Deserialize> Deserialize for Vec<T> {
    fn deserialize<B: std::io::Read>(buf: &mut B) -> Result<Self, SerializeError> {
        let mut this = vec![];
        let len = buf.read_serialized::<usize>()?;
        for _ in 0..len {
            this.push(buf.read_serialized::<T>()?);
        }
        Ok(this)
    }
}
impl Serialize for String {
    fn serialize<B: std::io::Write>(self, buf: &mut B) -> Result<(), SerializeError> {
        buf.write_serialized(self.len())?;
        for item in self.bytes() {
            buf.write_serialized(item)?;
        }
        Ok(())
    }
}
impl Deserialize for String {
    fn deserialize<B: std::io::Read>(buf: &mut B) -> Result<Self, SerializeError> {
        let len = buf.read_serialized::<usize>()?;
        let mut bytes = vec![0; len];
        buf.read_exact(&mut bytes)?;
        match String::from_utf8(bytes) {
            Ok(string) => Ok(string),
            Err(_) => Err(SerializeError::InvalidData),
        }
    }
}
impl<T: Serialize> Serialize for Box<T> {
    fn serialize<B: std::io::Write>(self, buf: &mut B) -> Result<(), SerializeError> {
        buf.write_serialized(*self)
    }
}
impl<T: Deserialize> Deserialize for Box<T> {
    fn deserialize<B: std::io::Read>(buf: &mut B) -> Result<Self, SerializeError> {
        let inner = buf.read_serialized::<T>()?;
        Ok(Self::new(inner))
    }
}
impl<T: Serialize> Serialize for Option<T> {
    fn serialize<B: std::io::Write>(
        self,
        buf: &mut B,
    ) -> Result<(), crate::SerializeError> {
        use crate::SerialWrite;
        let discriminant = { unsafe { *((&self) as *const _ as *const u64) } };
        buf.write_serialized(discriminant)?;
        match self {
            Self::Some(f0) => {
                buf.write_serialized(f0)?;
            }
            Self::None => {}
        }
        Ok(())
    }
}
impl<T: Deserialize> Deserialize for Option<T> {
    fn deserialize<B: std::io::Read>(
        buf: &mut B,
    ) -> Result<Self, crate::SerializeError> {
        use crate::SerialRead;
        let discriminant: u64 = buf.read_serialized::<u64>()?;
        match discriminant {
            0u64 => {
                let f0 = buf.read_serialized()?;
                return Ok(Self::Some(f0));
            }
            1u64 => Ok(Self::None),
            _ => Err(crate::SerializeError::InvalidData),
        }
    }
}
impl<K: Serialize, V: Serialize> Serialize for HashMap<K, V> {
    fn serialize<B: std::io::Write>(self, buf: &mut B) -> Result<(), SerializeError> {
        let items = self.into_iter();
        buf.write_serialized(items.len())?;
        for item in items {
            buf.write_serialized(item.0)?;
            buf.write_serialized(item.1)?;
        }
        Ok(())
    }
}
impl<K: Deserialize + std::cmp::Eq + std::hash::Hash, V: Deserialize> Deserialize for HashMap<K, V> {
    fn deserialize<B: std::io::Read>(buf: &mut B) -> Result<Self, SerializeError> {
        let len = buf.read_serialized::<usize>()?;
        let mut this = HashMap::new();

        for _ in 0..len {
            let key = buf.read_serialized::<K>()?;
            let val = buf.read_serialized::<V>()?;
            this.insert(key, val);
        }

        return Ok(this)
    }
}

impl<T: Serialize, const N: usize> Serialize for [T; N] {
    fn serialize<B: std::io::Write>(self, buf: &mut B) -> Result<(), SerializeError> {
        for item in self.into_iter() {
            buf.write_serialized(item)?
        }
        Ok(())
    }
}
impl<T: Deserialize, const N: usize> Deserialize for [T; N] {
    fn deserialize<B: std::io::Read>(buf: &mut B) -> Result<Self, SerializeError> {
        use std::mem::MaybeUninit;
        let mut arr: [MaybeUninit<T>; N] = [const { MaybeUninit::zeroed() }; N];

        for i in 0..N {
            arr[i] = MaybeUninit::new(T::deserialize(buf)?);
        }

        // SAFETY: All elements of `arr` have been initialized.
        Ok(unsafe { std::mem::transmute_copy(&arr) })
    }
}

macro_rules! impl_for_tuple {
    ($($name:ident),*) => {
        impl<$($name: Serialize),*> Serialize for ($($name),*) {
            fn serialize<Buf: std::io::Write>(self, buf: &mut Buf) -> Result<(), SerializeError> {
                #[allow(non_snake_case)]
                let ($($name),*) = self;
                $(
                    $name.serialize(buf)?;
                )*
                Ok(())
            }
        }

        impl<$($name: Deserialize),*> Deserialize for ($($name),*) {
            fn deserialize<Buf: std::io::Read>(buf: &mut Buf) -> Result<Self, SerializeError> {
                Ok(($(
                    $name::deserialize(buf)?
                ),*))
            }
        }
    };
}
impl_for_tuple!(A, B);
impl_for_tuple!(A, B, C);
impl_for_tuple!(A, B, C, D);
impl_for_tuple!(A, B, C, D, E);
impl_for_tuple!(A, B, C, D, E, F);
impl_for_tuple!(A, B, C, D, E, F, G);
impl_for_tuple!(A, B, C, D, E, F, G, H);
impl_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
impl_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
impl_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
impl_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
impl_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
impl_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
impl_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
impl_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
impl_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
impl_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

