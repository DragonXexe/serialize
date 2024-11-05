
use std::{any::Any, fmt::Debug};

use crate::{Bytes, Deserialize, SerialRead, Serialize};

#[test]
fn string_test() {
    let mut bytes = Bytes::new();

    let string = "Hello, World!!!".to_string();
    string.clone().serialize(&mut bytes).unwrap();
    let found_string = String::deserialize(&mut bytes).unwrap();
    assert_eq!(string, found_string);
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq)]
pub struct Test(u64, u64);
#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq)]
pub struct Test1 {
    x: u64,
    y: u64,
}

#[derive(Clone, Deserialize, Default, Serialize)]
pub struct Dict<K, V> {
    items: Vec<(K, V)>,
}

#[test]
fn derive_test() {
    let mut bytes = Bytes::new();
    let string = Test(345, 564746324436543543);
    string.serialize(&mut bytes).unwrap();
    let found_string = Test::deserialize(&mut bytes).unwrap();
    assert_eq!(string, found_string);
}
#[test]
fn enum_derive_test() {
    let mut bytes = Bytes::new();
    let enum1 = TestEnum1::ONE("string".to_string());
    enum1.clone().serialize(&mut bytes).unwrap();
    let found_enum = TestEnum1::deserialize(&mut bytes).unwrap();
    assert_eq!(enum1, found_enum);
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TestEnum {
    Variant0(u64, String),
    Variant1(usize, String),
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TestEnum1 {
    ONE(String),
    TWO,
    THREE((usize, f64, isize, f32)),
    FOUR,
}
#[test]
fn hash_map_test() {
    use std::collections::HashMap;
    let mut bytes = Bytes::new();
    let mut hashmap: std::collections::HashMap<String, i32> = HashMap::new();
    hashmap.insert("Hello".to_string(), 103);
    hashmap.insert("Three".to_string(), 3);
    hashmap.insert("Hey".to_string(), 4);
    hashmap.serialize(&mut bytes).unwrap();
    let hashmap = <HashMap<String, i32>>::deserialize(&mut bytes).unwrap();
    assert_eq!(hashmap.get(&"Hello".to_string()), Some(&103));
    assert_eq!(hashmap.get(&"Three".to_string()), Some(&3));
    assert_eq!(hashmap.get(&"Hey".to_string()), Some(&4));
}
#[test]
fn tuple_test() {
    let mut bytes = Bytes::new();
    let tuple: (u8, String, u8, u8, u8, u8) = (34, "test".to_string(), 34, 34, 34, 34);
    tuple.clone().serialize(&mut bytes).unwrap();
    assert_eq!(bytes.read_serialized::<(u8, String, u8, u8, u8, u8)>().unwrap(), tuple);
}


#[test]
fn float_test() {
    let mut buffer = Bytes::new();
    
    let float: f32 = -1.0 / 0.0;
    float.serialize(&mut buffer).unwrap();
    assert_eq!(float, f32::deserialize(&mut buffer).unwrap());
}
#[test]
fn array_test() {
    let mut buffer = Bytes::new();
    let array: [u8; 16] = [12; 16];
    array.serialize(&mut buffer).unwrap();
    assert_eq!(array, <[u8; 16]>::deserialize(&mut buffer).unwrap())
}


#[test]
fn usize_test() {
    let num: usize = 342453646;
    
    // Create a buffer with sufficient capacity for the number of bytes
    let mut buffer = Bytes::new();
    
    // Serialize the number into the buffer
    num.serialize(&mut buffer).unwrap();
    
    // Deserialize the number from the buffer
    let deserialized_num = usize::deserialize(&mut buffer).unwrap();
    
    // Assert that the original and deserialized values are the same
    assert_eq!(num, deserialized_num);
}
#[test]
fn vec_test() {
    let num: Vec<usize> = vec![1, 2, 3, 4, 5];
    
    // Create a buffer with sufficient capacity for the number of bytes
    let mut buffer = Bytes::new();
    
    // Serialize the number into the buffer
    num.clone().serialize(&mut buffer).unwrap();
    
    // Deserialize the number from the buffer
    let deserialized_num = <Vec<usize>>::deserialize(&mut buffer).unwrap();
    
    // Assert that the original and deserialized values are the same
    assert_eq!(num, deserialized_num);
}
#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum CustomOption<T> {
    Some(T),
    None,
}

#[test]
fn generics_test() {
    let mut bytes = Bytes::new();
    let option: CustomOption<String> = CustomOption::Some("Test1".to_string());
    option.clone().serialize(&mut bytes).unwrap();
    // dbg!(&bytes);
    let new_option = bytes.read_serialized::<CustomOption<String>>();
    // dbg!(&bytes);
    assert_eq!(option, new_option.unwrap());
}
#[test]
fn invalid_test() {
    let mut bytes = Bytes::new();
    0x33445566u32.serialize(&mut bytes).unwrap();
    assert!(bytes.read_serialized::<usize>().is_err());
}
#[derive(Serialize, Deserialize)]
pub struct Vec2<N: Any> {
    pub x: N,
    pub y: N,
}
