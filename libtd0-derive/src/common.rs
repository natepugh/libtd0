use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::{Arm, Expr, Stmt};
use syn::{Attribute, Field, Ident, ImplItemFn, Lit, Meta, Path, Token, Type, parse::Parse};

#[derive(Clone, Debug, PartialEq)]
pub struct AttrKeyValue {
    key: Ident,
    val: Lit,
}

impl Parse for AttrKeyValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        let _: Token![=] = input.parse()?;
        let val: Lit = input.parse()?;
        Ok(Self { key: key, val: val })
    }
}

impl TryFrom<&AttrKeyValue> for u8 {
    type Error = syn::Error;

    fn try_from(kv_pair: &AttrKeyValue) -> Result<Self, <Self as TryFrom<&AttrKeyValue>>::Error> {
        match &kv_pair.val {
            Lit::Int(lit_int) => lit_int
                .base10_parse()
                .map_err(|err| syn::Error::new_spanned(lit_int, err.to_string())),
            _ => Err(syn::Error::new_spanned(
                &kv_pair.val,
                "Must be an Integer type.",
            )),
        }
    }
}

impl TryFrom<&AttrKeyValue> for u32 {
    type Error = syn::Error;

    fn try_from(kv_pair: &AttrKeyValue) -> Result<Self, <Self as TryFrom<&AttrKeyValue>>::Error> {
        match &kv_pair.val {
            Lit::Int(lit_int) => lit_int
                .base10_parse()
                .map_err(|err| syn::Error::new_spanned(lit_int, err.to_string())),
            _ => Err(syn::Error::new_spanned(
                &kv_pair.val,
                "Must be an Integer type.",
            )),
        }
    }
}

impl TryFrom<&AttrKeyValue> for u16 {
    type Error = syn::Error;

    fn try_from(kv_pair: &AttrKeyValue) -> Result<Self, <Self as TryFrom<&AttrKeyValue>>::Error> {
        match &kv_pair.val {
            Lit::Int(lit_int) => lit_int
                .base10_parse()
                .map_err(|err| syn::Error::new_spanned(lit_int, err.to_string())),
            _ => Err(syn::Error::new_spanned(
                &kv_pair.val,
                "Must be an Integer type.",
            )),
        }
    }
}

impl TryFrom<&AttrKeyValue> for i8 {
    type Error = syn::Error;

    fn try_from(kv_pair: &AttrKeyValue) -> Result<Self, <Self as TryFrom<&AttrKeyValue>>::Error> {
        match &kv_pair.val {
            Lit::Int(lit_int) => lit_int
                .base10_parse()
                .map_err(|err| syn::Error::new_spanned(lit_int, err.to_string())),
            _ => Err(syn::Error::new_spanned(
                &kv_pair.val,
                "Must be an Integer type.",
            )),
        }
    }
}

impl TryFrom<&AttrKeyValue> for i16 {
    type Error = syn::Error;

    fn try_from(kv_pair: &AttrKeyValue) -> Result<Self, <Self as TryFrom<&AttrKeyValue>>::Error> {
        match &kv_pair.val {
            Lit::Int(lit_int) => lit_int
                .base10_parse()
                .map_err(|err| syn::Error::new_spanned(lit_int, err.to_string())),
            _ => Err(syn::Error::new_spanned(
                &kv_pair.val,
                "Must be an Integer type.",
            )),
        }
    }
}

impl TryFrom<&AttrKeyValue> for f64 {
    type Error = syn::Error;

    fn try_from(kv_pair: &AttrKeyValue) -> Result<Self, <Self as TryFrom<&AttrKeyValue>>::Error> {
        match &kv_pair.val {
            Lit::Float(lit_float) => lit_float
                .base10_parse()
                .map_err(|err| syn::Error::new_spanned(lit_float, err.to_string())),
            _ => Err(syn::Error::new_spanned(
                &kv_pair.val,
                "Must be an decimal number.",
            )),
        }
    }
}

impl TryFrom<&AttrKeyValue> for String {
    type Error = syn::Error;

    fn try_from(kv_pair: &AttrKeyValue) -> Result<Self, <Self as TryFrom<&AttrKeyValue>>::Error> {
        match &kv_pair.val {
            Lit::Str(lit_text) => Ok(lit_text.to_token_stream().to_string().replace("\"", "")),
            _ => Err(syn::Error::new_spanned(
                &kv_pair.val,
                "Must be an string type.",
            )),
        }
    }
}

impl AttrKeyValue {
    pub fn get_val(&self) -> &Lit {
        &self.val
    }
}

#[derive(Default, Clone)]
pub struct AttrKeyValueList {
    pub pairs: Vec<AttrKeyValue>,
}

