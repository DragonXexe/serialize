use crate::{Bytes, Serialize};

#[test]
fn string_test() {
    let string = "Hello, World!!!".to_string();
    let bytes = string.clone().serialize();
    let found_string = String::deserialize(&bytes, 0).unwrap();
    assert_eq!(string, found_string);
}

use crate as serialr;
#[derive(Serialize, Clone, Debug, Copy, PartialEq)]
pub struct Test(u64, u64);
#[derive(Serialize, Clone, Debug, Copy, PartialEq)]
pub struct Test1 {
    x: u64,
    y: u64,
}

#[derive(Clone, Default, Serialize)]
pub struct Dict<K, V> {
    items: Vec<(K, V)>,
}

#[test]
fn derive_test() {
    let string = Test(345, 564746324436543543);
    let bytes = string.serialize();
    let found_string = Test::deserialize(&bytes, 0).unwrap();
    assert_eq!(string, found_string);
}
#[test]
fn enum_derive_test() {
    let enum1 = TestEnum1::ONE("string".to_string());
    let bytes = enum1.clone().serialize();
    let found_enum = TestEnum1::deserialize(&bytes, 0).unwrap();
    assert_eq!(enum1, found_enum);
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum TestEnum {
    Variant0(u64, String),
    Variant1(usize, String),
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum TestEnum1 {
    ONE(String),
    TWO,
    THREE((usize, f64, isize, f32)),
    FOUR,
}
#[derive(Serialize, PartialEq, Clone, Copy, Debug)]
pub enum CustomOption<T> {
    Some(T),
    None,
}
// #[derive(Serialize, PartialEq, Clone, Debug)]
// pub struct Dict<K, V> {
//     keys: Vec<K>,
//     vals: Vec<V>,
// }

#[test]
fn tuple_test() {
    let tuple: (u8, String, u8, u8, u8, u8) = (34, "test".to_string(), 34, 34, 34, 34);
    let bytes = tuple.clone().serialize();
    assert_eq!(tuple, bytes.read(0).unwrap());
}

#[test]
fn float_test() {
    let float: f32 = -1.0 / 0.0;
    let bytes = float.serialize();
    assert_eq!(float, bytes.read(0).unwrap())
}
#[test]
fn array_test() {
    let array: [u8; 16] = [12; 16];
    let bytes = array.serialize();
    assert_eq!(array, bytes.read::<[u8; 16]>(0).unwrap())
}
#[test]
fn usize_test() {
    let num: usize = 342453646;
    let bytes = num.serialize();
    assert_eq!(num, bytes.read(0).unwrap());
}
#[test]
fn genarics_test() {
    let option: CustomOption<String> = CustomOption::Some("Test1".to_string());
    let bytes = option.clone().serialize();
    assert_eq!(option, bytes.read(0).unwrap());
}
#[test]
fn invalid_test() {
    let invalid_bytes: Bytes = 0x33445566u32.serialize();
    assert_eq!(invalid_bytes.read::<String>(0), None);
}
