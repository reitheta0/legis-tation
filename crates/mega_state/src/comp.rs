struct Comp {
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
}

impl quote::ToTokens for Comp {
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
