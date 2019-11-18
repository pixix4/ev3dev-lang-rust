extern crate proc_macro;
extern crate quote;
extern crate syn;

use crate::proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Device)]
pub fn device_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_device_macro(&ast)
}

fn impl_device_macro(ast: &syn::DeriveInput) -> TokenStream {
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
