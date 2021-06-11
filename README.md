# variants-struct

A derive macro to convert enums into a struct where the variants are members.
Effectively, its like using a `HashMap<MyEnum, MyData>`, but it generates a hard-coded struct instead
of a HashMap to reduce overhead.

## Basic Example

Applying the macro to a basic enum (i.e. one without tuple variants or struct variants) like this:

```rust
use variants_struct::VariantsStruct;

#[derive(VariantsStruct)]
enum Hello {
    World,
    There
}
```

would produce the following code:

```rust
struct HelloStruct<T> {
    pub world: T,
    pub there: T
}

impl<T> HelloStruct<T> {
    pub fn new(world: T, there: T) -> HelloStruct<T> {
        HelloStruct {
            world,
            there
        }
    }

    pub fn get_unchecked(&self, var: &Hello) -> &T {
        match var {
            &Hello::World => &self.world,
            &Hello::There => &self.there
        }
    }

    pub fn get_mut_unchecked(&mut self, var: &Hello) -> &mut T {
        match var {
            &Hello::World => &mut self.world,
            &Hello::There => &mut self.there
        }
    }

    pub fn get(&self, var: &Hello) -> Option<&T> {
        match var {
            &Hello::World => Some(&self.world),
            &Hello::There => Some(&self.there)
        }
    }

    pub fn get_mut(&mut self, var: &Hello) -> Option<&mut T> {
        match var {
            &Hello::World => Some(&mut self.world),
            &Hello::There => Some(&mut self.there)
        }
    }
}
```

The members can be accessed either directly (like `hello.world`) or by using the getter methods, like:

```rust
fn main() {
    let mut hello = HelloStruct::new(2, 3);
    *hello.get_mut_unchecked(&Hello::World) = 5;

    assert_eq!(hello.world, 5);
    assert_eq!(hello.world, *hello.get_unchecked(&Hello::World));
}
```

The getters can be particularly useful with the [enum-iterator](https://docs.rs/crate/enum-iterator/) crate. For basic enums,
the checked-getters will always return `Some(...)`, so using `get_unchecked` is recommended, *but this is not the case when the enum contains tuple variants*.

Keep in mind that the enum variants are renamed from CamelCase to snake_case, to be consistent with Rust's naming conventions.

## Visibility

The struct fields are always `pub`, and the struct shares the same visibility as the enum.

## Customizing the struct

### Renaming

By default, the struct's name is `<OriginalEnumName>Struct`. You can set it to something else with the `struct_name` attribute. For example, this:

```rust
#[derive(VariantsStruct)]
#[struct_name = "SomeOtherName"]
pub enum NotThisName {
    Variant
}
```

will produce a struct with name `SomeOtherName`.

### Derives

By default no derives are applied to the generated struct. You can add derive macro invocations with the `struct_derive` attribute. For example, this:

```rust
use serde::{Serialize, Deserialize};

#[derive(VariantsStruct)]
#[struct_derive(Debug, Default, Serialize, Deserialize)]
enum Hello {
    World,
    There
}
```

would produce the following code:

```rust
#[derive(Debug, Default, Serialize, Deserialize)]
struct HelloStruct<T> {
    pub world: T,
    pub there: T
}

// impl block omitted
```

### Trait Bounds

By default the struct's type argument `T` has no trait bounds, but you can add them with the `struct_bounds` attribute. For example, this:

```rust
#[derive(VariantsStruct)]
#[struct_bounds(Clone)]
enum Hello {
    World,
    There
}
```

would produce the following code:

```rust
struct HelloStruct<T: Clone> {
    # go_away: T,
    // fields omitted
}

impl<T: Clone> HelloStruct<T> {
    // methods omitted
}
```

### Combinations

Note that many derives don't require that the type argument `T` fulfills any trait bounds. For example, applying the `Clone`
derive to the struct only makes the struct cloneable if `T` is cloneable, and still allows un-cloneable types to be used with the struct.

So if you want the struct to *always* be cloneable, you have to use both the derive and the trait bound:

```rust
#[derive(VariantsStruct)]
#[struct_derive(Clone)]
#[struct_bounds(Clone)]
enum Hello {
    // variants omitted
}
```

These three attributes can be used in any order, or even multiple times (although that wouldn't be very readable).

## Tuple Variants

Tuple variants are turned into a `HashMap`, where the data stored in the tuple is the key (so the data must implement `Hash`).
Unfortunately, variants with more than one value in them are not supported.

Tuple variants are omitted from the struct's `new` function. For example, this:

```rust
#[derive(VariantsStruct)]
enum Hello {
    World,
    There(i32)
}
```

produces the following code:

```rust
struct HelloStruct<T> {
    pub world: T,
    pub there: std::collections::HashMap<i32, T>
}

impl<T> HelloStruct<T> {
    fn new(world: T) -> HelloStruct<T> {
        HelloStruct {
            world,
            there: std::collections::HashMap::new()
        }
    }

    pub fn get_unchecked(&self, var: &Hello) -> &T {
        match var {
            &Hello::World => &self.world,
            &Hello::There(key) => self.there.get(&key)
                .expect("tuple variant key not found in hashmap")
        }
    }

    pub fn get_mut_unchecked(&mut self, var: &Hello) -> &mut T {
        match var {
            &Hello::World => &mut self.world,
            &Hello::There(key) => self.there.get_mut(&key)
                .expect("tuple variant key not found in hashmap")
        }
    }

    pub fn get(&self, var: &Hello) -> Option<&T> {
        match var {
            &Hello::World => Some(&self.world),
            &Hello::There(key) => self.there.get(&key)
        }
    }

    pub fn get_mut(&mut self, var: &Hello) -> Option<&mut T> {
        match var {
            &Hello::World => Some(&mut self.world),
            &Hello::There(key) => self.there.get_mut(&key)
        }
    }
}
```

Notice that the `new` function now only takes the `world` argument, and the unchecked getter methods query the hashmap and unwrap the result.
