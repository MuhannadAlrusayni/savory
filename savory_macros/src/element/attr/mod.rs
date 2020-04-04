pub(crate) mod lens;

use syn::{
    parse::{Parse, ParseStream, Result},
    Ident,
};

use self::lens::Lens;

#[derive(Debug)]
pub(crate) enum Attribute {
    Lens(Lens),
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        match input.fork().parse::<Ident>()?.to_string().as_str() {
            "theme_lens" => input.parse::<Lens>().map(Attribute::Lens),
            _ => Err(input.error("`element` only accept `theme_lens` attribute"))?,
        }
    }
}
