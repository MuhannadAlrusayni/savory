use super::attr::Attribute;
use syn::parse::Parse;

#[derive(Debug)]
pub(crate) enum FieldType {
    Option(syn::Type),
    Normal(syn::Type),
}

#[derive(Debug)]
pub(crate) struct Field {
    pub(crate) attrs: Vec<Attribute>,
    pub(crate) name: syn::Ident,
    pub(crate) ty: FieldType,
}

impl Field {
    pub(crate) fn from_syn_field(field: &syn::Field) -> Result<Self, syn::Error> {
        // get name
        let name = field
            .ident
            .clone()
            .ok_or_else(|| syn::Error::new_spanned(field, "expected field ident"))?;

        // get type
        let ty = field.ty.clone();
        let ty = match ty {
            syn::Type::Path(syn::TypePath {
                path: syn::Path { ref segments, .. },
                ..
            }) => {
                let last_segment = segments.iter().last();
                match last_segment {
                    Some(syn::PathSegment {
                        ident,
                        arguments:
                            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                                args,
                                ..
                            }),
                    }) => {
                        if ident != "Option" {
                            FieldType::Normal(ty)
                        } else {
                            let first_arg = args.into_iter().nth(0);
                            match first_arg {
                                Some(syn::GenericArgument::Type(ty)) => {
                                    FieldType::Option(ty.clone())
                                }
                                _ => FieldType::Normal(ty),
                            }
                        }
                    }
                    _ => FieldType::Normal(ty),
                }
            }
            _ => FieldType::Normal(ty),
        };

        // get attributes
        let mut attrs = vec![];
        for attr in field.attrs.iter() {
            if attr.path.is_ident("element") {
                let result = attr
                    .parse_args_with(|parse_stream: syn::parse::ParseStream| {
                        parse_stream.parse_terminated::<_, syn::Token![,]>(Attribute::parse)
                    })?
                    .into_iter()
                    .collect::<Vec<Attribute>>();
                attrs.extend(result);
            }
        }

        Ok(Field { attrs, name, ty })
    }
}
