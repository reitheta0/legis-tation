use syn::{
  Expr, Ident, Result, Token,
  parse::{Parse, ParseStream},
};

pub struct SharedType {
  name: Ident,
  default_value: Option<Expr>,
}

impl Parse for SharedType {
  fn parse(input: ParseStream) -> Result<Self> {
    let _: Token![type] = input.parse()?;
    let _: Token![=] = input.parse()?;
    let name: Ident = input.parse()?;
    let default_value: Option<Expr>;
    if let _ = input.parse::<Token![default]>()
      && let _ = input.parse::<Token![=]>()
    {
      default_value = Some(input.parse()?);
    } else {
      default_value = None;
    }
    Ok(SharedType {
      name: name,
      default_value: default_value,
    })
  }
}
