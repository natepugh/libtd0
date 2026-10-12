// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use core::convert::Into;
use std::collections::{HashMap, HashSet};

use num_traits::Bounded;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::spanned::Spanned as _;
use syn::{
    Arm, Attribute, Expr, Field, FieldValue, Fields, Ident, ImplGenerics, ImplItemFn, ItemStruct,
    Lit, Path, TypeGenerics, WhereClause, parse_macro_input, parse_quote, parse2,
};

use super::common::{
    AttrKeyValue, AttrKeyValueList, NativeType, const_name_from_ident, get_field_native_type,
    parse_kv_list, push_arm_to_fn_match,
};

const DECIMAL_REQUIRED: [&str; 2] = ["min", "max"];
const DECIMAL_VALID: [&str; 2] = ["min", "max"];
const ENUMSTR_REQUIRED: [&str; 1] = ["collection"];
const ENUMSTR_VALID: [&str; 1] = ["collection"];
const NUMBER_REQUIRED: [&str; 0] = [];
const NUMBER_VALID: [&str; 2] = ["min", "max"];
const SLICE_REQUIRED: [&str; 0] = [];
const SLICE_VALID: [&str; 0] = [];
const TEXT_REQUIRED: [&str; 0] = [];
const TEXT_VALID: [&str; 1] = ["pad_byte"];
const VOLUME_REQUIRED: [&str; 0] = [];
const VOLUME_VALID: [&str; 0] = [];

type AttrHashMap = HashMap<String, AttrKeyValue>;

// A field type that a user can declare in a field attribute of a TD0Struct.
#[derive(Debug)]
enum FieldType {
    EnumStr { collection: String },
    I8 { min: Option<i8>, max: Option<i8> },
    I16 { min: Option<i16>, max: Option<i16> },
    Slice,
    Text { pad_byte: Option<u8> },
    TD0Decimal { min: f32, max: f32 },
    U8 { min: Option<u8>, max: Option<u8> },
    U16 { min: Option<u16>, max: Option<u16> },
    U32 { min: Option<u32>, max: Option<u32> },
    Volume,
}

/// Metadata about a syn::ItemStruct, extracted for use in helper functions.
struct ItemStructMeta<'a> {
    pub ident: &'a Ident,
    pub impl_generics: ImplGenerics<'a>,
    pub type_generics: TypeGenerics<'a>,
    pub where_clause: Option<&'a WhereClause>,
    pub core_lib_path: &'a Path,
}

impl<'a> ItemStructMeta<'a> {
    fn new_from_struct(source: &'a ItemStruct, core_lib_path: &'a Path) -> Self {
        let (impl_generics, type_generics, where_clause) = source.generics.split_for_impl();
        Self {
            ident: &source.ident,
            impl_generics,
            type_generics,
            where_clause,
            core_lib_path,
        }
    }
}

struct TD0Field {
    pub ident: Ident,
    pub ident_str: String,
    pub field_type: FieldType,
    pub native_type: NativeType,
}

/// The derive macro that creates TD0ChunkItem and TD0ChunkItemMut implementations.
pub fn td0_struct_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //let input = TokenStream::from(input);
    let input = parse_macro_input!(input);
    let source: ItemStruct = parse2(input).expect("Error parsing input struct!");

    let core_lib_path: Path = match core_lib_path_from_struct(&source) {
        Ok(Some(path)) => path,
        Ok(None) => parse_quote!(td0_core),
        Err(error) => {
            return error.to_compile_error().into();
        }
    };

    let source_clone = source.clone();
    let source_meta: ItemStructMeta =
        ItemStructMeta::new_from_struct(&source_clone, &core_lib_path);

    let fields = match parse_struct_fields(source) {
        Ok(parsed) => parsed,
        Err(errors) => return errors.to_compile_error().into(),
    };

    let mut output: TokenStream = TokenStream::new();
    match build_impl_td0_chunkitem(&fields, &source_meta) {
        Ok(impl_stream) => output.extend(impl_stream),
        Err(error) => return error.to_compile_error().into(),
    }
    output.extend(build_trait_chunkitem_set_value_helper(
        &source_clone.ident,
        &core_lib_path,
    ));
    output.extend(build_impl_td0_chunkitem_set_value_helper(&source_meta));
    match build_impl_default(&fields, &source_meta) {
        Ok(impl_stream) => output.extend(impl_stream),
        Err(error) => return error.to_compile_error().into(),
    }
    //let output = TokenStream::new();
    output.into()
}

fn build_fn_chunkitem_field_options(fields: &[TD0Field]) -> ImplItemFn {
    let field_match_arms: Vec<Arm> = fields
        .iter()
        .filter_map(|field| match &field.field_type {
            FieldType::EnumStr { collection } => {
                let collection_ident: Ident = format_ident!("{collection}");
                let field: &String = &field.ident_str;
                Some(parse_quote!(#field => Some(&#collection_ident)))
            }
            _ => None,
        })
        .collect();

    if field_match_arms.is_empty() {
        parse_quote!(
            fn field_options(&self, field: &str) -> Option<&'static [&'static str]> {
                None
            }
        )
    } else {
        parse_quote!(
            fn field_options(&self, field: &str) ->  Option<&'static [&'static str]> {
                match field {
                    #( #field_match_arms, )*
                    _ => None
                }
            }
        )
    }
}

