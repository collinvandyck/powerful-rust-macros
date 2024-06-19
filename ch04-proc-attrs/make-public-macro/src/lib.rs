#![allow(unused)]

use paste::paste;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, token::Colon, DeriveInput, Visibility,
};

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let fields = match ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only structs with named fields"),
    };
    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { pub #name: #ty }
    });
    let public_version = quote! {
        pub struct #name {
            #(#builder_fields,)*
        }
    };
    public_version.into()
}

struct StructField {
    name: syn::Ident,
    ty: syn::Type,
}

impl StructField {
    fn new(field: &syn::Field) -> Self {
        Self {
            name: field.ident.as_ref().unwrap().clone(),
            ty: field.ty.clone(),
        }
    }
}

impl quote::ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;
        quote! {pub #n: #t}.to_tokens(tokens)
    }
}

/// uses a struct to generate the output fields
#[proc_macro_attribute]
pub fn public_2(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let fields = match ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only structs with named fields"),
    };
    let fields = fields.iter().map(|f| StructField::new(f));
    let public_version = quote! {
        pub struct #name {
            #(#fields,)*
        }
    };
    public_version.into()
}

struct ParsingStructField {
    name: syn::Ident,
    ty: syn::Ident,
}

impl Parse for ParsingStructField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _vis: syn::Result<Visibility> = input.parse();
        let list = Punctuated::<syn::Ident, Colon>::parse_terminated(input).unwrap();
        Ok(ParsingStructField {
            name: list.first().unwrap().clone(),
            ty: list.last().unwrap().clone(),
        })
    }
}

impl quote::ToTokens for ParsingStructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;
        quote! {pub #n: #t}.to_tokens(tokens)
    }
}

/// uses syn::parse::Parse impl to generate output fields
#[proc_macro_attribute]
pub fn public_3(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let fields = match ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only structs with named fields"),
    };
    let fields = fields
        .iter()
        .map(|f| syn::parse2::<ParsingStructField>(f.to_token_stream()).unwrap());
    let public_version = quote! {
        pub struct #name {
            #(#fields,)*
        }
    };
    public_version.into()
}
