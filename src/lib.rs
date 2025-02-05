extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use rand::Rng;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn obfuscate(_: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let func_sig = &func.sig;
    let func_attrs = &func.attrs;
    let func_vis = &func.vis;
    let block = &func.block;
    let stmts = &block.stmts;
    let mut new_stmts = proc_macro2::TokenStream::new();

    for stmt in stmts {
        let junk = generate_junk_block();
        new_stmts.extend(junk);
        new_stmts.extend(quote! { #stmt });
    }

    let output = quote! {
        #(#func_attrs)*
        #func_vis #func_sig {
            #new_stmts
        }
    };

    // eprintln!("Generated function:\n{}", output.to_string());

    output.into()
}

#[derive(Clone, Copy)]
enum JunkType {
    Number,
    Boolean,
    String,
}

impl JunkType {
    fn random(rng: &mut impl Rng) -> Self {
        match rng.random_range(0..3) {
            0 => JunkType::Number,
            1 => JunkType::Boolean,
            _ => JunkType::String,
        }
    }
}

fn generate_junk_block() -> proc_macro2::TokenStream {
    let mut rng = rand::rng();
    let junk_type = JunkType::random(&mut rng);
    let var_count = rng.random_range(100..201);

    let (ty, init_value) = match junk_type {
        JunkType::Number => (quote!(u32), quote!(42u32)),
        JunkType::Boolean => (quote!(bool), quote!(true)),
        JunkType::String => (quote!(String), quote!(String::from("initial"))),
    };

    let base_var = syn::Ident::new("_x", proc_macro2::Span::call_site());
    let mut var_statements = proc_macro2::TokenStream::new();

    for i in 0..var_count {
        let var_name = format!("_v{}", i);
        let var_ident = syn::Ident::new(&var_name, proc_macro2::Span::call_site());

        let expr = match junk_type {
            JunkType::Number => {
                let num = rng.random::<u32>();
                match rng.random_range(0..6) {
                    0 => quote! { #base_var.wrapping_add(#num) },
                    1 => quote! { #base_var.wrapping_sub(#num) },
                    2 => quote! { #base_var.wrapping_shl(1) },
                    3 => quote! { #base_var.wrapping_shr(1) },
                    4 => quote! { #base_var.wrapping_mul(#num) },
                    _ => quote! { #base_var.wrapping_rem(#num.wrapping_add(1)) },
                }
            }
            JunkType::Boolean => match rng.random_range(0..3) {
                0 => quote! { !#base_var },
                1 => quote! { #base_var && true },
                _ => quote! { #base_var || false },
            },
            JunkType::String => match rng.random_range(0..3) {
                0 => quote! { format!("{}", #base_var) },
                1 => quote! { #base_var.clone() + "" },
                _ => quote! { String::from(#base_var) },
            },
        };

        var_statements.extend(quote! {
            let #var_ident: #ty = #expr;
            #base_var = #var_ident;
        });
    }

    quote! {{
        let mut #base_var: #ty = #init_value;
        #var_statements
        let _ = &#base_var;
    }}
}
