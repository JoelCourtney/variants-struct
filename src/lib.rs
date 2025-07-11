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
//! #[struct_bounds(Copy + Clone)]
//! enum Hello {
//!     World,
//!     There
//! }
//! ```
//!
//! would produce the following code:
//!
//! ```
//! struct HelloStruct<T: Copy + Clone> {
//!     # go_away: T,
//!     // fields omitted
//! }
//!
//! impl<T: Copy + Clone> HelloStruct<T> {
//!     // methods omitted
//! }
//! ```
//!
//! ## Arbitrary attributes
//!
//! To apply other arbitrary attributes to the struct, use `#[struct_attr(...)]`. For example, if you apply
//! `serde::Serialize` to the struct, and your bounds already include a trait that requires `T: Serialize`,
//! serde will give an error. Serde documentation tells you to add `#[serde(bound(serialize = ...))]`,
//! and you can pass that along with `struct_attr`.
//!
//! ```
//! # use variants_struct::VariantsStruct;
//! # use serde::Serialize;
//! trait MyTrait: Serialize {}
//!
//! #[derive(VariantsStruct)]
//! #[struct_derive(Serialize)]
//! #[struct_bounds(MyTrait)]
//! #[struct_attr(serde(bound(serialize = "T: MyTrait")))]
//! enum MyEnum {
//!     MyVariant
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
//! enum MyEnum {
//!     MyVariant
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

use check_keyword::CheckKeyword;
use heck::ToSnekCase;
use proc_macro::TokenStream;
use proc_macro_error2::{emit_error, proc_macro_error};
use quote::{format_ident, quote};
use syn::{Fields, Ident, ItemEnum, parse_macro_input};

/// Stores basic information about variants.
struct VariantInfo {
    normal: Ident,
    snake: Ident,
    fields: Fields,
}

/// Derives the variants struct and impl.
#[proc_macro_error]
#[proc_macro_derive(
    VariantsStruct,
    attributes(struct_bounds, struct_derive, struct_name, field_name, struct_attr)
)]
pub fn variants_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);
    let enum_ident = input.ident.clone();
    let mut struct_ident = format_ident!("{}Struct", input.ident);
    let visibility = input.vis.clone();

    // read the `struct_bounds`, `struct_derive`, and `struct_name` attributes. (ignore any others)
    let mut bounds = quote! {};
    let mut derives = vec![];
    let mut attrs = vec![];
    for attr in input.clone().attrs {
        if attr.path().is_ident("struct_bounds") {
            let syn::Meta::List(l) = attr.meta else {
                emit_error!(
                    attr,
                    "struct_bounds must be of the form #[struct_bounds(Bound)]"
                );
                return quote! {}.into();
            };
            bounds = l.tokens;
        } else if attr.path().is_ident("struct_derive") {
            attr.parse_nested_meta(|meta| {
                derives.push(meta.path);
                Ok(())
            })
            .unwrap();
        } else if attr.path().is_ident("struct_name") {
            if let syn::Meta::NameValue(syn::MetaNameValue { value, .. }) = attr.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }) = value
                {
                    struct_ident = format_ident!("{}", lit_str.value());
                } else {
                    emit_error!(value, "must be a str literal");
                }
            }
        } else if attr.path().is_ident("struct_attr") {
            let syn::Meta::List(l) = attr.meta else {
                emit_error!(attr, "struct_attr must be of the form #[struct_attr(attr)]");
                return quote! {}.into();
            };
            attrs.push(l.tokens);
        }
    }

    if input.variants.is_empty() {
        return (quote! {
            #[derive(#(#derives),*)]
            #visibility struct #struct_ident;
        })
        .into();
    }

    let vars: Vec<_> = input
        .clone()
        .variants
        .iter()
        .map(|var| {
            let mut names = vec![];
            for attr in &var.attrs {
                if attr.path().is_ident("field_name") {
                    if let syn::Meta::NameValue(syn::MetaNameValue { value, .. }) = &attr.meta {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        }) = value
                        {
                            names.push(lit_str.value());
                        } else {
                            emit_error!(value, "must be a str literal");
                        }
                    }
                }
            }

            let snake = if names.is_empty() {
                format_ident!("{}", var.ident.to_string().to_snek_case().into_safe())
            } else {
                format_ident!("{}", names.first().unwrap().into_safe())
            };
            VariantInfo {
                normal: var.ident.clone(),
                snake,
                fields: var.fields.clone(),
            }
        })
        .collect();

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
    for VariantInfo {
        normal,
        snake,
        fields,
    } in &vars
    {
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
        #(#[#attrs])*
        #visibility struct #struct_ident<T: #bounds> {
            #(#struct_fields),*
        }

        impl<T: #bounds> #struct_ident<T> {
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

        impl<T: #bounds> std::ops::Index<#enum_ident> for #struct_ident<T> {
            type Output = T;
            fn index(&self, var: #enum_ident) -> &T {
                self.get_unchecked(&var)
            }
        }

        impl<T: #bounds> std::ops::IndexMut<#enum_ident> for #struct_ident<T> {
            fn index_mut(&mut self, var: #enum_ident) -> &mut T {
                self.get_mut_unchecked(&var)
            }
        }

        impl<T: #bounds> std::ops::Index<&#enum_ident> for #struct_ident<T> {
            type Output = T;
            fn index(&self, var: &#enum_ident) -> &T {
                self.get_unchecked(var)
            }
        }

        impl<T: #bounds> std::ops::IndexMut<&#enum_ident> for #struct_ident<T> {
            fn index_mut(&mut self, var: &#enum_ident) -> &mut T {
                self.get_mut_unchecked(var)
            }
        }
    })
    .into()
}
