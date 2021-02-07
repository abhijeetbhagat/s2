extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, NestedMeta};

#[proc_macro_attribute]
pub fn job(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let mut value = 1;
    for arg in args {
        match arg {
            NestedMeta::Meta(_) => {
                // TODO abhi: flag error here
            }
            NestedMeta::Lit(lit) => {
                match lit {
                    syn::Lit::Int(secs) => {
                        value = secs.base10_parse::<u64>().unwrap();
                        break;
                    }
                    _ => {
                        // TODO abhi: flag error here
                    }
                }
            }
        }
    }

    let mut input = syn::parse_macro_input!(input as syn::ItemFn);
    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &mut input.sig;
    let body = &input.block;

    (quote! {
        #(#attrs)*
        #vis #sig {
            use std::thread;
            use std::time::Duration;
            use std::sync::mpsc::TryRecvError;

            loop {
                #body
                thread::sleep(Duration::from_millis(#value * 1000));
                match rx.try_recv() {
                    Ok(_) => break,
                    Err(e) => match e {
                        TryRecvError::Empty => continue,
                        _ => break
                    }
                }
            }
        }
    })
    .into()
}
