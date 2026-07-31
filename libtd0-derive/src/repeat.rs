use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident};
use syn::{Attribute, ItemStruct, parse2};

use super::common::{
    get_named_attr, parse_kv_list, validate_attr_list_required_attrs,
    validate_attr_list_valid_attrs,
};

pub fn repeat_fields(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let mut source: ItemStruct = parse2(input).expect("Error parsing input struct!");

    if let syn::Fields::Named(ref mut fields) = source.fields {
        let mut new_fields = syn::FieldsNamed {
            brace_token: Default::default(),
            named: syn::punctuated::Punctuated::new(),
        };

        for field in fields.named.iter() {
            let target_attr = get_named_attr("repeat", field);
            let mut new_attrs = field.attrs.clone();
            new_attrs.retain(|attr| {
                // Keep all except "repeat".
                !attr.path().is_ident("repeat")
            });

            // Early out: The "repeat" attr was not set, add the field and
            // move to the next iteration.
            if target_attr.is_none() {
                new_fields.named.push(field.clone());
                continue;
            }

            let target_attr = target_attr.expect("is_none already checked.");
            let repeat_attrs = match RepeatFieldAttr::try_from(target_attr) {
                Ok(attr) => RepeatFieldAttr::from(attr),
                Err(err) => return err.into_compile_error().into(),
            };

            let format = &repeat_attrs.format;
            let count = repeat_attrs.count;

            for idx in 1..count + 1 {
                let mut new_field = field.clone();
                new_field.attrs = new_attrs.clone();

                let ident_str = format.replace("{}", idx.to_string().as_str());
                new_field.ident = format_ident!("{}", ident_str).into();
                new_fields.named.push(new_field);
            }
        }

        source.fields = syn::Fields::Named(new_fields);
    }

    source.into_token_stream().into()
}

#[derive(Clone, Debug, PartialEq)]
struct RepeatFieldAttr {
    count: u8,
    format: String,
}

impl TryFrom<&Attribute> for RepeatFieldAttr {
    type Error = syn::Error;

    fn try_from(attr: &Attribute) -> Result<Self, <Self as TryFrom<&Attribute>>::Error> {
        static REQUIRED_NAMES: [&str; 2] = ["count", "format"];
        static VALID_NAMES: [&str; 2] = ["count", "format"];

        let kv_list = parse_kv_list(attr)?;
        validate_attr_list_required_attrs(&kv_list, &REQUIRED_NAMES, attr)?;
        validate_attr_list_valid_attrs(&kv_list, &VALID_NAMES)?;

        Ok(Self {
            count: kv_list
                .find_by_name("count")
                .expect("Guarded above.")
                .try_into()?,
            format: kv_list
                .find_by_name("format")
                .expect("Guarded above.")
                .try_into()?,
        })
    }
}
