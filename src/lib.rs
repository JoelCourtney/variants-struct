use proc_macro::TokenStream;
use syn::{Ident, parse_macro_input, ItemEnum, Fields};
use quote::{quote, format_ident};
use inflector::Inflector;
use proc_macro_error::{proc_macro_error, emit_error};

/// Stores basic information about variants.
struct VariantInfo {
    normal: Ident,
    snake: Ident,
    fields: Fields
}

/// Derives the variants struct and generates impls.
#[proc_macro_error]
#[proc_macro_derive(VariantsStruct, attributes(struct_bounds, struct_derive, struct_name))]
pub fn variants_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);
    let enum_ident = input.ident.clone();
    let mut struct_ident = format_ident!("{}Struct", input.ident);
    let visibility = input.vis.clone();

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
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let vars: Vec<_> = input.clone().variants.iter().map(
        |var| VariantInfo {
            normal: var.ident.clone(),
            snake: format_ident!("{}", var.ident.to_string().to_snake_case()),
            fields: var.fields.clone()
        }
    ).collect();

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
                        &#enum_ident::#normal(key) => self.#snake.get(key)
                    });
                    get_muts.push(quote! {
                        &#enum_ident::#normal(key) => self.#snake.get_mut(key)
                    });
                    get_uncheckeds.push(quote! {
                        &#enum_ident::#normal(key) => self.#snake.get(key)
                            .expect("tuple variant key not found in hashmap")
                    });
                    get_mut_uncheckeds.push(quote! {
                        &#enum_ident::#normal(key) => self.#snake.get_mut(key)
                            .expect("tuple variant key not found in hashmap")
                    });
                    new_fields.push(quote! {#snake: std::collections::HashMap::new()});
                } else {
                    emit_error!(unnamed, "only tuple variants with exactly one value are allowed");
                }
            }
            _ => {}
        }
    }



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
