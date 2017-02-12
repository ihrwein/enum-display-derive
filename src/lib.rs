extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Display)]
pub fn display(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    let gen = match ast.body {
        syn::Body::Enum(ref variants) => {
            let name = &ast.ident;
            impl_display(name, variants)
        }
        _ => panic!("#[derive(Display)] works only on enums"),
    };

    gen.parse().unwrap()
}

fn impl_display(name: &syn::Ident, variants: &[syn::Variant]) -> quote::Tokens {
    let variants = variants.iter()
        .map(|variant| impl_display_for_variant(name, variant));

    quote! {
        impl Display for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                match *self {
                    #(#variants)*
                }
            }
        }
    }
}

fn impl_display_for_variant(name: &syn::Ident, variant: &syn::Variant) -> quote::Tokens {
    let id = &variant.ident;
    match variant.data {
        syn::VariantData::Unit => {
            quote! {
                #name::#id => {
                    f.write_str(stringify!(#id))
                }
            }
        }
        syn::VariantData::Tuple(ref fields) => {
            match fields.len() {
                0 => {
                    quote! {
                        #name::#id() => {
                            f.write_str(stringify!(#id()))
                        }
                    }
                }
                1 => {
                    quote! {
                        #name::#id(ref inner) => {
                            ::std::fmt::Display::fmt(inner, f)
                        }
                    }
                }
                _ => {
                    panic!("#[derive(Display)] does not support tuple variants with more than one \
                            fields")
                }
            }
        }
        _ => panic!("#[derive(Display)] works only with unit and tuple variants"),
    }
}
