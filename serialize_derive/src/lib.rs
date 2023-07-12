extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::Type;

#[proc_macro_derive(Serialize)]
pub fn serialize_derive(input: TokenStream) -> TokenStream { 
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    // println!("{}",impl_serialize_macro(&ast).to_string());
    impl_serialize_macro(&ast)
}
fn impl_serialize_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let attrs = &ast.data;
    let struct_data = match attrs {
        syn::Data::Struct(r#struct) => r#struct,
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    };
    let attributes = &struct_data.fields;
    // for field in attributes {
    //     // println!("{:?}: {:?}",field.ident, field.ty);
    // }
    let fields: &Vec<&Option<syn::Ident>> = &attributes.iter().map(|x| &x.ident).collect();
    let types: &Vec<&Type> = &attributes.iter().map(|x| &x.ty).collect();
    let gen = quote! {
        impl Serialize for #name {
            fn serialize(self) -> serialr::Bytes {
                let mut bytes = serialr::Bytes::new();
                #(bytes.append(&self.#fields.serialize());)*
                return bytes;
            }
            fn deserialize(bytes: &serialr::Bytes, mut index: usize) -> Option<Self> {
                let (#(#fields),*): (#(#types),*);
                #(if let Some(field) = bytes.read(index) {
                    #fields = field;
                    index += #fields.size();
                } else {
                    return None;
                })*
                return Some(#name { #(#fields),* });
            }
            fn size(&self) -> usize {
                #(self.#fields.size())+*
            }
            
        }
    };
    gen.into()
}
