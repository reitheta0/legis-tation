use syn::{

  Ident, Result,

  parse::{Parse, ParseStream},

};

use crate::shared_type::SharedType;

struct VariantsData {
  name: Ident,
  types: Vec<SharedType>,
  implementation: Option<RustCode>
}

impl Parse for VariantsData {
  fn parse(input: ParseStream) -> Result<Self> {
    let name: Ident;
    let ident: Ident = input.parse()?;
    if ident != "variants" {
      return Err(input.error("expected 'from'"));
    }
    let ident: Ident = input.parse()?;
    if ident != "from" {
      return Err(input.error("expected 'from'"));
    }
    let types: Vec<Type> = input.parse()?;
    Ok(VariantsData {
      name: name,
      types: types,
      implementation: //TODO: implement,
    })
  }
}
