use syn::{
  Expr, Ident, Result,
  parse::{Parse, ParseStream},
};

use crate::shared_type::{self, SharedType};

struct BundleData {
  name: Ident,
  types: Vec<(Ident, SharedType, Expr)>,
}

impl Parse for BundleData {
  fn parse(input: ParseStream) -> Result<Self> {
    let ident: Ident = input.parse()?;
    if ident != "from" {
      return Err(input.error("expected 'from'"));
    }
    let types: Vec<shared_type> = input.parse()?;
    Ok(BundleData {
      name: bundle,
      types: types,
    })
  }
}
