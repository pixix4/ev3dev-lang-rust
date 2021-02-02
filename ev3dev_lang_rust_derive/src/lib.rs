extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Device)]
pub fn device_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;

    let gen = quote! {
        impl Device for #name {
            fn get_attribute(&self, name: &str) -> Attribute {
                self.driver.get_attribute(name)
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Sensor)]
pub fn sensor_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let gen = quote! {
        impl Sensor for #name {}
    };
    gen.into()
}
