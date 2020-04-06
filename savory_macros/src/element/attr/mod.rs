pub(crate) mod lens;
pub(crate) mod props;

use syn::{
    parse::{Parse, ParseStream, Result},
    Ident,
};

use self::{lens::Lens, props::Props};

#[derive(Debug)]
pub(crate) enum Attribute {
    Lens(Lens),
    Props(Props),
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        match input.fork().parse::<Ident>()?.to_string().as_str() {
            "theme_lens" => input.parse::<Lens>().map(Attribute::Lens),
            "props" => input.parse::<Props>().map(Attribute::Props),
            _ => Err(input.error("`element` only accept `theme_lens` attribute"))?,
        }
    }
}
