//! A derive macro to convert enums into a struct where the variants are members.
//! Effectively, its like using a `HashMap<MyEnum, MyData>`, but it generates a hard-coded struct instead
//! of a HashMap to reduce overhead.
//!
//! # Basic Example
//!
//! Applying the macro to a basic enum (i.e. one without tuple variants or struct variants) like this:
//!
//! ```
//! use variants_struct::VariantsStruct;
//!
//! #[derive(VariantsStruct)]
//! enum Hello {
//!     World,
//!     There
//! }
//! ```
//!
//! would produce the following code:
//!
//! ```
//! # enum Hello {
//! #     World,
//! #     There
//! # }
//! struct HelloStruct<T> {
//!     pub world: T,
//!     pub there: T
//! }
//!
//! impl<T> HelloStruct<T> {
//!     pub fn new(world: T, there: T) -> HelloStruct<T> {
//!         HelloStruct {
//!             world,
//!             there
//!         }
//!     }
//!
//!     pub fn get_unchecked(&self, var: &Hello) -> &T {
//!         match var {
//!             &Hello::World => &self.world,
//!             &Hello::There => &self.there
//!         }
//!     }
//!
//!     pub fn get_mut_unchecked(&mut self, var: &Hello) -> &mut T {
//!         match var {
//!             &Hello::World => &mut self.world,
//!             &Hello::There => &mut self.there
//!         }
//!     }
//!
//!     pub fn get(&self, var: &Hello) -> Option<&T> {
//!         match var {
//!             &Hello::World => Some(&self.world),
//!             &Hello::There => Some(&self.there)
//!         }
//!     }
//!
//!     pub fn get_mut(&mut self, var: &Hello) -> Option<&mut T> {
//!         match var {
//!             &Hello::World => Some(&mut self.world),
//!             &Hello::There => Some(&mut self.there)
//!         }
//!     }
//! }
//! ```
//!
//! The members can be accessed either directly (like `hello.world`) or by using the getter methods, like:
//!
//! ```
//! # use variants_struct::VariantsStruct;
//! # #[derive(VariantsStruct)]
//! # enum Hello {
//! #     World,
//! #     There
//! # }
//! let mut hello = HelloStruct::new(2, 3);
//! *hello.get_mut_unchecked(&Hello::World) = 5;
//!
//! assert_eq!(hello.world, 5);
//! assert_eq!(hello.world, *hello.get_unchecked(&Hello::World));
//! ```
//!
//! The getters can be particularly useful with the [enum-iterator](https://docs.rs/crate/enum-iterator/) crate. For basic enums,
//! the checked-getters will always return `Some(...)`, so using `get_unchecked` is recommended, *but this is not the case when the enum contains tuple variants*.
//!
//! Keep in mind that the enum variants are renamed from CamelCase to snake_case, to be consistent with Rust's naming conventions.
//!
//! # Visibility
//!
//! The struct fields are always `pub`, and the struct shares the same visibility as the enum.
//!
//! # Customizing the struct
//!
//! ## Renaming
//!
//! By default, the struct's name is `<OriginalEnumName>Struct`. You can set it to something else with the `struct_name` attribute. For example, this:
//!
//! ```
//! # use variants_struct::VariantsStruct;
//! #[derive(VariantsStruct)]
//! #[struct_name = "SomeOtherName"]
//! enum NotThisName {
//!     Variant
//! }
//! ```
//!
//! will produce a struct with name `SomeOtherName`.
//!
//! You can also rename the individual fields manually with the `field_name` attribute. For example, this:
//!
//! ```
//! # use variants_struct::VariantsStruct;
//! #[derive(VariantsStruct)]
//! enum ChangeMyVariantName {
//!     #[field_name = "this_name"] NotThisName
//! }
//! ```
//!
//! Will produce the following struct:
//!
//! ```
//! struct ChangeMyVariantName<T> {
//!     this_name: T
//! }
//! ```
//!
//! ## Derives
//!
//! By default no derives are applied to the generated struct. You can add derive macro invocations with the `struct_derive` attribute. For example, this:
//!
//! ```
//! # use variants_struct::VariantsStruct;
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(VariantsStruct)]
//! #[struct_derive(Debug, Default, Serialize, Deserialize)]
//! enum Hello {
//!     World,
//!     There
//! }
//! ```
//!
//! would produce the following code:
//!
//! ```
//! # use serde::{Serialize, Deserialize};
//! #[derive(Debug, Default, Serialize, Deserialize)]
//! struct HelloStruct<T> {
//!     pub world: T,
//!     pub there: T
//! }
//!
//! // impl block omitted
//! ```
//!
//! ## Trait Bounds
//!
//! By default the struct's type argument `T` has no trait bounds, but you can add them with the `struct_bounds` attribute. For example, this:
//!
//! ```
//! # use variants_struct::VariantsStruct;
//! #[derive(VariantsStruct)]
//! #[struct_bounds(Clone)]
//! enum Hello {
//!     World,
//!     There
//! }
//! ```
//!
//! would produce the following code:
//!
//! ```
//! struct HelloStruct<T: Clone> {
//!     # go_away: T,
//!     // fields omitted
//! }
//!
//! impl<T: Clone> HelloStruct<T> {
//!     // methods omitted
//! }
//! ```
//!
//! ## Combinations
//!
//! Note that many derives don't require that the type argument `T` fulfills any trait bounds. For example, applying the `Clone`
//! derive to the struct only makes the struct cloneable if `T` is cloneable, and still allows un-cloneable types to be used with the struct.
//!
//! So if you want the struct to *always* be cloneable, you have to use both the derive and the trait bound:
//!
//! ```
//! # use variants_struct::VariantsStruct;
//! #[derive(VariantsStruct)]
//! #[struct_derive(Clone)]
//! #[struct_bounds(Clone)]
//! enum Hello {
//!     // variants omitted
//! }
//! ```
//!
//! These two attributes, and the `struct_name` attribute, can be used in any order, or even multiple times (although that wouldn't be very readable).
//!
//! # Tuple and Struct Variants
//!
//! Tuple variants are turned into a `HashMap`, where the data stored in the tuple is the key (so the data must implement `Hash`).
//! Unfortunately, variants with more than one field in them are not supported.
//!
//! Tuple variants are omitted from the struct's `new` function. For example, this:
//!
//! ```
//! # use variants_struct::VariantsStruct;
//! #[derive(VariantsStruct)]
//! enum Hello {
//!     World,
//!     There(i32)
//! }
//! ```
//!
//! produces the following code:
//!
//! ```
//! # enum Hello {
//! #     World,
//! #     There(i32)
//! # }
//! struct HelloStruct<T> {
//!     pub world: T,
//!     pub there: std::collections::HashMap<i32, T>
//! }
//!
//! impl<T> HelloStruct<T> {
//!     fn new(world: T) -> HelloStruct<T> {
//!         HelloStruct {
//!             world,
//!             there: std::collections::HashMap::new()
//!         }
//!     }
//!
//!     pub fn get_unchecked(&self, var: &Hello) -> &T {
//!         match var {
//!             &Hello::World => &self.world,
//!             &Hello::There(key) => self.there.get(&key)
//!                 .expect("tuple variant key not found in hashmap")
//!         }
//!     }
//!
//!     pub fn get_mut_unchecked(&mut self, var: &Hello) -> &mut T {
//!         match var {
//!             &Hello::World => &mut self.world,
//!             &Hello::There(key) => self.there.get_mut(&key)
//!                 .expect("tuple variant key not found in hashmap")
//!         }
//!     }
//!
//!     pub fn get(&self, var: &Hello) -> Option<&T> {
//!         match var {
//!             &Hello::World => Some(&self.world),
//!             &Hello::There(key) => self.there.get(&key)
//!         }
//!     }
//!
//!     pub fn get_mut(&mut self, var: &Hello) -> Option<&mut T> {
//!         match var {
//!             &Hello::World => Some(&mut self.world),
//!             &Hello::There(key) => self.there.get_mut(&key)
//!         }
//!     }
//! }
//! ```
//!
//! Notice that the `new` function now only takes the `world` argument, and the unchecked getter methods query the hashmap and unwrap the result.
//!
//! The same can also be done in struct variants that have only one field.