fn build_bounded_setter_expr<T>(
    min: Option<T>,
    max: Option<T>,
    field_ident: &Ident,
    core_lib_path: &Path,
) -> Expr
where
    T: Bounded + ToTokens + Copy,
{
    let min_expr: Expr = match min {
        Some(min_val) => parse_quote!(Some(#min_val)),
        None => parse_quote!(None),
    };

    let max_expr: Expr = match max {
        Some(max_val) => parse_quote!(Some(#max_val)),
        None => parse_quote!(None),
    };

    let min_val: Expr = match min {
        Some(val) => parse_quote!(#val),
        None => parse_quote!(0),
    };

    let max_val: Expr = match max {
        Some(val) => parse_quote!(#val),
        None => parse_quote!(i16::MAX),
    };

    parse_quote!(
        if #core_lib_path::in_range_inclusive(val, #min_expr, #max_expr) {
            self.#field_ident = val.clone().into();
            Ok(())
        }
        else {
            Err(#core_lib_path::TD0Error::out_of_range_error(#min_val, #max_val))
        }
    )
}

fn build_clamp_vals<T>(min: &Option<T>, max: &Option<T>) -> (Expr, Expr)
where
    T: Ord + Bounded + quote::ToTokens,
{
    (
        match min {
            Some(val) => parse_quote!(Some(#val)),
            None => parse_quote!(None),
        },
        match max {
            Some(val) => parse_quote!(Some(#val)),
            None => parse_quote!(None),
        },
    )
}

fn build_default_fieldvalue_for_field(
    field: &TD0Field,
    core_lib_path: &Path,
) -> syn::Result<FieldValue> {
    let ident = &field.ident;
    match (&field.field_type, &field.native_type) {
        (FieldType::I8 { min, max }, NativeType::I8) => {
            let (min_expr, max_expr) = build_clamp_vals(min, max);
            Ok(parse_quote!(#ident: #core_lib_path::clamp_optional(0i8, #min_expr, #max_expr)))
        }
        (FieldType::I16 { min, max }, NativeType::I16) => {
            let (min_expr, max_expr) = build_clamp_vals(min, max);
            Ok(
                parse_quote!(#ident: I16::from(#core_lib_path::clamp_optional(0i16, #min_expr, #max_expr))),
            )
        }
        (FieldType::Volume, NativeType::I16) => Ok(parse_quote!(#ident: I16::from(0))),
        (FieldType::Text { pad_byte }, NativeType::Slice(slice)) => {
            let pad_byte = pad_byte.unwrap_or(0);
            let size = slice.size;
            Ok(parse_quote!(#ident: [#pad_byte; #size]))
        }
        (FieldType::Slice, NativeType::Slice(slice)) => {
            let size = slice.size;
            Ok(parse_quote!(#ident: [0u8; #size]))
        }
        (FieldType::EnumStr { .. }, NativeType::U8) => Ok(parse_quote!(#ident: 0)),
        (FieldType::U8 { min, max }, NativeType::U8) => {
            let (min_expr, max_expr) = build_clamp_vals(min, max);
            Ok(parse_quote!(#ident: #core_lib_path::clamp_optional(0u8, #min_expr, #max_expr)))
        }
        (FieldType::U16 { min, max }, NativeType::U16) => {
            let (min_expr, max_expr) = build_clamp_vals(min, max);
            Ok(
                parse_quote!(#ident: U16::from(#core_lib_path::clamp_optional(0u16, #min_expr, #max_expr))),
            )
        }
        (FieldType::U32 { min, max }, NativeType::U32) => {
            let (min_expr, max_expr) = build_clamp_vals(min, max);
            Ok(
                parse_quote!(#ident: U32::from(#core_lib_path::clamp_optional(0u32, #min_expr, #max_expr))),
            )
        }
        (FieldType::TD0Decimal { min, max }, NativeType::U16) => {
            let default = 0f32.clamp(*min, *max);
            Ok(
                parse_quote!(#ident: U16::try_from(#core_lib_path::IntEncodedDecimal::from(#default)).unwrap_or(U16::from(0))),
            )
        }
        (FieldType::TD0Decimal { min, max }, NativeType::U8) => {
            let default = 0f32.clamp(*min, *max);
            Ok(
                parse_quote!(#ident: u8::try_from(#core_lib_path::IntEncodedDecimal::from(#default)).unwrap_or(0u8)),
            )
        }
        _ => Err(syn::Error::new_spanned(
            &field.ident,
            format!(
                "unexpected field type: `{:?}` with native type `{:?}`",
                field.field_type, field.native_type
            ),
        )),
    }
}

fn build_fn_chunkitem_field_type(fields: &[TD0Field]) -> ImplItemFn {
    let arms: Vec<Arm> = fields
        .iter()
        .map(|field| {
            let field_name = &field.ident_str;
            let field_type = field_type_pretty_name(field);
            parse_quote!( #field_name => Some(#field_type) )
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

fn build_fn_chunkitem_field_value(fields: &[TD0Field], core_lib_path: &Path) -> ImplItemFn {
    let get_fields_clauses: Vec<Arm> = fields
        .iter()
        .map(|field| {
            let fld_name = field.ident.to_string();
            let expr = build_getter_expr(field, core_lib_path);
            parse_quote!(#fld_name => #expr)
        })
        .collect();

    parse_quote!(
        fn field_value(&self, field: &str) -> Option<#core_lib_path::TD0Value> {
            match field {
                #( #get_fields_clauses, )*
                _ => None
            }
        }
    )
}

fn build_fn_chunkitem_field_value_raw(fields: &[TD0Field], core_lib_path: &Path) -> ImplItemFn {
    let get_fields_clauses: Vec<Arm> = fields
        .iter()
        .map(|field| {
            let fld_name = field.ident.to_string();
            let expr = build_getter_expr_raw(field, core_lib_path);
            parse_quote!(#fld_name => #expr)
        })
        .collect();

    parse_quote!(
        fn field_value_raw(&self, field: &str) -> Option<#core_lib_path::TD0ValueRaw> {
            match field {
                #( #get_fields_clauses, )*
                _ => None
            }
        }
    )
}

fn build_fn_chunkitem_set_field_value(
    fields: &[TD0Field],
    core_lib_path: &Path,
) -> syn::Result<ImplItemFn> {
    // Create a series of match arms, representing each valid (field, TD0ValueType) combo.
    let mut set_fields_clauses: Vec<Arm> = Vec::new();
    let mut field_errors: Option<syn::Error> = None;

    for field in fields.iter() {
        match build_setter_expr(field, core_lib_path) {
            Ok(arm) => set_fields_clauses.push(arm),
            Err(field_error) => match field_errors {
                Some(ref mut existing) => existing.combine(field_error),
                None => field_errors = Some(field_error),
            },
        }
    }

    if let Some(errors) = field_errors {
        return Err(errors);
    }

    // Create DataType errors for all fields. ("field1" | "field2", _) ... for all fields.
    set_fields_clauses.extend(build_setter_data_type_errors(
        fields,
        core_lib_path,
        field_type_pretty_name,
    ));
    // Any remaining requested fields are invalid.
    set_fields_clauses.push(parse_quote!(
        _ => Err(#core_lib_path::TD0Error::InvalidField(field.to_string()))
    ));

    let mut fn_skel: ImplItemFn = parse_quote!(
        fn set_field_value(
            &mut self,
            field: &str,
            value: #core_lib_path::TD0Value,
        ) -> #core_lib_path::TD0Result<()> {
            match (field, value) { }
        }
    );

    // NOTE TO FUTURE READERS: You might think to yourself, "I'll remove this loop and use
    // syn's quote interpolation instead."  Save yourself the headaches- don't do it. The
    // macro errors produced will be the least helpful errors you've ever enountered, too.
    for arm in set_fields_clauses {
        push_arm_to_fn_match(&mut fn_skel, &arm);
    }

    Ok(fn_skel)
}

fn build_fn_chunkitem_set_field_value_raw(
    fields: &[TD0Field],
    core_lib_path: &Path,
) -> syn::Result<ImplItemFn> {
    let mut set_fields_clauses: Vec<Arm> = Vec::new();
    let mut field_errors: Option<syn::Error> = None;

    for field in fields.iter() {
        match build_setter_expr_raw(field, core_lib_path) {
            Ok(arm) => set_fields_clauses.push(arm),
            Err(field_error) => match field_errors {
                Some(ref mut existing) => existing.combine(field_error),
                None => field_errors = Some(field_error),
            },
        }
    }

    if let Some(errors) = field_errors {
        return Err(errors);
    }

    // Create DataType errors for all fields. ("field1" | "field2", _) ... for all fields.
    set_fields_clauses.extend(build_setter_data_type_errors(
        fields,
        core_lib_path,
        field_type_pretty_name_raw,
    ));
    // Any remaining requested fields are invalid.
    set_fields_clauses.push(parse_quote!(
        _ => Err(#core_lib_path::TD0Error::InvalidField(field.to_string()))
    ));

    let mut fn_skel: ImplItemFn = parse_quote!(
        fn set_field_value_raw(
            &mut self,
            field: &str,
            value: #core_lib_path::TD0ValueRaw,
        ) -> #core_lib_path::TD0Result<()> {
            match (field, value) { }
        }
    );

    // NOTE TO FUTURE READERS: You might think to yourself, "I'll remove this loop and use
    // syn's quote interpolation instead."  Save yourself the headaches- don't do it. The
    // macro errors produced will be the least helpful errors you've ever enountered, too.
    for arm in set_fields_clauses {
        push_arm_to_fn_match(&mut fn_skel, &arm);
    }

    Ok(fn_skel)
}

fn build_getter_expr(field: &TD0Field, core_lib_path: &Path) -> Expr {
    let ident = &field.ident;
    match &field.field_type {
        FieldType::Slice => parse_quote!(
            Some(#core_lib_path::TD0Value::Slice(Box::new(self.#ident.clone())))
        ),
        FieldType::Text { pad_byte } => {
            let pad_val: u8 = pad_byte.unwrap_or(0);
            parse_quote!(Some(#core_lib_path::TD0Value::text_from_u8_array(&self.#ident, &#pad_val)))
        }
        FieldType::I16 { .. } => {
            parse_quote!(Some(#core_lib_path::TD0Value::I16(self.#ident.get())))
        }
        FieldType::U16 { .. } => {
            parse_quote!(Some(#core_lib_path::TD0Value::U16(self.#ident.get())))
        }
        FieldType::U32 { .. } => {
            parse_quote!(Some(#core_lib_path::TD0Value::U32(self.#ident.get())))
        }
        FieldType::U8 { .. } => parse_quote!(Some(#core_lib_path::TD0Value::U8(self.#ident))),
        FieldType::I8 { .. } => parse_quote!(Some(#core_lib_path::TD0Value::I8(self.#ident))),
        FieldType::TD0Decimal { .. } => {
            parse_quote!(Some(
                #core_lib_path::TD0Value::Decimal(f32::from(
                    #core_lib_path::IntEncodedDecimal::from(self.#ident)
                ))
            ))
        }
        FieldType::Volume => parse_quote!(
            match #core_lib_path::Volume::try_from(self.#ident.get()) {
                Ok(vol) => Some(#core_lib_path::TD0Value::Decimal(vol.into())),
                Err{..} => None
            }
        ),
        FieldType::EnumStr { collection } => {
            let collection_ident = format_ident!("{collection}");
            parse_quote!({
                Some(#core_lib_path::TD0Value::Text(String::from(
                    *#collection_ident.get(usize::from(self.#ident)).unwrap_or(&"INVALID")
                )))
            })
        }
    }
}

fn build_getter_expr_raw(field: &TD0Field, core_lib_path: &Path) -> Expr {
    let ident = &field.ident;
    match field.native_type {
        NativeType::I16 => parse_quote!(Some(#core_lib_path::TD0ValueRaw::I16(self.#ident.get()))),
        NativeType::I8 => parse_quote!(Some(#core_lib_path::TD0ValueRaw::I8(self.#ident))),
        NativeType::Slice(_) => {
            parse_quote!(Some(#core_lib_path::TD0ValueRaw::Slice(Box::new(self.#ident.clone()))))
        }
        NativeType::U16 => parse_quote!(Some(#core_lib_path::TD0ValueRaw::U16(self.#ident.get()))),
        NativeType::U32 => parse_quote!(Some(#core_lib_path::TD0ValueRaw::U32(self.#ident.get()))),
        NativeType::U8 => parse_quote!(Some(#core_lib_path::TD0ValueRaw::U8(self.#ident))),
    }
}

fn build_impl_default(
    fields: &[TD0Field],
    source_meta: &ItemStructMeta,
) -> syn::Result<proc_macro2::TokenStream> {
    let (ident, core_lib_path) = (source_meta.ident, source_meta.core_lib_path);
    let (impl_generics, type_generics, where_clause) = (
        &source_meta.impl_generics,
        &source_meta.type_generics,
        source_meta.where_clause,
    );

    let mut field_value_pairs: Vec<FieldValue> = Vec::new();
    let mut field_value_errors: Option<syn::Error> = None;

    for field in fields.iter() {
        match build_default_fieldvalue_for_field(field, core_lib_path) {
            Ok(field_value) => field_value_pairs.push(field_value),
            Err(err) => match field_value_errors {
                Some(ref mut existing) => existing.combine(err),
                None => field_value_errors = Some(err),
            },
        }
    }

    if let Some(errors) = field_value_errors {
        return Err(errors);
    }

    Ok(quote!(
        #[automatically_derived]
        impl #impl_generics core::default::Default for #ident #type_generics #where_clause {
            fn default() -> Self {
                Self {
                    #( #field_value_pairs ),*
                }
            }
        }
    ))
}

fn build_impl_td0_chunkitem(
    fields: &[TD0Field],
    source_meta: &ItemStructMeta,
) -> syn::Result<proc_macro2::TokenStream> {
    let (ident, core_lib_path) = (source_meta.ident, source_meta.core_lib_path);
    let (impl_generics, type_generics, where_clause) = (
        &source_meta.impl_generics,
        &source_meta.type_generics,
        source_meta.where_clause,
    );
    let fn_field_options: ImplItemFn = build_fn_chunkitem_field_options(fields);
    let fn_field_type: ImplItemFn = build_fn_chunkitem_field_type(fields);
    let fn_field_value: ImplItemFn =
        build_fn_chunkitem_field_value(fields, source_meta.core_lib_path);
    let fn_field_value_raw: ImplItemFn =
        build_fn_chunkitem_field_value_raw(fields, source_meta.core_lib_path);

    let fn_set_field_value: ImplItemFn =
        build_fn_chunkitem_set_field_value(fields, source_meta.core_lib_path)?;
    let fn_set_field_value_raw =
        build_fn_chunkitem_set_field_value_raw(fields, source_meta.core_lib_path)?;

    let field_names: Vec<&String> = fields.iter().map(|field| &field.ident_str).collect();
    let num_fields: usize = field_names.len();
    let field_names_array = format_ident!("{}_FIELDS", const_name_from_ident(ident));

    Ok(quote! {
        const #field_names_array : [&str; #num_fields] = [ #( #field_names, )* ];

        #[automatically_derived]
        impl #impl_generics #core_lib_path::TD0ChunkItem for #ident #type_generics #where_clause {
            fn as_bytes(&self) -> &[u8] {
                zerocopy::IntoBytes::as_bytes(self)
            }
            #fn_field_options
            #fn_field_type
            #fn_field_value
            #fn_field_value_raw
            #fn_set_field_value
            #fn_set_field_value_raw
            fn list_fields(&self) -> &'static [&'static str] { &#field_names_array }
        }
    })
}

fn build_impl_td0_chunkitem_set_value_helper(source_meta: &ItemStructMeta) -> TokenStream {
    let (ident, core_lib_path) = (source_meta.ident, source_meta.core_lib_path);
    let (impl_generics, type_generics, where_clause) = (
        &source_meta.impl_generics,
        &source_meta.type_generics,
        source_meta.where_clause,
    );
    let trait_name = format_ident!("TD0ChunkItemMutDeriveHelper{ident}");

    parse_quote!(
        #[automatically_derived]
        impl #impl_generics #trait_name for #ident #type_generics #where_clause {
            fn set_volume_field_i16<T>(field: &mut zerocopy::I16<T>, val: f32) -> #core_lib_path::TD0Result<()>
                where T: zerocopy::ByteOrder
            {
                let tmp = #core_lib_path::Volume::try_from(val)?;
                *field = tmp.try_into().map_err(|_|
                    #core_lib_path::TD0Error::out_of_range_error_decimal(#core_lib_path::Volume::MIN, #core_lib_path::Volume::MAX)
                )?;
                Ok(())
            }

            fn set_enumstr_field(field: &mut u8, val: &str, field_name: &str, collection: &[&str]) -> #core_lib_path::TD0Result<()> {
                let mut index = collection.iter().position(|item| *item == val)
                    .ok_or_else(|| #core_lib_path::TD0Error::InvalidInput(format!("ah that sucks")))?;
                *field = u8::try_from(index).expect("EnumStr indexes are all valid u8 values.");
                Ok(())
            }

            fn set_decimal_field_u16<T>(field: &mut zerocopy::U16<T>, val: f32, min: f32, max: f32) -> #core_lib_path::TD0Result<()>
                where T: zerocopy::ByteOrder
            {
                if #core_lib_path::in_range_inclusive(val, Some(min), Some(max)) {
                    *field = zerocopy::U16::try_from(#core_lib_path::IntEncodedDecimal::from(val))
                        .map_err(|_| #core_lib_path::TD0Error::out_of_range_error_decimal(min, max))?;
                    Ok(())
                } else {
                    Err(#core_lib_path::TD0Error::out_of_range_error_decimal(min, max))
                }
            }

            fn set_decimal_field_u8(field: &mut u8, val: f32, min: f32, max: f32) -> #core_lib_path::TD0Result<()> {
                if #core_lib_path::in_range_inclusive(val, Some(min), Some(max)) {
                    *field = u8::try_from(#core_lib_path::IntEncodedDecimal::from(val))
                        .map_err(|_| #core_lib_path::TD0Error::out_of_range_error_decimal(min, max))?;
                    Ok(())
                } else {
                    Err(#core_lib_path::TD0Error::out_of_range_error_decimal(min, max))
                }
            }

        }
    )
}

fn build_invalid_field_combo_compile_error(field: &TD0Field) -> syn::Error {
    syn::Error::new_spanned(
        &field.ident,
        format!(
            "invalid field type '{:?}' for native type '{:?}'",
            field.field_type, field.native_type
        ),
    )
}

fn build_setter_data_type_errors(
    fields: &[TD0Field],
    core_lib_path: &Path,
    pretty_field_name_fn: fn(&TD0Field) -> &str,
) -> Vec<Arm> {
    // Build match arms to create "invalid data type" errors for all fields at once.
    // The match arms are consolidated, e.g. ("field1" | "field2", _)  => {Err(..)}.
    let mut field_names_by_pretty_name: HashMap<&str, Vec<&str>> = HashMap::new();
    for field in fields.iter() {
        field_names_by_pretty_name
            .entry(pretty_field_name_fn(field))
            .or_default()
            .push(&field.ident_str);
    }

    let mut arms: Vec<Arm> = Vec::new();
    for (&expected_type, field_names) in field_names_by_pretty_name.iter() {
        // let fields_str = field_names.join(" | ");
        let err_str =
            format!("invalid data type setting field '{{field}}'. Expected `{expected_type}`");
        // NOTE: This particular syntax joins field_names (which will parse as "quoted_strings") with
        //       the pipe operator: #(#field_names)|*
        //       See: https://docs.rs/quote/1.0.47/quote/macro.quote.html#interpolation
        arms.push(
            parse_quote!((#(#field_names)|*, _) => Err(#core_lib_path::TD0Error::DataType(format!(#err_str))))
        );
    }
    arms
}

fn build_setter_expr(field: &TD0Field, core_lib_path: &Path) -> syn::Result<Arm> {
    let ident = &field.ident;
    let ident_str = &field.ident_str;

    match (&field.field_type, &field.native_type) {
        (FieldType::TD0Decimal { min, max }, NativeType::U8) => Ok(parse_quote!(
            (#ident_str, #core_lib_path::TD0Value::Decimal(val)) => Self::set_decimal_field_u8(&mut self.#ident, val, #min, #max)
        )),
        (FieldType::TD0Decimal { min, max }, NativeType::U16) => Ok(parse_quote!(
            (#ident_str, #core_lib_path::TD0Value::Decimal(val)) => Self::set_decimal_field_u16(&mut self.#ident, val, #min, #max)
        )),
        (FieldType::U32 { min, max }, NativeType::U32) => {
            let expr: Expr = if min.is_none() && max.is_none() {
                parse_quote!({ self.#ident = val.into(); Ok(()) })
            } else {
                build_bounded_setter_expr(*min, *max, ident, core_lib_path)
            };

            Ok(parse_quote!(
                (#ident_str, #core_lib_path::TD0Value::U32(val)) => #expr,
            ))
        }
        (FieldType::U16 { min, max }, NativeType::U16) => {
            let expr: Expr = if min.is_none() && max.is_none() {
                parse_quote!({ self.#ident = val.into(); Ok(()) })
            } else {
                build_bounded_setter_expr(*min, *max, ident, core_lib_path)
            };

            Ok(parse_quote!(
                (#ident_str, #core_lib_path::TD0Value::U16(val)) => #expr,
            ))
        }
        (FieldType::U8 { min, max }, NativeType::U8) => {
            let expr: Expr = if min.is_none() && max.is_none() {
                parse_quote!({ self.#ident = val.into(); Ok(()) })
            } else {
                build_bounded_setter_expr(*min, *max, ident, core_lib_path)
            };

            Ok(parse_quote!( (#ident_str, #core_lib_path::TD0Value::U8(val)) => #expr,))
        }
        (FieldType::I8 { min, max }, NativeType::I8) => {
            let expr: Expr = if min.is_none() && max.is_none() {
                parse_quote!({ self.#ident = val.into(); Ok(()) })
            } else {
                build_bounded_setter_expr(*min, *max, ident, core_lib_path)
            };

            Ok(parse_quote!( (#ident_str, #core_lib_path::TD0Value::I8(val)) => #expr,))
        }
        (FieldType::I16 { min, max }, NativeType::I16) => {
            let expr: Expr = if min.is_none() && max.is_none() {
                parse_quote!({ self.#ident = val.into(); Ok(()) })
            } else {
                build_bounded_setter_expr(*min, *max, ident, core_lib_path)
            };

            Ok(parse_quote!(
                (#ident_str, #core_lib_path::TD0Value::I16(val)) => #expr,
            ))
        }
        (FieldType::EnumStr { collection }, NativeType::U8) => {
            let collection_ident: Ident = format_ident!("{collection}");
            Ok(parse_quote!(
                (#ident_str, #core_lib_path::TD0Value::Text(val)) => {
                    Self::set_enumstr_field(&mut self.#ident, &val, #ident_str, &#collection_ident)
                }
            ))
        }
        (FieldType::Slice, NativeType::Slice(_)) => Ok(parse_quote!(
            (#ident_str, #core_lib_path::TD0Value::Slice(_)) => Err(#core_lib_path::TD0Error::ReadOnlyField(#ident_str.to_string())),
        )),
        (FieldType::Text { pad_byte }, NativeType::Slice(_)) => {
            let text_pad_byte = pad_byte.unwrap_or(0u8);
            Ok(parse_quote!(
                (#ident_str, #core_lib_path::TD0Value::Text(val)) => {
                    #core_lib_path::copy_ascii_str_to_u8_slice(val.as_str(), &mut self.#ident, #text_pad_byte)?;
                    Ok(())
                }
            ))
        }
        (FieldType::Volume, NativeType::I16) => Ok(parse_quote!(
            (#ident_str, #core_lib_path::TD0Value::Decimal(val)) => Self::set_volume_field_i16(&mut self.#ident, val)
        )),
        _ => Err(build_invalid_field_combo_compile_error(field)),
    }
}

fn build_setter_expr_raw(field: &TD0Field, core_lib_path: &Path) -> syn::Result<Arm> {
    let ident = &field.ident;
    let ident_str = &field.ident_str;

    match &field.native_type {
        NativeType::I8 => Ok(parse_quote!(
            (#ident_str, #core_lib_path::TD0ValueRaw::I8(val)) => { self.#ident = val; Ok(())}
        )),
        NativeType::U8 => Ok(parse_quote!(
            (#ident_str, #core_lib_path::TD0ValueRaw::U8(val)) => { self.#ident = val; Ok(())}
        )),
        NativeType::I16 => Ok(parse_quote!(
            (#ident_str, #core_lib_path::TD0ValueRaw::I16(val)) => { self.#ident = I16::from(val); Ok(())}
        )),
        NativeType::U16 => Ok(parse_quote!(
            (#ident_str, #core_lib_path::TD0ValueRaw::U16(val)) => { self.#ident = U16::from(val); Ok(())}
        )),
        NativeType::U32 => Ok(parse_quote!(
            (#ident_str, #core_lib_path::TD0ValueRaw::U32(val)) => { self.#ident = U32::from(val); Ok(())}
        )),
        NativeType::Slice(_) => Ok(parse_quote!(
            (#ident_str, #core_lib_path::TD0ValueRaw::Slice(val)) => {
                #core_lib_path::copy_slice_to_native(&*val, &mut self.#ident)?;
                Ok(())
            }
        )),
    }
}

fn build_trait_chunkitem_set_value_helper(
    source_ident: &Ident,
    core_lib_path: &Path,
) -> TokenStream {
    let name = format_ident!("TD0ChunkItemMutDeriveHelper{}", source_ident);

    parse_quote!(
        trait #name {
            fn set_volume_field_i16<T>(field: &mut zerocopy::I16<T>, val: f32) -> #core_lib_path::TD0Result<()>
                where T: zerocopy::ByteOrder;

            fn set_enumstr_field(field: &mut u8, val: &str, field_name: &str, collection: &[&str]) -> #core_lib_path::TD0Result<()>;

            fn set_decimal_field_u8(field: &mut u8, val: f32, min: f32, max: f32) -> #core_lib_path::TD0Result<()>;

            fn set_decimal_field_u16<T>(field: &mut zerocopy::U16<T>, val: f32, min: f32, max: f32) -> #core_lib_path::TD0Result<()>
                where T: zerocopy::ByteOrder;
        }
    )
}

fn core_lib_path_from_struct(source: &ItemStruct) -> syn::Result<Option<Path>> {
    let mut lib_path: Option<Path> = None;

    // Loop through all attributes on the struct
    for attr in &source.attrs {
        if attr.path().is_ident("td0_struct") {
            let kv_map = hashmap_from_kv_list(parse_kv_list(attr)?);
            validate_field_type_keys(
                &kv_map,
                &["td0_core_crate"],
                &["td0_core_crate"],
                attr.span(),
            )?;
            if let Lit::Str(path_str) = kv_map.get("td0_core_crate").unwrap().val() {
                match path_str.parse::<Path>() {
                    Ok(pathobj) => lib_path = Some(pathobj),
                    Err(_) => {
                        return Err(syn::Error::new_spanned(
                            attr,
                            "invalid path (should look like crate::module",
                        ));
                    }
                }
            } else {
                return Err(syn::Error::new_spanned(
                    attr,
                    "td0_core_crate = \"value\" must be a string.",
                ));
            }
        }
    }
    Ok(lib_path)
}

fn field_type_from_attrs(kv_map: &AttrHashMap, attr_span: Span) -> syn::Result<FieldType> {
    let Some(field_type_kv) = kv_map.get("field_type") else {
        return Err(syn::Error::new(attr_span, "field_type is required."));
    };
    match field_type_kv
        .val()
        .to_token_stream()
        .to_string()
        .replace("\"", "")
        .as_str()
    {
        "U8" => {
            validate_field_type_keys(
                kv_map,
                NUMBER_REQUIRED.as_slice(),
                NUMBER_VALID.as_slice(),
                attr_span,
            )?;
            let min = match kv_map.get("min") {
                Some(kv_val) => Some(kv_val.try_into()?),
                None => None,
            };
            let max = match kv_map.get("max") {
                Some(kv_val) => Some(kv_val.try_into()?),
                None => None,
            };
            Ok(FieldType::U8 { min, max })
        }
        "U16" => {
            validate_field_type_keys(
                kv_map,
                NUMBER_REQUIRED.as_slice(),
                NUMBER_VALID.as_slice(),
                attr_span,
            )?;
            let min = match kv_map.get("min") {
                Some(kv_val) => Some(kv_val.try_into()?),
                None => None,
            };
            let max = match kv_map.get("max") {
                Some(kv_val) => Some(kv_val.try_into()?),
                None => None,
            };
            Ok(FieldType::U16 { min, max })
        }
        "Slice" => {
            validate_field_type_keys(
                kv_map,
                SLICE_REQUIRED.as_slice(),
                SLICE_VALID.as_slice(),
                attr_span,
            )?;
            Ok(FieldType::Slice)
        }
        "Text" => {
            validate_field_type_keys(
                kv_map,
                TEXT_REQUIRED.as_slice(),
                TEXT_VALID.as_slice(),
                attr_span,
            )?;
            let pad_byte: Option<u8> = match kv_map.get("pad_byte") {
                Some(kv_val) => Some(kv_val.try_into()?),
                None => None,
            };
            Ok(FieldType::Text { pad_byte })
        }
        "EnumStr" => {
            validate_field_type_keys(
                kv_map,
                ENUMSTR_REQUIRED.as_slice(),
                ENUMSTR_VALID.as_slice(),
                attr_span,
            )?;
            Ok(FieldType::EnumStr {
                collection: kv_map.get("collection").expect("validated").try_into()?,
            })
        }
        "I8" => {
            validate_field_type_keys(
                kv_map,
                NUMBER_REQUIRED.as_slice(),
                NUMBER_VALID.as_slice(),
                attr_span,
            )?;
            let min = match kv_map.get("min") {
                Some(kv_val) => Some(kv_val.try_into()?),
                None => None,
            };
            let max = match kv_map.get("max") {
                Some(kv_val) => Some(kv_val.try_into()?),
                None => None,
            };
            Ok(FieldType::I8 { min, max })
        }
        "I16" => {
            validate_field_type_keys(
                kv_map,
                NUMBER_REQUIRED.as_slice(),
                NUMBER_VALID.as_slice(),
                attr_span,
            )?;
            let min = match kv_map.get("min") {
                Some(kv_val) => Some(kv_val.try_into()?),
                None => None,
            };
            let max = match kv_map.get("max") {
                Some(kv_val) => Some(kv_val.try_into()?),
                None => None,
            };
            Ok(FieldType::I16 { min, max })
        }
        "U32" => {
            validate_field_type_keys(
                kv_map,
                NUMBER_REQUIRED.as_slice(),
                NUMBER_VALID.as_slice(),
                attr_span,
            )?;
            let min = match kv_map.get("min") {
                Some(kv_val) => Some(kv_val.try_into()?),
                None => None,
            };
            let max = match kv_map.get("max") {
                Some(kv_val) => Some(kv_val.try_into()?),
                None => None,
            };
            Ok(FieldType::U32 { min, max })
        }
        "Volume" => {
            validate_field_type_keys(
                kv_map,
                VOLUME_REQUIRED.as_slice(),
                VOLUME_VALID.as_slice(),
                attr_span,
            )?;
            Ok(FieldType::Volume)
        }
        "TD0Decimal" => {
            validate_field_type_keys(
                kv_map,
                DECIMAL_REQUIRED.as_slice(),
                DECIMAL_VALID.as_slice(),
                attr_span,
            )?;
            let min: f32 = kv_map.get("min").expect("validated").try_into()?;
            let max: f32 = kv_map.get("max").expect("validated").try_into()?;
            Ok(FieldType::TD0Decimal { min, max })
        }
        unknown => Err(syn::Error::new_spanned(
            field_type_kv.key(),
            format!("unknown field_type '{unknown}'"),
        )),
    }
}

fn field_type_pretty_name(field: &TD0Field) -> &'static str {
    match &field.field_type {
        FieldType::EnumStr { .. } | FieldType::Text { .. } => "Text",
        FieldType::I16 { .. } => "I16",
        FieldType::I8 { .. } => "I8",
        FieldType::Slice => "Slice",
        FieldType::TD0Decimal { .. } | FieldType::Volume => "Decimal",
        FieldType::U32 { .. } => "U16",
        FieldType::U16 { .. } => "U16",
        FieldType::U8 { .. } => "U8",
    }
}

fn field_type_pretty_name_raw(field: &TD0Field) -> &'static str {
    match field.native_type {
        NativeType::I16 => "I16",
        NativeType::I8 => "I8",
        NativeType::Slice(_) => "Slice",
        NativeType::U8 => "U8",
        NativeType::U16 => "U16",
        NativeType::U32 => "U32",
    }
}

fn get_td0_field_attrs(field: &mut Field) -> syn::Result<(Attribute, AttrHashMap)> {
    let mut field_attrs_kv: Option<AttrKeyValueList> = None;
    let mut parse_error: Option<syn::Error> = None;
    let mut td0_field_attr: Option<Attribute> = None;

    field.attrs.retain(|attr| {
        if attr.path().is_ident("td0_field") {
            // Guard against duplicate td0_field attrs:
            if td0_field_attr.is_some() {
                parse_error = Some(syn::Error::new_spanned(
                    attr,
                    "duplicate td0_field attribute.",
                ));
            } else {
                td0_field_attr = Some(attr.clone());
                field_attrs_kv = match parse_kv_list(attr) {
                    Ok(attr_list) => Some(attr_list),
                    Err(err) => {
                        parse_error = Some(err);
                        None
                    }
                };
            }
            false
        } else {
            true
        }
    });

    if let Some(error) = parse_error {
        return Err(error);
    };

    if td0_field_attr.is_none() || field_attrs_kv.is_none() {
        return Err(syn::Error::new_spanned(
            field,
            "unable to find or parse td0_field attribute.",
        ));
    }
    Ok((
        td0_field_attr.unwrap(),
        hashmap_from_kv_list(field_attrs_kv.unwrap()),
    ))
}

fn hashmap_from_kv_list(kv_list: AttrKeyValueList) -> AttrHashMap {
    HashMap::from_iter(
        kv_list
            .pairs
            .into_iter()
            .map(|kv_item| (kv_item.key().to_string(), kv_item)),
    )
}

fn parse_struct_fields(source: ItemStruct) -> syn::Result<Vec<TD0Field>> {
    let mut fields: Vec<TD0Field> = Vec::new();
    let mut field_errors: Option<syn::Error> = None;
    if let Fields::Named(named) = source.fields {
        for mut named_field in named.named {
            match try_parse_field(&mut named_field) {
                Ok(parsed_field) => fields.push(parsed_field),
                Err(err) => match &mut field_errors {
                    Some(existing) => existing.combine(err),
                    None => field_errors = Some(err),
                },
            }
        }
    } else {
        return Ok(fields);
    }

    match field_errors {
        Some(errors) => Err(errors),
        None => Ok(fields),
    }
}

fn try_parse_field(field: &mut Field) -> syn::Result<TD0Field> {
    let ident = match field.ident.clone() {
        Some(val) => val,
        None => {
            return Err(syn::Error::new_spanned(
                field,
                "td0_field may only be used on named fields.",
            ));
        }
    };
    let (td0_field_attr, field_attrs_kv) = get_td0_field_attrs(field)?;
    let field_type = field_type_from_attrs(&field_attrs_kv, td0_field_attr.span())?;
    let Some(native_type) = get_field_native_type(field) else {
        return Err(syn::Error::new_spanned(
            field,
            "td0_field: unsupported native field type",
        ));
    };
    let ident_str = ident.to_string();
    Ok(TD0Field {
        ident,
        ident_str,
        field_type,
        native_type,
    })
}

fn validate_field_type_keys(
    kv_map: &AttrHashMap,
    required: &[&str],
    valid: &[&str],
    attr_span: Span,
) -> syn::Result<()> {
    let mut attrs_present: HashSet<&str> = HashSet::from_iter(kv_map.keys().map(String::as_str));
    attrs_present.remove("field_type");

    // Early out for no (additional) fields present and none required.
    if attrs_present.is_empty() && required.is_empty() {
        return Ok(());
    }

    let required_vals: HashSet<&str> = required.iter().copied().collect();
    let valid_vals: HashSet<&str> = valid.iter().copied().collect();

    let mut errors: Option<syn::Error> = None;
    for &invalid in attrs_present.difference(&valid_vals) {
        let err = syn::Error::new_spanned(kv_map.get(invalid).unwrap().key(), "invalid parameter");
        match &mut errors {
            Some(existing) => existing.combine(err),
            None => errors = Some(err),
        }
    }

    for &missing in required_vals.difference(&attrs_present) {
        let err = syn::Error::new(attr_span, format!("missing parameter {missing}"));
        match &mut errors {
            Some(existing) => existing.combine(err),
            None => errors = Some(err),
        }
    }

    if let Some(error) = errors {
        Err(error)
    } else {
        Ok(())
    }
}
