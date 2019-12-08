extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

mod utils;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Lit, Meta, MetaNameValue, TypePath};

#[proc_macro_derive(Motor)]
pub fn motor_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let gen = quote! {
        impl Motor for #name {}
    };
    gen.into()
}

#[proc_macro_derive(TachoMotor)]
pub fn tacho_motor_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let gen = quote! {
        impl TachoMotor for #name {}
    };
    gen.into()
}

#[proc_macro_derive(ServoMotor)]
pub fn servo_motor_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let gen = quote! {
        impl ServoMotor for #name {}
    };
    gen.into()
}

#[proc_macro_derive(DcMotor)]
pub fn dc_motor_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let gen = quote! {
        impl DcMotor for #name {}
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

#[proc_macro_derive(Findable, attributes(class_name, driver_name, port))]
pub fn findable_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;

    let mut class_attr: Option<String> = None;
    let mut driver_attr: Option<String> = None;
    let mut port_attr: Option<String> = None;

    for option in ast.attrs {
        let option = option.parse_meta().unwrap();
        match option {
            Meta::NameValue(MetaNameValue {
                ref path, ref lit, ..
            }) if path.get_ident().unwrap() == "class_name" => {
                if let Lit::Str(lit) = lit {
                    class_attr = Some(lit.value());
                }
            }
            Meta::NameValue(MetaNameValue {
                ref path, ref lit, ..
            }) if path.get_ident().unwrap() == "driver_name" => {
                if let Lit::Str(lit) = lit {
                    driver_attr = Some(lit.value());
                }
            }
            Meta::NameValue(MetaNameValue { ref path, lit, .. })
                if path.get_ident().unwrap() == "port" =>
            {
                if let Lit::Str(lit) = lit {
                    port_attr = Some(lit.value());
                }
            }
            _ => {}
        }
    }

    let class_attr = class_attr.unwrap_or_else(|| "".to_owned());
    let driver_attr = driver_attr.unwrap_or_else(|| "".to_owned());
    let port_attr = port_attr.unwrap_or_else(|| "crate::Motor".to_owned());
    let port_type: TypePath = utils::parse_str(&port_attr).unwrap();

    let gen = quote! {
        impl Findable<#port_type> for #name {
            fn get(port: #port_type) -> Ev3Result<Self> {
                let name = Driver::find_name_by_port_and_driver(#class_attr, &port, #driver_attr)?;

                Ok(#name {
                    driver: Driver::new(#class_attr, &name),
                })
            }

            fn find() -> Ev3Result<Self> {
                let name = Driver::find_name_by_driver(#class_attr, #driver_attr)?;

                Ok(#name {
                    driver: Driver::new(#class_attr, &name),
                })
            }

            fn list() -> Ev3Result<Vec<Self>> {
                Ok(
                    Driver::find_names_by_driver(#class_attr, #driver_attr)?
                        .iter()
                        .map(|name| #name {
                            driver: Driver::new(#class_attr, name),
                        })
                        .collect(),
                )
            }
        }
    };
    gen.into()
}
