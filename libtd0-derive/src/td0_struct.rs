// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use core::convert::Into;
use num_traits::Bounded;
use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};
use std::ops::Mul;
use syn::{Arm, Expr, LitFloat, Stmt, parse_quote};
use syn::{Attribute, Field, Ident, ImplItemFn, ItemStruct, parse2};

use crate::common::NativeType;

use super::common::{
    AttrKeyValueList, get_field_native_type, get_named_attr, parse_kv_list, push_arm_to_fn_match,
    validate_attr_list_required_attrs, validate_attr_list_valid_attrs,
};

pub fn td0_struct_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let source: ItemStruct = parse2(input).expect("Error parsing input struct!");

    let mut output: TokenStream = td0_struct_gen_impl_default(&source);
    output.extend(td0_struct_gen_td0_chunk_item_impl(&source));
    output.extend(td0_struct_gen_from_u8_slice_impl(&source));
    output.into()
}

fn td0_field_validate_allowed_attrs(
    kv_list: &AttrKeyValueList,
    valid_names: &[&str],
) -> syn::Result<()> {
    let mut my_valid_names: Vec<&str> = Vec::from(valid_names);
    my_valid_names.insert(0, "field_type");
    validate_attr_list_valid_attrs(kv_list, &my_valid_names[..])
}

fn td0_field_validate_required_attrs(
    kv_list: &AttrKeyValueList,
    required_names: &[&str],
    source_attr: &Attribute,
) -> syn::Result<()> {
    let mut my_required_names: Vec<&str> = Vec::from(required_names);
    my_required_names.insert(0, "field_type");
    validate_attr_list_required_attrs(kv_list, &my_required_names[..], source_attr)
}

fn td0_field_parse_and_validate_kv_list(
    attr: &Attribute,
    required_names: &[&str],
    valid_names: &[&str],
) -> syn::Result<AttrKeyValueList> {
    let kv_list = parse_kv_list(attr)?;
    td0_field_validate_required_attrs(&kv_list, required_names, attr)?;
    td0_field_validate_allowed_attrs(&kv_list, valid_names)?;
    Ok(kv_list)
}

#[derive(Clone, Debug, PartialEq)]
struct TD0FieldTypeText {
    pub pad_byte: Option<u8>,
}

impl TryFrom<&Attribute> for TD0FieldTypeText {
    type Error = syn::Error;

