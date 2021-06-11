use variants_struct::VariantsStruct;

#[derive(VariantsStruct)]
pub enum Hello {
    World,
    There
}

#[derive(VariantsStruct)]
enum HasTuples {
    #[allow(dead_code)]
    Zero,
    One(&'static str),
    Two(&'static str, &'static str),
    TwoBool(bool, bool)
}

#[test]
fn store() {
    let hello = HelloStruct {
        world: 5,
        there: 3
    };
    assert_eq!(hello.world, 5);
    assert_eq!(*hello.get(Hello::World), 5);
}

#[test]
fn modify() {
    let mut hello = HelloStruct::new(3, 5);
    hello.there = 7;
    assert_eq!(hello.there, 7);
    *hello.get_mut(Hello::There) = 10;
    assert_eq!(hello.there, 10);
}

#[test]
fn tuple_variant() {
    let mut tuple_boi = HasTuplesStruct::new(3);
    tuple_boi.one.insert("hello there", 2);
    assert_eq!(*tuple_boi.get(HasTuples::One("hello there")), 2);
    assert_eq!(tuple_boi.one.get("hello there"), Some(&2));
    assert_eq!(tuple_boi.one.get("asdf"), None);

    tuple_boi.two.insert(("hello", "there"), 5);
    assert_eq!(*tuple_boi.get(HasTuples::Two("hello", "there")), 5);
    assert_eq!(tuple_boi.two.get(&("hello", "there")), Some(&5));
    assert_eq!(tuple_boi.two.get(&("asdf", "")), None);

    tuple_boi.two_bool.insert((true, false), 10);
    assert_eq!(*tuple_boi.get(HasTuples::TwoBool(true, false)), 10);
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