use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, format_ident};
use syn::{Attribute, Field, FieldsNamed, ItemStruct, parse2};

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
    static MY_ATTR_NAMES: [&str; 3] = ["repeat", "repeat_section", "repeat_section_last"];

    if let syn::Fields::Named(ref mut fields) = source.fields {
        let mut new_fields = FieldsNamed {
            brace_token: Default::default(),
            named: syn::punctuated::Punctuated::new(),
        };
        let mut new_section_fields = FieldsNamed {
            brace_token: Default::default(),
            named: syn::punctuated::Punctuated::new(),
        };
        let fake_ident: Ident = format_ident!("{}", "__BOGUS_IDENT_DO_NOT_USE__");

        let mut section_attr: Option<RepeatSectionAttr> = None;
        let mut section_last_attr: Option<RepeatSectionLastAttr> = None;

        for field in fields.named.iter() {
            let repeat_field_attr = get_named_attr("repeat", field);
            let repeat_section_attr = get_named_attr("repeat_section", field);
            let repeat_section_last_attr = get_named_attr("repeat_section_last", field);

            match (repeat_section_attr, repeat_section_last_attr) {
                (Some(_), None) => {
                    if section_attr.is_some() {
                        return syn::Error::new_spanned(repeat_section_attr.expect("Infallible"), "`repeat_section` must be closed with `repeat_section_last` prior to opening a new one.").into_compile_error().into();
                    }
                    section_attr =
                        match RepeatSectionAttr::try_from(repeat_section_attr.expect("Infallible"))
                        {
                            Ok(rpt_attr) => Some(rpt_attr),
                            Err(err) => {
                                return err.to_compile_error().into();
                            }
                        };
                    new_section_fields.named.clear();
                }
                (None, Some(_)) => {
                    if section_last_attr.is_some() {
                        return syn::Error::new_spanned(repeat_section_last_attr.expect("Infallible"), "`repeat_section` must be closed with `repeat_section_last` prior to opening a new one.").into_compile_error().into();
                    }

                    if section_attr.is_none() {
                        return syn::Error::new_spanned(
                            repeat_section_last_attr.expect("Infallible"),
                            "`repeat_section_last` requires an opening `repeat_section`.",
                        )
                        .into_compile_error()
                        .into();
                    }

                    section_last_attr = match RepeatSectionLastAttr::try_from(
                        repeat_section_last_attr.expect("Infallible"),
                    ) {
                        Ok(rpt_last_attr) => Some(rpt_last_attr),
                        Err(err) => {
                            return err.to_compile_error().into();
                        }
                    }
                }
                (Some(_), Some(_)) => {
                    return syn::Error::new_spanned(
                        repeat_section_attr.expect("Infallible"),
                        "`repeat_section` must be comprised of at least 2 fields.",
                    )
                    .into_compile_error()
                    .into();
                }
                (None, None) => (),
            }

            let mut new_attrs = field.attrs.clone();
            new_attrs.retain(|attr| {
                // Keep all except "repeat" and its related attrs.
                let attr_path_name = attr
                    .path()
                    .get_ident()
                    .unwrap_or_else(|| &fake_ident)
                    .to_string();
                !MY_ATTR_NAMES.contains(&attr_path_name.as_str())
            });

            // Early out: The "repeat" attr was not set, add the field and
            // move to the next iteration.
            if repeat_field_attr.is_none() {
                let mut fld = field.clone();
                fld.attrs = new_attrs.clone();
                if section_attr.is_some() {
                    new_section_fields.named.push(fld);
                    if section_last_attr.is_some() {
                        let attr = RepeatSectionAttr::try_from(section_attr.expect("Infallible"))
                            .expect("Infallible");
                        push_section_repeated(&mut new_fields, &new_section_fields, &attr);
                        section_attr = None;
                        section_last_attr = None;
                    }
                } else {
                    new_fields.named.push(fld);
                }

                continue;
            }

            let repeat_field_attr = repeat_field_attr.expect("is_none already checked.");
            let repeat_attrs = match RepeatFieldAttr::try_from(repeat_field_attr) {
                Ok(attr) => RepeatFieldAttr::from(attr),
                Err(err) => return err.into_compile_error().into(),
            };

            let format = &repeat_attrs.format;
            let count = repeat_attrs.count;

            if section_attr.is_some() {
                push_field_repeated(&mut new_section_fields, field, &new_attrs, count, format);
            } else {
                push_field_repeated(&mut new_fields, field, &new_attrs, count, format);
            }

            if section_last_attr.is_some() {
                let attr = RepeatSectionAttr::try_from(section_attr.expect("Infallible"))
                    .expect("Infallible");
                push_section_repeated(&mut new_fields, &new_section_fields, &attr);
                push_section_repeated(&mut new_fields, &new_section_fields, &attr);
                section_attr = None;
                section_last_attr = None;
            }
        }

        source.fields = syn::Fields::Named(new_fields);
    }

    source.into_token_stream().into()
}

fn push_field_repeated(
    dest: &mut FieldsNamed,
    field: &Field,
    new_attrs: &Vec<Attribute>,
    count: u8,
    format: &str,
) {
    for idx in 1..count + 1 {
        let mut new_field = field.clone();
        new_field.attrs = new_attrs.clone();

        let ident_str = format.replace("{}", idx.to_string().as_str());
        new_field.ident = format_ident!("{}", ident_str).into();
        dest.named.push(new_field);
    }
}

fn push_section_repeated(
    dest: &mut FieldsNamed,
    section: &FieldsNamed,
    section_attr: &RepeatSectionAttr,
) {
    for section_num in 1..section_attr.count + 1 {
        let section_prefix = section_attr
            .prefix_format
            .replace("{}", section_num.to_string().as_str());
        for field in section.named.iter() {
            let mut new_field = field.clone();
            new_field.ident = Some(format_ident!(
                "{}_{}",
                section_prefix,
                &new_field
                    .ident
                    .clone()
                    .expect("Field is named.")
                    .to_string()
            ));
            dest.named.push(new_field);
        }
    }
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

struct RepeatSectionAttr {
    count: u8,
    prefix_format: String,
}

impl TryFrom<&Attribute> for RepeatSectionAttr {
    type Error = syn::Error;

    fn try_from(attr: &Attribute) -> Result<Self, <Self as TryFrom<&Attribute>>::Error> {
        static REQUIRED_NAMES: [&str; 2] = ["count", "prefix_format"];
        static VALID_NAMES: [&str; 2] = ["count", "prefix_format"];

        let kv_list = parse_kv_list(attr)?;
        validate_attr_list_required_attrs(&kv_list, &REQUIRED_NAMES, attr)?;
        validate_attr_list_valid_attrs(&kv_list, &VALID_NAMES)?;

        Ok(Self {
            count: kv_list
                .find_by_name("count")
                .expect("Guarded above.")
                .try_into()?,
            prefix_format: kv_list
                .find_by_name("prefix_format")
                .expect("Guarded above.")
                .try_into()?,
        })
    }
}

struct RepeatSectionLastAttr;

impl TryFrom<&Attribute> for RepeatSectionLastAttr {
    type Error = syn::Error;

    fn try_from(attr: &Attribute) -> Result<Self, <Self as TryFrom<&Attribute>>::Error> {
        static REQUIRED_NAMES: [&str; 0] = [];
        static VALID_NAMES: [&str; 0] = [];

        let kv_list = parse_kv_list(attr)?;
        validate_attr_list_required_attrs(&kv_list, &REQUIRED_NAMES, attr)?;
        validate_attr_list_valid_attrs(&kv_list, &VALID_NAMES)?;

        Ok(Self {})
    }
}
