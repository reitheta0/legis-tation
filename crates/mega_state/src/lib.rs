mod bundle_data;
mod hash_map;
mod shared_type;
mod variants_data;

use logging::macros::logln;
use proc_macro::TokenStream;
use quote::quote;
use syn::Expr;
use syn::parse::{Parse, ParseStream};
use syn::{
  Error, Ident, Lit, LitStr, Result, Token, Type, braced, parse_macro_input, punctuated::Punctuated,
};

//
//
//
//
//
// This macro uses the notation from {object} get {proprety};
//
// to use the logln! macro to log to a file as well as
//
// print the result in the console.
//
//
//
//
//
/*struct Comp {
  object: Object,
  property: Property,
}

impl Parse for Comp {
  fn parse(input: ParseStream) -> Result<Self> {
    let ident: Ident = input.parse()?;
    if ident != "from" {
      return Err(input.error("expected 'from'"));
    }
    let object: Object = input.parse()?;
    let ident: Ident = input.parse()?;
    if ident != "get" {
      return Err(input.error("expected 'from'"));
    }
    let property: Property = input.parse()?;
    Ok(Self { object, property })
  }
}*/

/*impl quote::ToTokens for Comp {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let Object { name: object } = &self.object;
    let Property { name: property } = &self.property;
    let method_ident = Ident::new(&format!("get_{}", property.to_string()), property.span());
    let grabbed = quote! { #method_ident };

    tokens.extend(quote! {
    if let Some(grabbed) = #object.#grabbed() {
      logln!('t', "Here's the {} property of {}: {}!", #property, #object, grabbed);
    } else {
      logln!('i', "No {} found for this {}.", #property, #object);
    }
    });
  }
}

#[proc_macro]
pub fn grab_get(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let comp: Comp = parse_macro_input!(input as Comp);
  quote! { #comp }.into()
}*/