impl Parse for AttrKeyValueList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let pairs: Punctuated<AttrKeyValue, Token![,]> =
            Punctuated::parse_separated_nonempty(input)?;
        Ok(Self {
            pairs: pairs.into_iter().collect(),
        })
    }
}

impl AttrKeyValueList {
    pub fn find_by_name(&self, name: &str) -> Option<&AttrKeyValue> {
        self.pairs
            .iter()
            .find(|pair| pair.key.to_string().as_str() == name)
    }
}

pub fn parse_kv_list(attr: &Attribute) -> syn::Result<AttrKeyValueList> {
    match &attr.meta {
        Meta::List(_mlist) => attr.parse_args::<AttrKeyValueList>(),
        Meta::Path(_mpath) => Ok(AttrKeyValueList::default()),
        Meta::NameValue(_m_namevalue) => attr.parse_args::<AttrKeyValueList>(),
    }
}

pub fn get_type_path(typeobj: &Type) -> &Path {
    match typeobj {
        Type::Path(tpath) => &tpath.path,
        Type::Array(apath) => get_type_path(&*apath.elem),
        _ => unimplemented!("get_type_path: Type: {:#?}", typeobj),
    }
}

pub fn validate_attr_list_valid_attrs(
    kv_list: &AttrKeyValueList,
    valid_names: &[&str],
) -> syn::Result<()> {
    match kv_list
        .pairs
        .iter()
        .find(|pair| !valid_names.contains(&pair.key.to_string().as_str()))
    {
        Some(pair) => Err(syn::Error::new_spanned(&pair.key, "Unexpected key")),
        None => Ok(()),
    }
}

pub fn validate_attr_list_required_attrs(
    kv_list: &AttrKeyValueList,
    required_names: &[&str],
    source_attr: &Attribute,
) -> syn::Result<()> {
    let missing_keys: Vec<String> = required_names
        .iter()
        .filter_map(|name| match kv_list.find_by_name(name) {
            Some(_) => None,
            None => Some(name.to_string()),
        })
        .collect();

    if !missing_keys.is_empty() {
        let msg = format!("Missing required key(s) {missing_keys:?}");
        return Err(syn::Error::new_spanned(source_attr, msg));
    }

    Ok(())
}

pub fn push_arm_to_fn_match(func: &mut ImplItemFn, arm: &Arm) {
    for stmt in func.block.stmts.iter_mut() {
        if let Stmt::Expr(expr, _) = stmt {
            if let Expr::Match(match_stmt) = expr {
                match_stmt.arms.push(arm.clone());
            }
        };
    }
}

pub fn get_named_attr<'a>(attr_name: &'static str, field: &'a Field) -> Option<&'a Attribute> {
    field.attrs.iter().find(|attr| match &attr.meta {
        Meta::List(mlist) => mlist.path.is_ident(attr_name),
        Meta::Path(mpath) => mpath.is_ident(attr_name),
        _ => false,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct NativeTypeSlice {
    pub size: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NativeType {
    I8,
    I16,
    Slice(NativeTypeSlice),
    U16,
    U32,
    U8,
}

pub fn get_field_native_type(field: &Field) -> Option<NativeType> {
    match &field.ty {
        Type::Path(tpath) => {
            if tpath.path.segments.len() == 1 {
                match tpath.path.segments[0].ident.to_string().as_str() {
                    "I16" => Some(NativeType::I16),
                    "i8" => Some(NativeType::I8),
                    "u8" => Some(NativeType::U8),
                    "U16" => Some(NativeType::U16),
                    "U32" => Some(NativeType::U32),
                    _ => {
                        println!("Unexpected type ident: {:#?}", tpath.path.segments[0].ident);
                        None
                    }
                }
            } else {
                println!("Unexpected tpath segments: {tpath:#?}");
                None
            }
        }
        Type::Array(apath) => {
            let path = get_type_path(&*apath.elem);
            let last_segment = path
                .segments
                .iter()
                .last()
                .expect("Path must have segments");

            match last_segment.ident.to_string().as_str() {
                "u8" => {
                    if let Expr::Lit(type_expr) = &apath.len
                        && let Lit::Int(int_expr) = &type_expr.lit
                    {
                        Some(NativeType::Slice(NativeTypeSlice {
                            size: int_expr.base10_parse().unwrap_or_else(|_| 0),
                        }))
                    } else {
                        None
                    }
                }
                _ => {
                    println!(
                        "Unexpected array base type ident: {:#?}",
                        last_segment.ident
                    );
                    None
                }
            }
        }
        _ => None,
    }
}
