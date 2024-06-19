use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Hello)]
pub fn hello(_item: TokenStream) -> TokenStream {
    let res = quote! {
        impl Example {
            fn hello_world(&self) {
                println!("Hello, World");
            }
        }
    };
    res.into()
}