    fn try_from(attr: &Attribute) -> Result<Self, <Self as TryFrom<&Attribute>>::Error> {
        static REQUIRED_NAMES: [&str; 0] = [];
        static VALID_NAMES: [&str; 1] = ["pad_byte"];

        let kv_list = td0_field_parse_and_validate_kv_list(attr, &REQUIRED_NAMES, &VALID_NAMES)?;
        Ok(Self {
            pad_byte: match kv_list.find_by_name("pad_byte") {
                Some(pair) => Some(pair.try_into()?),
                None => None,
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
struct TD0FieldTypeNone;

impl TryFrom<&Attribute> for TD0FieldTypeNone {
    type Error = syn::Error;

    fn try_from(attr: &Attribute) -> Result<Self, <Self as TryFrom<&Attribute>>::Error> {
        static REQUIRED_NAMES: [&str; 0] = [];
        static VALID_NAMES: [&str; 0] = [];

        let _ = td0_field_parse_and_validate_kv_list(attr, &REQUIRED_NAMES, &VALID_NAMES)?;
        Ok(Self {})
    }
}

#[derive(Clone, Debug, PartialEq)]
struct TD0FieldTypeEnumStr {
    collection: String,
}

impl TryFrom<&Attribute> for TD0FieldTypeEnumStr {
    type Error = syn::Error;

    fn try_from(attr: &Attribute) -> Result<Self, <Self as TryFrom<&Attribute>>::Error> {
        static REQUIRED_NAMES: [&str; 1] = ["collection"];
        static VALID_NAMES: [&str; 1] = ["collection"];

        let kv_list = td0_field_parse_and_validate_kv_list(attr, &REQUIRED_NAMES, &VALID_NAMES)?;
        Ok(Self {
            collection: kv_list
                .find_by_name("collection")
                .expect("Guarded above.")
                .try_into()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
struct TD0FieldTypeTD0Decimal {
    min: f32,
    max: f32,
}

impl TryFrom<&Attribute> for TD0FieldTypeTD0Decimal {
    type Error = syn::Error;

    fn try_from(attr: &Attribute) -> Result<Self, <Self as TryFrom<&Attribute>>::Error> {
        static REQUIRED_NAMES: [&str; 2] = ["min", "max"];
        static VALID_NAMES: [&str; 2] = ["min", "max"];

        let kv_list = td0_field_parse_and_validate_kv_list(attr, &REQUIRED_NAMES, &VALID_NAMES)?;
        let min: f32 = kv_list
            .find_by_name("min")
            .expect("Guarded above.")
            .try_into()?;
        let max: f32 = kv_list
            .find_by_name("max")
            .expect("Guarded above.")
            .try_into()?;
        Ok(Self { min, max })
    }
}

impl TD0FieldTypeTD0Decimal {
    pub fn clamp<T>(&self, val: T) -> Result<T, &'static str>
    where
        T: Ord + Bounded + TryFrom<i64>,
        f32: From<T>,
    {
        // Scale min and max to the native value scale (x10).
        let min: T = T::try_from(self.min.mul(10.0f32).round() as i64)
            .map_err(|_| "value must be convertable to field type.")?;
        let max: T = T::try_from(self.max.mul(10.0f32).round() as i64)
            .map_err(|_| "value must be convertable to field type.")?;

        Ok(num_traits::clamp(val, min, max))
    }
}

#[derive(Clone, Debug, PartialEq)]
struct TD0FieldTypeMaybeBoundedI8 {
    min: Option<i8>,
    max: Option<i8>,
}

impl TD0FieldTypeMaybeBoundedI8 {
    fn clamp(&self, val: i8) -> i8 {
        val.clamp(self.min.unwrap_or(i8::MIN), self.max.unwrap_or(i8::MAX))
    }
}

impl TryFrom<&Attribute> for TD0FieldTypeMaybeBoundedI8 {
    type Error = syn::Error;

    fn try_from(attr: &Attribute) -> Result<Self, <Self as TryFrom<&Attribute>>::Error> {
        static REQUIRED_NAMES: [&str; 0] = [];
        static VALID_NAMES: [&str; 2] = ["min", "max"];

        let kv_list = td0_field_parse_and_validate_kv_list(attr, &REQUIRED_NAMES, &VALID_NAMES)?;
        Ok(Self {
            min: match kv_list.find_by_name("min") {
                Some(pair) => Some(pair.try_into()?),
                None => None,
            },
            max: match kv_list.find_by_name("max") {
                Some(pair) => Some(pair.try_into()?),
                None => None,
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
struct TD0FieldTypeMaybeBoundedU8 {
    min: Option<u8>,
    max: Option<u8>,
}

impl TD0FieldTypeMaybeBoundedU8 {
    fn clamp(&self, val: u8) -> u8 {
        val.clamp(self.min.unwrap_or(u8::MIN), self.max.unwrap_or(u8::MAX))
    }
}

impl TryFrom<&Attribute> for TD0FieldTypeMaybeBoundedU8 {
    type Error = syn::Error;

    fn try_from(attr: &Attribute) -> Result<Self, <Self as TryFrom<&Attribute>>::Error> {
        static REQUIRED_NAMES: [&str; 0] = [];
        static VALID_NAMES: [&str; 2] = ["min", "max"];

        let kv_list = td0_field_parse_and_validate_kv_list(attr, &REQUIRED_NAMES, &VALID_NAMES)?;
        Ok(Self {
            min: match kv_list.find_by_name("min") {
                Some(pair) => Some(pair.try_into()?),
                None => None,
            },
            max: match kv_list.find_by_name("max") {
                Some(pair) => Some(pair.try_into()?),
                None => None,
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
struct TD0FieldTypeMaybeBoundedU16 {
    min: Option<u16>,
    max: Option<u16>,
}

impl TD0FieldTypeMaybeBoundedU16 {
    fn clamp(&self, val: u16) -> u16 {
        val.clamp(self.min.unwrap_or(u16::MIN), self.max.unwrap_or(u16::MAX))
    }
}

impl TryFrom<&Attribute> for TD0FieldTypeMaybeBoundedU16 {
    type Error = syn::Error;

    fn try_from(attr: &Attribute) -> Result<Self, <Self as TryFrom<&Attribute>>::Error> {
        static REQUIRED_NAMES: [&str; 0] = [];
        static VALID_NAMES: [&str; 2] = ["min", "max"];

        let kv_list = td0_field_parse_and_validate_kv_list(attr, &REQUIRED_NAMES, &VALID_NAMES)?;
        Ok(Self {
            min: match kv_list.find_by_name("min") {
                Some(pair) => Some(pair.try_into()?),
                None => None,
            },
            max: match kv_list.find_by_name("max") {
                Some(pair) => Some(pair.try_into()?),
                None => None,
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
struct TD0FieldTypeMaybeBoundedU32 {
    min: Option<u32>,
    max: Option<u32>,
}

impl TD0FieldTypeMaybeBoundedU32 {
    fn clamp(&self, val: u32) -> u32 {
        val.clamp(self.min.unwrap_or(u32::MIN), self.max.unwrap_or(u32::MAX))
    }
}

impl TryFrom<&Attribute> for TD0FieldTypeMaybeBoundedU32 {
    type Error = syn::Error;

    fn try_from(attr: &Attribute) -> Result<Self, <Self as TryFrom<&Attribute>>::Error> {
        static REQUIRED_NAMES: [&str; 0] = [];
        static VALID_NAMES: [&str; 2] = ["min", "max"];

        let kv_list = td0_field_parse_and_validate_kv_list(attr, &REQUIRED_NAMES, &VALID_NAMES)?;
        Ok(Self {
            min: match kv_list.find_by_name("min") {
                Some(pair) => Some(pair.try_into()?),
                None => None,
            },
            max: match kv_list.find_by_name("max") {
                Some(pair) => Some(pair.try_into()?),
                None => None,
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
struct TD0FieldTypeMaybeBoundedI16 {
    min: Option<i16>,
    max: Option<i16>,
}

impl TD0FieldTypeMaybeBoundedI16 {
    fn clamp(&self, val: i16) -> i16 {
        val.clamp(self.min.unwrap_or(i16::MIN), self.max.unwrap_or(i16::MAX))
    }
}

impl TryFrom<&Attribute> for TD0FieldTypeMaybeBoundedI16 {
    type Error = syn::Error;

    fn try_from(attr: &Attribute) -> Result<Self, <Self as TryFrom<&Attribute>>::Error> {
        static REQUIRED_NAMES: [&str; 0] = [];
        static VALID_NAMES: [&str; 2] = ["min", "max"];

        let kv_list = td0_field_parse_and_validate_kv_list(attr, &REQUIRED_NAMES, &VALID_NAMES)?;
        Ok(Self {
            min: match kv_list.find_by_name("min") {
                Some(pair) => Some(pair.try_into()?),
                None => None,
            },
            max: match kv_list.find_by_name("max") {
                Some(pair) => Some(pair.try_into()?),
                None => None,
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
enum TD0FieldType {
    EnumStr(TD0FieldTypeEnumStr),
    I16(TD0FieldTypeMaybeBoundedI16),
    I8(TD0FieldTypeMaybeBoundedI8),
    Slice(TD0FieldTypeNone),
    TD0Decimal(TD0FieldTypeTD0Decimal),
    Text(TD0FieldTypeText),
    U16(TD0FieldTypeMaybeBoundedU16),
    U32(TD0FieldTypeMaybeBoundedU32),
    U8(TD0FieldTypeMaybeBoundedU8),
    Volume(TD0FieldTypeNone),
}

#[derive(Clone, Debug, PartialEq)]
struct TD0Field {
    ident: Ident,
    field_type: TD0FieldType,
    field_type_name: String,
    native_type: NativeType,
}

impl TryFrom<&Field> for TD0Field {
    type Error = syn::Error;

    fn try_from(field: &Field) -> Result<Self, <Self as TryFrom<&Field>>::Error> {
        let attr = get_named_attr("td0_field", field)
            .ok_or(syn::Error::new_spanned(field, "Field is not a td0_field"))?
            .clone();
        let kv_list = parse_kv_list(&attr)?;
        td0_field_validate_required_attrs(&kv_list, &["field_type"], &attr)?;

        let ident = field
            .ident
            .clone()
            .ok_or(syn::Error::new_spanned(field, "Field can't be anonymous"))?;

        let field_type = kv_list.find_by_name("field_type").expect("Verified above.");
        let field_type_name: String = field_type.try_into()?;

        let parsed_field_attr: TD0FieldType = match field_type_name.as_str() {
            "EnumStr" => TD0FieldType::EnumStr(TD0FieldTypeEnumStr::try_from(&attr)?),
            "I16" => TD0FieldType::I16(TD0FieldTypeMaybeBoundedI16::try_from(&attr)?),
            "I8" => TD0FieldType::I8(TD0FieldTypeMaybeBoundedI8::try_from(&attr)?),
            "Slice" => TD0FieldType::Slice(TD0FieldTypeNone {}),
            "Text" => TD0FieldType::Text(TD0FieldTypeText::try_from(&attr)?),
            "TD0Decimal" => TD0FieldType::TD0Decimal(TD0FieldTypeTD0Decimal::try_from(&attr)?),
            "U16" => TD0FieldType::U16(TD0FieldTypeMaybeBoundedU16::try_from(&attr)?),
            "U32" => TD0FieldType::U32(TD0FieldTypeMaybeBoundedU32::try_from(&attr)?),
            "U8" => TD0FieldType::U8(TD0FieldTypeMaybeBoundedU8::try_from(&attr)?),
            "Volume" => TD0FieldType::Volume(TD0FieldTypeNone {}),
            _ => {
                return Err(syn::Error::new_spanned(
                    field_type.val(),
                    format!("Unknown field_type: `{field_type_name}`"),
                ));
            }
        };

        let Some(native_type) = get_field_native_type(field) else {
            return Err(syn::Error::new_spanned(
                field,
                "Unimplemented native field type.",
            ));
        };

        Ok(Self {
            ident,
            field_type: parsed_field_attr,
            field_type_name,
            native_type,
        })
    }
}

fn default_tokenstream_for_field_type(field: &Field) -> TokenStream {
    let mut output: TokenStream = TokenStream::new();
    let Some(native_type) = get_field_native_type(field) else {
        return syn::Error::new_spanned(field, "Unimplemented native field type.")
            .into_compile_error();
    };

    output.extend(match native_type {
        NativeType::I16 => {
            let i16_default: i16 = match TD0Field::try_from(field) {
                Ok(td0field) => match td0field.field_type {
                    TD0FieldType::I16(field_type) => field_type.clamp(0),
                    _ => 0,
                },
                Err(_) => 0,
            };
            quote!(I16::from(#i16_default))
        }
        NativeType::I8 => {
            let i8_default: i8 = match TD0Field::try_from(field) {
                Ok(td0field) => match td0field.field_type {
                    TD0FieldType::I8(field_type) => field_type.clamp(0),
                    _ => 0,
                },
                Err(_) => 0,
            };
            quote!(#i8_default)
        }
        NativeType::U8 => {
            let u8_default: u8 = match TD0Field::try_from(field) {
                Ok(td0field) => match td0field.field_type {
                    TD0FieldType::U8(field_type) => field_type.clamp(0),
                    _ => 0,
                },
                Err(_) => 0,
            };
            quote!(#u8_default)
        }
        NativeType::U16 => {
            let u16_default: u16 = match TD0Field::try_from(field) {
                Ok(td0field) => match td0field.field_type {
                    TD0FieldType::TD0Decimal(field_type) => match field_type.clamp(0u16) {
                        Ok(val) => val,
                        Err(err) => {
                            return syn::Error::new_spanned(field, err).into_compile_error();
                        }
                    },
                    TD0FieldType::U16(field_type) => field_type.clamp(0),
                    _ => 0,
                },
                Err(_) => 0,
            };
            quote!(U16::from(#u16_default))
        }
        NativeType::U32 => {
            let u32_default: u32 = match TD0Field::try_from(field) {
                Ok(td0field) => match td0field.field_type {
                    TD0FieldType::U32(field_type) => field_type.clamp(0),
                    _ => 0,
                },
                Err(_) => 0,
            };
            quote!(U32::from(#u32_default))
        }
        NativeType::Slice(slice) => {
            let size = slice.size;
            let slice_default: u8 = match TD0Field::try_from(field) {
                Ok(td0field) => match td0field.field_type {
                    TD0FieldType::Text(attr) => attr.pad_byte.unwrap_or(0),
                    _ => 0,
                },
                Err(_) => 0,
            };
            quote!([#slice_default; #size])
        }
    });

    output
}

fn td0_struct_gen_impl_default(source: &ItemStruct) -> TokenStream {
    let ident = &source.ident;
    let mut field_defs: Vec<TokenStream> = Vec::new();
    for field in &source.fields {
        let field_ident = field.ident.clone().expect("Fields must be named.");
        let default_val = default_tokenstream_for_field_type(field);
        field_defs.push(quote! {#field_ident: #default_val});
    }

    quote! {
        #[automatically_derived]
        impl core::default::Default for #ident {
            fn default() -> Self {
                Self {
                    #( #field_defs ),*
                }
            }
        }
    }
}

fn const_name_from_ident(val: &Ident) -> String {
    // NOTE: Assumes ASCII identifiers!
    let mut output: String = String::new();
    let val_string = val.to_string();
    let mut val_chars = val_string.chars();
    let mut last_char = val_chars.next().unwrap();

    output.push(last_char.to_ascii_uppercase());
    for char in val_chars {
        // Add underscores before upper-case chars to convert from
        // PascalCase or camelCase to CONST_CASE.
        //
        // If the previous char was also uppercase (e.g. TD0File,) omit the
        // underscore. (TD0FILE, not T_D_0FILE). Digits and symbols are effectively
        // uppercase for this function.
        if !char.is_lowercase() && last_char.is_lowercase() {
            output.push('_');
        }
        output.push(char.to_ascii_uppercase());
        last_char = char;
    }
    output
}

fn td0_struct_gen_from_u8_slice_impl(source: &ItemStruct) -> TokenStream {
    let source_ident = &source.ident;
    quote! {
        #[automatically_derived]
        impl core::convert::TryFrom<&[u8]> for #source_ident  {
            type Error = libtd0_core::result::TD0Error;

            fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
                use zerocopy::FromBytes;
                Self::read_from_bytes(bytes).map_err(|_| libtd0_core::result::TD0Error::InvalidChunkItem)
            }
        }
    }
}

fn td0_struct_gen_td0_chunk_item_impl(source: &ItemStruct) -> TokenStream {
    let mut field_tokenstreams: Vec<String> = Vec::new();
    let source_ident = &source.ident;
    let mut td0fields: Vec<TD0Field> = Vec::new();

    for field in source.fields.iter() {
        let field_ident = match &field.ident {
            Some(val) => val,
            None => {
                return syn::Error::new_spanned(field, "Anonymous fields are not supported.")
                    .into_compile_error();
            }
        };

        field_tokenstreams.push(field_ident.to_string());
        // TODO:  Make this more efficient:
        //        TD0Field.try_from(field) calls get_named_attr, which we're
        //        already call here and discarding the result.
        match get_named_attr("td0_field", field) {
            Some(_attr) => match TD0Field::try_from(field) {
                Ok(td0field) => td0fields.push(td0field),
                Err(err) => return err.into_compile_error(),
            },
            None => {
                // TODO: Can return a compile error here for struct fields that don't have
                //       the td0_field attr set, if wanted.
            }
        }
    }

    let num_fields: Literal = Literal::usize_unsuffixed(field_tokenstreams.len());
    let const_field_array_name = format_ident!("{}_FIELDS", const_name_from_ident(source_ident));

    let mut output: TokenStream = quote! {
        pub const #const_field_array_name: [&str; #num_fields] = [
            #( #field_tokenstreams ),*
        ];
    };

    let fn_field_options: ImplItemFn = build_chunkitem_field_options(&td0fields);
    let fn_field_value: ImplItemFn = build_chunkitem_field_value(&td0fields);
    let fn_set_field_value: ImplItemFn = build_chunkitem_set_field_value(&td0fields);
    let fn_field_value_raw: ImplItemFn = build_chunkitem_field_value_raw(&td0fields);
    let fn_set_field_value_raw: ImplItemFn = build_chunkitem_set_field_value_raw(&td0fields);
    let fn_field_type: ImplItemFn = build_chunkitem_field_type(&td0fields);

    // Create an impl with get_fields().
    output.extend(quote!(
        #[automatically_derived]
        impl libtd0_core::TD0ChunkItem for #source_ident {
            fn list_fields(&self) -> &'static [&'static str] { &#const_field_array_name }
            #fn_field_options
            #fn_field_value
            #fn_set_field_value
            #fn_field_value_raw
            #fn_set_field_value_raw
            #fn_field_type
            fn as_bytes(&self) -> &[u8] {
                zerocopy::IntoBytes::as_bytes(self)
            }
        }
    ));

    output
}

fn build_getter_expr(td0field: &TD0Field) -> Expr {
    let ident = &td0field.ident;
    match &td0field.field_type {
        TD0FieldType::Slice(_) => parse_quote!(
            Some(libtd0_core::TD0Value::Slice(Box::new(self.#ident.clone())))
        ),
        TD0FieldType::Text(attr) => {
            let pad_val: u8 = attr.pad_byte.unwrap_or(0);
            parse_quote!(Some(libtd0_core::TD0Value::text_from_u8_array(&self.#ident, &#pad_val)))
        }
        TD0FieldType::I16(_) => {
            parse_quote!(Some(libtd0_core::TD0Value::I16(self.#ident.get())))
        }
        TD0FieldType::U16(_) => {
            parse_quote!(Some(libtd0_core::TD0Value::U16(self.#ident.get())))
        }
        TD0FieldType::U32(_) => {
            parse_quote!(Some(libtd0_core::TD0Value::U32(self.#ident.get())))
        }
        TD0FieldType::U8(_) => parse_quote!(Some(libtd0_core::TD0Value::U8(self.#ident))),
        TD0FieldType::I8(_) => parse_quote!(Some(libtd0_core::TD0Value::I8(self.#ident))),
        TD0FieldType::TD0Decimal(_) => {
            parse_quote!(Some(
                libtd0_core::TD0Value::Decimal(f32::from(
                    libtd0_core::IntEncodedDecimal::from(self.#ident)
                ))
            ))
        }
        TD0FieldType::Volume(_) => parse_quote!(
            match libtd0_core::Volume::try_from(self.#ident.get()) {
                Ok(vol) => Some(libtd0_core::TD0Value::Decimal(vol.into())),
                Err(_) => None
            }
        ),
        TD0FieldType::EnumStr(attr) => {
            let collection_ident = format_ident!("{}", &attr.collection);
            parse_quote!(Some(libtd0_core::TD0Value::Text(String::from(
                *#collection_ident
                    .get(self.#ident as usize)
                    .unwrap_or_else(|| &"INVALID")
            ))))
        }
    }
}

fn build_getter_expr_raw(td0field: &TD0Field) -> Expr {
    let ident = &td0field.ident;
    match td0field.native_type {
        NativeType::I16 => parse_quote!(Some(libtd0_core::TD0ValueRaw::I16(self.#ident.get()))),
        NativeType::I8 => parse_quote!(Some(libtd0_core::TD0ValueRaw::I8(self.#ident))),
        NativeType::Slice(_) => {
            parse_quote!(Some(libtd0_core::TD0ValueRaw::Slice(Box::new(self.#ident.clone()))))
        }
        NativeType::U16 => parse_quote!(Some(libtd0_core::TD0ValueRaw::U16(self.#ident.get()))),
        NativeType::U32 => parse_quote!(Some(libtd0_core::TD0ValueRaw::U32(self.#ident.get()))),
        NativeType::U8 => parse_quote!(Some(libtd0_core::TD0ValueRaw::U8(self.#ident))),
    }
}

fn build_decimal_setter_expr(
    attr: &TD0FieldTypeTD0Decimal,
    native_type: &NativeType,
    ident: &Ident,
) -> Stmt {
    let attr_min: LitFloat = LitFloat::new(attr.min.to_string().as_str(), Span::call_site());
    let attr_max: LitFloat = LitFloat::new(attr.max.to_string().as_str(), Span::call_site());
    let min: Expr = parse_quote!(#attr_min as f32);
    let max: Expr = parse_quote!(#attr_max as f32);

    let rhand: Expr = match native_type {
        NativeType::U8 => parse_quote!(
            u8::try_from(libtd0_core::IntEncodedDecimal::from(*val))
                .map_err(|_| libtd0_core::result::TD0Error::OutOfRange)?
        ),
        NativeType::U16 => parse_quote!(
            U16::try_from(libtd0_core::IntEncodedDecimal::from(*val))
                .map_err(|_| libtd0_core::result::TD0Error::OutOfRange)?
        ),
        NativeType::I8 => parse_quote!(
            i8::try_from(libtd0_core::IntEncodedDecimal::from(*val))
                .map_err(|_| libtd0_core::result::TD0Error::OutOfRange)?
        ),
        NativeType::I16 => parse_quote!(
            I16::try_from(libtd0_core::IntEncodedDecimal::from(*val))
                .map_err(|_| libtd0_core::result::TD0Error::OutOfRange)?
        ),
        _ => parse_quote!(val.clone()),
    };

    parse_quote!(
        if libtd0_core::in_range_inclusive(libtd0_core::IntEncodedDecimal::from(*val).val(), Some(#min), Some(#max)) {
            self.#ident = #rhand;
        } else {
            return Err(libtd0_core::result::TD0Error::OutOfRange)
        }
    )
}

fn build_setter_expr(td0field: &TD0Field) -> Arm {
    let ident = &td0field.ident;
    let ident_str = ident.to_string();

    match &td0field.field_type {
        TD0FieldType::Text(attr) => {
            let pad_byte = attr.pad_byte.unwrap_or(0u8);
            parse_quote!(
                (#ident_str, libtd0_core::TD0Value::Text(val)) => {
                    libtd0_core::copy_ascii_str_to_u8_slice(val, &mut self.#ident, #pad_byte)
                }
            )
        }
        TD0FieldType::EnumStr(attr) => {
            let collection = format_ident!("{}", &attr.collection);
            parse_quote!(
                (#ident_str, libtd0_core::TD0Value::Text(val)) => {
                    let pos : &usize = &#collection.iter().position(|item| *item == val.as_str())
                        .ok_or_else(|| libtd0_core::result::TD0Error::InvalidInput)?;
                    self.#ident = u8::try_from(*pos).expect("Collection index must be in range.");
                    Ok(())
                }
            )
        }
        TD0FieldType::TD0Decimal(attr) => {
            let inner: Stmt = build_decimal_setter_expr(attr, &td0field.native_type, ident);

            parse_quote!(
                (#ident_str, libtd0_core::TD0Value::Decimal(val)) => {
                    #inner
                    Ok(())
                }
            )
        }
        TD0FieldType::Volume(_) => {
            parse_quote!(
                (#ident_str, libtd0_core::TD0Value::Decimal(val)) => {
                    let tmp = libtd0_core::Volume::try_from(*val)?;
                    self.#ident = tmp.try_into().map_err(|_| libtd0_core::result::TD0Error::OutOfRange)?;
                    Ok(())
                }
            )
        }
        TD0FieldType::Slice(_) => parse_quote!(
            (#ident_str, ..) => {
                Err(libtd0_core::result::TD0Error::ReadOnlyField)
            }
        ),
        TD0FieldType::I16(attr) => {
            let min_expr: Expr = match attr.min {
                Some(min_val) => parse_quote!(Some(#min_val)),
                None => parse_quote!(None),
            };

            let max_expr: Expr = match attr.max {
                Some(max_val) => parse_quote!(Some(#max_val)),
                None => parse_quote!(None),
            };

            parse_quote!(
                (#ident_str, libtd0_core::TD0Value::I16(val)) => {
                    if libtd0_core::in_range_inclusive(*val, #min_expr, #max_expr) {
                        self.#ident = val.clone().into();
                        Ok(())
                    }
                    else {
                        Err(libtd0_core::result::TD0Error::OutOfRange)
                    }
                }
            )
        }
        TD0FieldType::U16(attr) => {
            let min_expr: Expr = match attr.min {
                Some(min_val) => parse_quote!(Some(#min_val)),
                None => parse_quote!(None),
            };

            let max_expr: Expr = match attr.max {
                Some(max_val) => parse_quote!(Some(#max_val)),
                None => parse_quote!(None),
            };

            parse_quote!(
                (#ident_str, libtd0_core::TD0Value::U16(val)) => {
                    if libtd0_core::in_range_inclusive(*val, #min_expr, #max_expr) {
                        self.#ident = val.clone().into();
                        Ok(())
                    }
                    else {
                        Err(libtd0_core::result::TD0Error::OutOfRange)
                    }
                }
            )
        }
        TD0FieldType::U32(attr) => {
            let min_expr: Expr = match attr.min {
                Some(min_val) => parse_quote!(Some(#min_val)),
                None => parse_quote!(None),
            };

            let max_expr: Expr = match attr.max {
                Some(max_val) => parse_quote!(Some(#max_val)),
                None => parse_quote!(None),
            };

            parse_quote!(
                (#ident_str, libtd0_core::TD0Value::U32(val)) => {
                    if libtd0_core::in_range_inclusive(*val, #min_expr, #max_expr) {
                        self.#ident = val.clone().into();
                        Ok(())
                    }
                    else {
                        Err(libtd0_core::result::TD0Error::OutOfRange)
                    }
                }
            )
        }
        TD0FieldType::U8(attr) => {
            let min_expr: Expr = match attr.min {
                Some(min_val) => parse_quote!(Some(#min_val)),
                None => parse_quote!(None),
            };

            let max_expr: Expr = match attr.max {
                Some(max_val) => parse_quote!(Some(#max_val)),
                None => parse_quote!(None),
            };

            parse_quote!(
                (#ident_str, libtd0_core::TD0Value::U8(val)) => {
                    if libtd0_core::in_range_inclusive(*val, #min_expr, #max_expr) {
                        self.#ident = val.clone().into();
                        Ok(())
                    }
                    else {
                        Err(libtd0_core::result::TD0Error::OutOfRange)
                    }
                }
            )
        }
        TD0FieldType::I8(attr) => {
            let min_expr: Expr = match attr.min {
                Some(min_val) => parse_quote!(Some(#min_val)),
                None => parse_quote!(None),
            };

            let max_expr: Expr = match attr.max {
                Some(max_val) => parse_quote!(Some(#max_val)),
                None => parse_quote!(None),
            };

            parse_quote!(
                (#ident_str, libtd0_core::TD0Value::I8(val)) => {
                    if libtd0_core::in_range_inclusive(*val, #min_expr, #max_expr) {
                        self.#ident = val.clone().into();
                        Ok(())
                    }
                    else {
                        Err(libtd0_core::result::TD0Error::OutOfRange)
                    }
                }
            )
        }
    }
}

fn build_setter_expr_raw(td0field: &TD0Field) -> Arm {
    let ident = &td0field.ident;
    let ident_str = ident.to_string();

    match td0field.native_type {
        NativeType::I16 => {
            parse_quote!(
                (#ident_str, libtd0_core::TD0ValueRaw::I16(val)) => {
                    self.#ident = I16::from(val.clone());
                    Ok(())
                }
            )
        }
        NativeType::I8 => parse_quote!(
            (#ident_str, libtd0_core::TD0ValueRaw::I8(val)) => {
                self.#ident = val.clone();
                Ok(())
            }
        ),
        NativeType::U8 => parse_quote!(
            (#ident_str, libtd0_core::TD0ValueRaw::U8(val)) => {
                self.#ident = val.clone();
                Ok(())
            }
        ),
        NativeType::U16 => {
            parse_quote!(
                (#ident_str, libtd0_core::TD0ValueRaw::U16(val)) => {
                    self.#ident = U16::from(val.clone());
                    Ok(())
                }
            )
        }
        NativeType::U32 => parse_quote!(
            (#ident_str, libtd0_core::TD0ValueRaw::U32(val)) => {
                self.#ident = U32::from(val.clone());
                Ok(())
            }
        ),
        NativeType::Slice(_) => {
            let inner: Stmt = match &td0field.field_type {
                TD0FieldType::Text(attr) => {
                    let pad = attr.pad_byte.unwrap_or(0);
                    parse_quote!(libtd0_core::copy_slice_to_native_padded(val, &mut self.#ident, #pad)?;)
                }
                _ => {
                    parse_quote!(libtd0_core::copy_slice_to_native(val, &mut self.#ident)?;)
                }
            };
            parse_quote!(
                (#ident_str, libtd0_core::TD0ValueRaw::Slice(val)) => {
                    #inner
                    Ok(())
                }
            )
        }
    }
}

fn build_chunkitem_field_value(td0fields: &[TD0Field]) -> ImplItemFn {
    let mut fn_skel: ImplItemFn = parse_quote!(
        fn field_value(&self, field: &str) -> Option<libtd0_core::TD0Value> {
            match field {}
        }
    );

    let mut get_fields_clauses: Vec<Arm> = td0fields
        .iter()
        .map(|td0field| {
            let fld_name = td0field.ident.to_string();
            let expr = build_getter_expr(td0field);
            parse_quote!(#fld_name => #expr)
        })
        .collect();
    get_fields_clauses.push(parse_quote!(_ => None));

    for arm in get_fields_clauses.iter() {
        push_arm_to_fn_match(&mut fn_skel, arm);
    }
    fn_skel
}

fn build_chunkitem_field_value_raw(td0fields: &[TD0Field]) -> ImplItemFn {
    let mut fn_skel: ImplItemFn = parse_quote!(
        fn field_value_raw(&self, field: &str) -> Option<libtd0_core::TD0ValueRaw> {
            match field {}
        }
    );

    let mut get_fields_clauses: Vec<Arm> = td0fields
        .iter()
        .map(|td0field| {
            let fld_name = td0field.ident.to_string();
            let expr = build_getter_expr_raw(td0field);
            parse_quote!(#fld_name => #expr)
        })
        .collect();
    get_fields_clauses.push(parse_quote!(_ => None));

    for arm in get_fields_clauses.iter() {
        push_arm_to_fn_match(&mut fn_skel, arm);
    }
    fn_skel
}

fn build_chunkitem_set_field_value(td0fields: &[TD0Field]) -> ImplItemFn {
    let mut fn_skel: ImplItemFn = parse_quote!(
        fn set_field_value(
            &mut self,
            field: &str,
            value: &libtd0_core::TD0Value,
        ) -> libtd0_core::result::TD0Result<()> {
            match (field, value) {}
        }
    );

    let mut set_fields_clauses: Vec<Arm> = td0fields.iter().map(build_setter_expr).collect();
    set_fields_clauses.push(parse_quote!(
        _ => Err(libtd0_core::result::TD0Error::InvalidFieldOrType)
    ));

    for arm in set_fields_clauses.iter() {
        push_arm_to_fn_match(&mut fn_skel, arm);
    }
    fn_skel
}

fn build_chunkitem_set_field_value_raw(td0fields: &[TD0Field]) -> ImplItemFn {
    let mut fn_skel: ImplItemFn = parse_quote!(
        fn set_field_value_raw(
            &mut self,
            field: &str,
            value: &libtd0_core::TD0ValueRaw,
        ) -> libtd0_core::result::TD0Result<()> {
            match (field, value) {}
        }
    );

    let mut set_fields_clauses: Vec<Arm> = td0fields.iter().map(build_setter_expr_raw).collect();
    set_fields_clauses.push(parse_quote!(
        _ => Err(libtd0_core::result::TD0Error::InvalidInput)
    ));

    for arm in set_fields_clauses.iter() {
        push_arm_to_fn_match(&mut fn_skel, arm);
    }
    fn_skel
}

fn build_chunkitem_field_type(td0fields: &[TD0Field]) -> ImplItemFn {
    let arms: Vec<Arm> = td0fields
        .iter()
        .map(|field| {
            let field_name_str = field.ident.to_string();
            let field_type_str = &field.field_type_name;
            parse_quote!( #field_name_str => Some(#field_type_str) )
        })
        .collect();

    parse_quote!(
        fn field_type(&self, field: &str) -> Option<&'static str> {
            match field {
                #( #arms, )*
                _ => None
            }
        }
    )
}

fn build_chunkitem_field_options(td0fields: &[TD0Field]) -> ImplItemFn {
    let mut arms: Vec<Arm> = Vec::new();
    for field in td0fields.iter() {
        if let TD0FieldType::EnumStr(fld) = &field.field_type {
            let field_name_str = field.ident.to_string();
            let collection_name = format_ident!("{}", &fld.collection);
            arms.push(parse_quote!( #field_name_str => Some(&#collection_name) ))
        };
    }

    parse_quote!(
        fn field_options(&self, field: &str) -> Option<&'static [&'static str]> {
            match field {
                #( #arms, )*
                _ => None
            }
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_td0_f32_clamp() {
        let field = TD0FieldTypeTD0Decimal {
            min: -60.0f32,
            max: 6.0f32,
        };
        assert_eq!(
            field.clamp(-600i16),
            Ok(-600i16),
            "Doesn't clamp at min value."
        );
        assert_eq!(
            field.clamp(-601i16),
            Ok(-600i16),
            "Does clamp at < min value."
        );
        assert_eq!(field.clamp(60i16), Ok(60i16), "Doesn't clamp at max value.");
        assert_eq!(field.clamp(61i16), Ok(60i16), "Does clamp at > max value.");

        let field = TD0FieldTypeTD0Decimal {
            min: 2.3f32,
            max: 12.0f32,
        };
        assert_eq!(field.clamp(23i8), Ok(23i8), "Doesn't clamp at min value.");
        assert_eq!(field.clamp(22i8), Ok(23i8), "Does clamp at < min value.");
        assert_eq!(field.clamp(120i8), Ok(120i8), "Doesn't clamp at max value.");
        assert_eq!(field.clamp(121i8), Ok(120i8), "Does clamp at > max value.");
    }
}
