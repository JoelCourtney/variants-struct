use proc_macro::TokenStream;
use syn::{Ident, parse_macro_input, ItemEnum, Fields};
use quote::{quote, format_ident};
use inflector::Inflector;

/// Stores basic information about variants.
struct VariantInfo {
    normal: Ident,
    snake: Ident,
    fields: Fields
}

/// Derives the variants struct and generates impls.
#[proc_macro_derive(VariantsStruct)]
pub fn variants_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);
    let enum_ident = input.ident.clone();
    let struct_ident = format_ident!("{}Struct", input.ident);
    let visibility = input.vis.clone();

    let vars: Vec<_> = input.clone().variants.iter().map(
        |var| VariantInfo {
            normal: var.ident.clone(),
            snake: format_ident!("{}", var.ident.to_string().to_snake_case()),
            fields: var.fields.clone()
        }
    ).collect();

    let mut field_names = vec! [];
    let mut struct_fields = vec! [];
    let mut gets = vec! [];
    let mut get_muts = vec! [];
    let mut new_args = vec! [];
    let mut new_fields = vec! [];
    for VariantInfo {normal, snake, fields} in vars {
        field_names.push(snake.clone());
        match fields {
            Fields::Unit => {
                struct_fields.push(quote! { pub #snake: T });
                gets.push(quote! { #enum_ident::#normal => &self.#snake });
                get_muts.push(quote! { #enum_ident::#normal => &mut self.#snake });
                new_args.push(quote! {#snake: T});
                new_fields.push(quote! {#snake});
            }
            Fields::Unnamed(syn::FieldsUnnamed{ unnamed, .. }) => {
                if unnamed.len() == 1 {
                    let ty = unnamed.first().unwrap().clone().ty;
                    struct_fields.push(quote! {
                        pub #snake: std::collections::HashMap<#ty, T>
                    });
                    gets.push(quote! {
                        #enum_ident::#normal(ref key) => self.#snake.get(key).unwrap()
                    });
                    get_muts.push(quote! {
                        #enum_ident::#normal(ref key) => self.#snake.get_mut(key).unwrap()
                    });
                } else {
                    let types: Vec<_> = unnamed.iter().map(
                        |field| field.ty.clone()
                    ).collect();
                    let mut match_args = vec! [];
                    for i in 0..unnamed.len() {
                        match_args.push(format_ident!("match_arg_{}", i));
                    }
                    struct_fields.push(quote! {
                        pub #snake: std::collections::HashMap<(#(#types),*), T>
                    });
                    gets.push(quote! {
                        #enum_ident::#normal(#(#match_args),*) => self.#snake.get(&(#(#match_args),*)).unwrap()
                    });
                    get_muts.push(quote! {
                        #enum_ident::#normal(#(#match_args),*) => self.#snake.get_mut(&(#(#match_args),*)).unwrap()
                    });
                }
                new_fields.push(quote! {#snake: std::collections::HashMap::new()});
            }
            _ => {}
        }
    }

    (quote! {
        #visibility struct #struct_ident<T> {
            #(#struct_fields),*
        }

        impl<T> #struct_ident<T> {
            pub fn new(#(#new_args),*) -> #struct_ident<T> {
                #struct_ident {
                    #(#new_fields),*
                }
            }

            pub fn get(&self, var: #enum_ident) -> &T {
                match var {
                    #(#gets),*
                }
            }

            pub fn get_mut(&mut self, var: #enum_ident) -> &mut T {
                match var {
                    #(#get_muts),*
                }
            }
        }

        impl<T: Default> Default for #struct_ident<T> {
            fn default() -> #struct_ident<T> {
                #struct_ident {
                    #(#field_names: Default::default()),*
                }
            }
        }
    }).into()
}
