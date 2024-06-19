use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn hello(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let res = quote! {
        impl #name {
            fn hello_world(&self) {
                println!("Hello, World");
            }
        }
    };
    res.into()
}

#[proc_macro_derive(HelloSimple)]
pub fn hello_simple(item: TokenStream) -> TokenStream {
    fn ident_name(item: TokenTree) -> String {
        match item {
            TokenTree::Ident(i) => i.to_string(),
            _ => panic!("no ident"),
        }
    }
    let name = ident_name(item.into_iter().nth(1).unwrap());
    format!(
        r#"
        impl {} {{
            fn hello_world(&self) {{
                println!("Hello, World (simple)");
            }}
        }}
        "#,
        name
    )
    .parse()
    .unwrap()
}