use proc_macro::TokenStream;
use syn::{Ident, parse_macro_input, ItemEnum, Fields};
use quote::{quote, format_ident};
use inflector::Inflector;
use proc_macro_error2::{proc_macro_error, emit_error, abort};
use check_keyword::CheckKeyword;

/// Stores basic information about variants.
struct VariantInfo {
    normal: Ident,
    snake: Ident,
    fields: Fields
}

/// Derives the variants struct and impl.
#[proc_macro_error]
#[proc_macro_derive(VariantsStruct, attributes(struct_bounds, struct_derive, struct_name, field_name))]
pub fn variants_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);
    let enum_ident = input.ident.clone();
    let mut struct_ident = format_ident!("{}Struct", input.ident);
    let visibility = input.vis.clone();

    // read the `struct_bounds`, `struct_derive`, and `struct_name` attributes. (ignore any others)
    let mut bounds = vec![];
    let mut derives = vec![];
    for attr in input.clone().attrs {
        match attr.parse_meta() {
            Ok(syn::Meta::List(syn::MetaList {path, nested, ..})) => {
                if let Some(ident) = path.get_ident() {
                    let attr_name = ident.to_string();
                    if attr_name == "struct_bounds" || attr_name == "struct_derive" {
                        let mut paths = vec![];
                        for meta in nested {
                            match meta {
                                syn::NestedMeta::Meta(syn::Meta::Path(path)) => {
                                    paths.push(path.clone());
                                }
                                _ => emit_error!(path, "only path arguments are accepted")
                            }
                        }
                        if attr_name == "struct_bounds" {
                            bounds.extend(paths);
                        } else {
                            derives.extend(paths);
                        }
                    }
                }
            }
            Ok(syn::Meta::NameValue(syn::MetaNameValue {path, lit, ..})) => {
                if let Some(ident) = path.get_ident() {
                    let attr_name = ident.to_string();
                    if attr_name == "struct_name" {
                        if let syn::Lit::Str(lit_str) = lit {
                            struct_ident = format_ident!("{}", lit_str.value());
                        } else {
                            emit_error!(lit, "must be a str literal");
                        }
                    }
                }
            }
            _ => {}
        }
    }

    if input.variants.len() == 0 {
        return (quote! {
            #[derive(#(#derives),*)]
            #visibility struct #struct_ident;
        }).into()
    }

    let vars: Vec<_> = input.clone().variants.iter().map(
        |var| {
            let snake = {
                let names: Vec<_> = var.attrs.iter().filter_map(
                    |attr| {
                        match attr.parse_meta() {
                            Ok(syn::Meta::NameValue(syn::MetaNameValue {path, lit, ..})) => {
                                if let Some(ident) = path.get_ident() {
                                    if ident.to_string() == "field_name" {
                                        if let syn::Lit::Str(lit_str) = lit {
                                            Some(lit_str.value())
                                        } else {
                                            abort!(lit, "must be a string literal");
                                        }
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            _ => None
                        }
                    }
                ).collect();
                if names.is_empty() {
                    let name = var.ident.to_string().to_snake_case();
                    format_ident!("{}", name.into_safe())
                } else {
                    format_ident!("{}", names.first().unwrap().to_safe())
                }
            };
            VariantInfo {
                normal: var.ident.clone(),
                snake,
                fields: var.fields.clone()
            }
        }
    ).collect();

    // generate the fields and impl code
    let mut field_idents = vec![];
    let mut field_names = vec![];
    let mut struct_fields = vec![];
    let mut get_uncheckeds = vec![];
    let mut get_mut_uncheckeds = vec![];
    let mut gets = vec![];
    let mut get_muts = vec![];
    let mut new_args = vec![];
    let mut new_fields = vec![];
    for VariantInfo { normal, snake, fields } in &vars {
        field_idents.push(snake.clone());
        field_names.push(snake.to_string());
        match fields {
            Fields::Unit => {
                struct_fields.push(quote! { pub #snake: T });
                gets.push(quote! { &#enum_ident::#normal => Some(&self.#snake) });
                get_muts.push(quote! { &#enum_ident::#normal => Some(&mut self.#snake) });
                get_uncheckeds.push(quote! { &#enum_ident::#normal => &self.#snake });
                get_mut_uncheckeds.push(quote! { &#enum_ident::#normal => &mut self.#snake });
                new_args.push(quote! {#snake: T});
                new_fields.push(quote! {#snake});
            }
            Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => {
                if unnamed.len() == 1 {
                    let ty = unnamed.first().unwrap().clone().ty;
                    struct_fields.push(quote! {
                        pub #snake: std::collections::HashMap<#ty, T>
                    });
                    gets.push(quote! {
                        &#enum_ident::#normal(key) => self.#snake.get(&key)
                    });
                    get_muts.push(quote! {
                        &#enum_ident::#normal(key) => self.#snake.get_mut(&key)
                    });
                    get_uncheckeds.push(quote! {
                        &#enum_ident::#normal(key) => self.#snake.get(&key)
                            .expect("tuple variant key not found in hashmap")
                    });
                    get_mut_uncheckeds.push(quote! {
                        &#enum_ident::#normal(key) => self.#snake.get_mut(&key)
                            .expect("tuple variant key not found in hashmap")
                    });
                    new_fields.push(quote! {#snake: std::collections::HashMap::new()});
                } else {
                    emit_error!(unnamed, "only tuples with one value are allowed");
                }
            }
            Fields::Named(syn::FieldsNamed { named, .. }) => {
                if named.len() == 1 {
                    let ty = named.first().unwrap().clone().ty;
                    let ident = named.first().unwrap().ident.clone().unwrap();
                    struct_fields.push(quote! {
                        pub #snake: std::collections::HashMap<#ty, T>
                    });
                    gets.push(quote! {
                        &#enum_ident::#normal {#ident}  => self.#snake.get(&#ident)
                    });
                    get_muts.push(quote! {
                        &#enum_ident::#normal {#ident}  => self.#snake.get_mut(&#ident)
                    });
                    get_uncheckeds.push(quote! {
                        &#enum_ident::#normal {#ident} => self.#snake.get(&#ident)
                            .expect("tuple variant key not found in hashmap")
                    });
                    get_mut_uncheckeds.push(quote! {
                        &#enum_ident::#normal {#ident} => self.#snake.get_mut(&#ident)
                            .expect("tuple variant key not found in hashmap")
                    });
                    new_fields.push(quote! {#snake: std::collections::HashMap::new()});
                } else {
                    emit_error!(named, "only structs with one field are allowed");
                }
            }
        }
    }

    // combine it all together
    (quote! {
        #[derive(#(#derives),*)]
        #visibility struct #struct_ident<T: #(#bounds)+*> {
            #(#struct_fields),*
        }

        impl<T: #(#bounds)+*> #struct_ident<T> {
            pub fn new(#(#new_args),*) -> #struct_ident<T> {
                #struct_ident {
                    #(#new_fields),*
                }
            }

            pub fn get_unchecked(&self, var: &#enum_ident) -> &T {
                match var {
                    #(#get_uncheckeds),*
                }
            }

            pub fn get_mut_unchecked(&mut self, var: &#enum_ident) -> &mut T {
                match var {
                    #(#get_mut_uncheckeds),*
                }
            }

            pub fn get(&self, var: &#enum_ident) -> Option<&T> {
                match var {
                    #(#gets),*
                }
            }

            pub fn get_mut(&mut self, var: &#enum_ident) -> Option<&mut T> {
                match var {
                    #(#get_muts),*
                }
            }
        }
    }).into()
}
