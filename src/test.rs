use crate::Serialize;


#[test]
fn string_test() {
    let string = "Hello, World!!!".to_string();
    let bytes = string.clone().serialize();
    let found_string = String::deserialize(&bytes, 0).unwrap();
    assert_eq!(string, found_string);
}
use serialize_derive::Serialize;
use crate as serialr;
#[derive(Serialize, Clone, Debug, Copy, PartialEq)]
pub struct Test {
    x: u64,
    y: u64,
}
#[test]
fn derive_test() {
    let string = Test {x: 345, y: 564746324436543543};
    let bytes = string.serialize();
    let found_string = Test::deserialize(&bytes, 0).unwrap();
    assert_eq!(string, found_string);
}