extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use std::fmt;

use proc_macro::TokenStream;

#[proc_macro_derive(Display)]
pub fn display(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_display(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_display(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl Display for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                f.write_str(stringify!(#name))
            }
        }
    }
}