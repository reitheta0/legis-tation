use proc_macro::TokenStream;
use quote::quote;
use syn::{Item, parse_macro_input};

#[proc_macro_attribute]
pub fn transform(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let input = parse_macro_input!(item as Item);

  let output = match input {
    Item::Struct(mut st) => {
      st.attrs.push(
        syn::parse_quote! {
            #[derive(Clone, Debug, getset_macro::GetSet, smart_default::SmartDefault,  serde::Deserialize, serde::Serialize)]
        },
      );
      Item::Struct(st)
    }
    Item::Enum(mut en) => {
      en.attrs.push(
        syn::parse_quote! {
            #[derive(Clone, Debug, smart_default::SmartDefault,  serde::Deserialize, serde::Serialize)]
        },
      );
      Item::Enum(en)
    }
    Item::Union(un) => Item::Union(un),
    _ => {
      panic!("Invalid input: not an enum, union or struct!")
    }
  };

  TokenStream::from(quote!(#output))
}

#[proc_macro_attribute]
pub fn transform_d(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let input = parse_macro_input!(item as Item);

  let output = match input {
    Item::Struct(mut st) => {
      st.attrs.push(syn::parse_quote! {
          #[derive(Clone, Debug, getset_macro::GetSet, serde::Deserialize, serde::Serialize)]
      });
      Item::Struct(st)
    }
    Item::Enum(mut en) => {
      en.attrs.push(syn::parse_quote! {
          #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
      });
      Item::Enum(en)
    }
    Item::Union(un) => Item::Union(un),
    _ => {
      panic!("Invalid input: not an enum, union or struct!")
    }
  };

  TokenStream::from(quote!(#output))
}

#[proc_macro_attribute]
pub fn transform_s(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let input = parse_macro_input!(item as Item);

  let output = match input {
    Item::Struct(mut st) => {
      st.attrs.push(syn::parse_quote! {
          #[derive(Clone, Debug, getset_macro::GetSet, smart_default::SmartDefault)]
      });
      Item::Struct(st)
    }
    Item::Enum(mut en) => {
      en.attrs.push(syn::parse_quote! {
          #[derive(Clone, Debug, smart_default::SmartDefault)]
      });
      Item::Enum(en)
    }
    Item::Union(un) => Item::Union(un),
    _ => {
      panic!("Invalid input: not an enum, union or struct!")
    }
  };

  TokenStream::from(quote!(#output))
}

#[proc_macro_attribute]
pub fn transform_sd(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let input = parse_macro_input!(item as Item);

  let output = match input {
    Item::Struct(mut st) => {
      st.attrs.push(syn::parse_quote! {
          #[derive(Clone, Debug, getset_macro::GetSet)]
      });
      Item::Struct(st)
    }
    Item::Enum(mut en) => {
      en.attrs.push(syn::parse_quote! {
          #[derive(Clone, Debug)]
      });
      Item::Enum(en)
    }
    Item::Union(un) => Item::Union(un),
    _ => {
      panic!("Invalid input: not an enum, union or struct!")
    }
  };

  TokenStream::from(quote!(#output))
}
