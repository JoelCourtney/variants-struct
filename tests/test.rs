use variants_struct::VariantsStruct;

#[derive(VariantsStruct)]
#[struct_derive(Copy, Clone, Default, PartialEq, Debug)]
#[struct_bounds(Clone)]
pub enum Hello {
    World,
    There,
}

#[derive(VariantsStruct, Clone, PartialEq, Debug)]
enum HasTuples {
    Zero,
    One(&'static str),
    OtherOne(i32),
    StructVariant { my_field: i32 },
}

pub struct NotClonable;

#[test]
fn store() {
    let hello = HelloStruct { world: 5, there: 3 };
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
fn hashmaps() {
    let mut tuple_boi = HasTuplesStruct::new(3);
    tuple_boi.one.insert("hello there", 2);
    assert_eq!(*tuple_boi.get_unchecked(&HasTuples::One("hello there")), 2);
    assert_eq!(tuple_boi.one.get("hello there"), Some(&2));
    assert_eq!(tuple_boi.one.get("asdf"), None);

    tuple_boi.other_one.insert(7, 70);
    assert_eq!(*tuple_boi.get_unchecked(&HasTuples::OtherOne(7)), 70);

    tuple_boi.struct_variant.insert(8, 80);
    assert_eq!(
        *tuple_boi.get_unchecked(&HasTuples::StructVariant { my_field: 8 }),
        80
    );
}

#[test]
fn default() {
    let hello: HelloStruct<u32> = Default::default();
    let manual = HelloStruct { world: 0, there: 0 };
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

// Renaming

#[derive(VariantsStruct)]
#[struct_name = "SomeOtherName"]
#[allow(dead_code)]
enum NotThisName {
    Struct,
    Fn,
    Async,
    #[field_name = "this_instead"]
    NotThis,
}

#[test]
fn renaming() {
    let hello = SomeOtherName {
        r#struct: 5,
        r#fn: 3,
        r#async: 2,
        this_instead: 1,
    };
    assert_eq!(hello.r#struct, 5);
    assert_eq!(hello.this_instead, 1);
    assert_eq!(*hello.get_unchecked(&NotThisName::NotThis), 1);
}

// Testing with serde

use serde::{Deserialize, Serialize};

#[derive(VariantsStruct)]
#[struct_derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Asdf {
    Zxcv,
    Qwer,
}

#[test]
fn test_serde() {
    let start = AsdfStruct::new(2, 3);

    let string = serde_json::to_string(&start).unwrap();
    assert_eq!(string, r#"{"zxcv":2,"qwer":3}"#);

    let end: AsdfStruct<i32> = serde_json::from_str(&string).unwrap();
    assert_eq!(start, end);
}
