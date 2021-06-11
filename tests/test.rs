use variants_struct::VariantsStruct;

#[derive(VariantsStruct)]
#[struct_derive(Copy, Clone, Default, PartialEq, Debug)]
#[struct_bounds(Clone)]
pub enum Hello {
    World,
    There
}

#[derive(VariantsStruct, Clone, PartialEq, Debug)]
enum HasTuples {
    Zero,
    One(&'static str),
}

pub struct NotClonable;

#[test]
fn store() {
    let hello = HelloStruct {
        world: 5,
        there: 3
    };
    assert_eq!(hello.world, 5);
    assert_eq!(*hello.get_unchecked(&Hello::World), 5);
}

#[test]
fn modify() {
    let mut hello = HelloStruct::new(3, 5);
    hello.there = 7;
    assert_eq!(hello.there, 7);
    *hello.get_mut_unchecked(&Hello::There) = 10;
    assert_eq!(hello.there, 10);
}

#[test]
fn tuple_variant() {
    let mut tuple_boi = HasTuplesStruct::new(3);
    tuple_boi.one.insert("hello there", 2);
    assert_eq!(*tuple_boi.get_unchecked(&HasTuples::One("hello there")), 2);
    assert_eq!(tuple_boi.one.get("hello there"), Some(&2));
    assert_eq!(tuple_boi.one.get("asdf"), None);
}

#[test]
fn default() {
    let hello: HelloStruct<u32> = Default::default();
    let manual = HelloStruct {
        world: 0,
        there: 0
    };
    assert_eq!(hello.world, manual.world);
    assert_eq!(hello.there, manual.there);
}

#[test]
fn bounds_and_derive() {
    let hello = HelloStruct::new(HasTuples::Zero, HasTuples::One("hello"));
    let clone = hello.clone();
    assert_eq!(hello, clone);

    // uncommenting this should be an error (violates clone bound);
    // let fail = HelloStruct::new(NotClonable, NotClonable);
}

#[derive(VariantsStruct)]
#[struct_name = "SomeOtherName"]
enum NotThisName {
    Hi
}

#[test]
fn renaming() {
    let hello = SomeOtherName {
        hi: 5
    };
    assert_eq!(hello.hi, 5);
    assert_eq!(*hello.get_unchecked(&NotThisName::Hi), 5);
}

// Testing with serde

use serde::{Deserialize, Serialize};

#[derive(VariantsStruct)]
#[struct_derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Asdf {
    Zxcv,
    Qwer
}

#[test]
fn test_serde() {
    let start = AsdfStruct::new(2, 3);

    let string = serde_json::to_string(&start).unwrap();
    assert_eq!(string, r#"{"zxcv":2,"qwer":3}"#);

    let end: AsdfStruct<i32> = serde_json::from_str(&string).unwrap();
    assert_eq!(start, end);
}