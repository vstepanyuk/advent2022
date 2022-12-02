use darling::FromDeriveInput;
// use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(aoc))]
struct Opts {
    file: Option<String>,
}

#[proc_macro_derive(Runner, attributes(aoc))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;

    let filename = match opts.file {
        Some(filename) => filename,
        None => format!("inputs/{}.txt", ident.to_string().to_lowercase()),
    };

    let output = quote! {
        impl Default for #ident {
            fn default() -> Self {
                Self {
                    filename: #filename,
                }
            }
        }

        impl Runnable for #ident {
            fn filename(&self) -> String {
                self.filename.to_owned()
            }
        }
    };
    output.into()
}
