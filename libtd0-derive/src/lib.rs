use proc_macro2::{TokenStream, Literal};
use quote::{ToTokens, format_ident, quote};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::mem::Discriminant;
use syn::punctuated::Punctuated;
use syn::{Arm, Expr, Stmt, parse_quote};
use syn::{Attribute, Field, Ident, ItemStruct, Lit, Path, Token, Type, parse::Parse, parse2, Meta, ImplItemFn};
use std::convert::TryInto;


#[derive(Clone, Debug, PartialEq)]
struct FieldAttrText {
    pad_byte: Option<u8>,
}

fn parse_kv_list(attr: &Attribute) -> syn::Result<AttrKeyValueList> {
    attr.parse_args::<AttrKeyValueList>()
}

fn validate_allowed_names(kv_list: &AttrKeyValueList, valid_names: &[&str]) -> syn::Result<()> {
    let mut my_valid_names : Vec<&str> = Vec::from(valid_names);
    my_valid_names.insert(0, "field_type");
    validate_attr_list_attr_names(kv_list, &my_valid_names[..])
}

fn validate_required_names(kv_list: &AttrKeyValueList, required_names: &[&str], source_attr: &Attribute) -> syn::Result<()> {
    let mut my_required_names : Vec<&str> = Vec::from(required_names);
    my_required_names.insert(0, "field_type");
    validate_attr_list_required_attrs(&kv_list, &my_required_names[..], source_attr)
}

fn parse_and_validate_kv_list(attr: &Attribute, required_names: &[&str], valid_names: &[&str]) -> syn::Result<AttrKeyValueList> {
    let kv_list  = parse_kv_list(attr)?;
    validate_required_names(&kv_list, required_names, attr)?;
    validate_allowed_names(&kv_list, valid_names)?;
    Ok(kv_list)
}

impl TryFrom<&Attribute> for FieldAttrText {
    type Error = syn::Error;

    fn try_from(attr: &Attribute) -> Result<Self, <Self as TryFrom<&Attribute>>::Error> {
        static REQUIRED_NAMES : [&str; 0] = [];
        static VALID_NAMES : [&str; 1] = ["pad_byte"];

        let kv_list  = parse_and_validate_kv_list(attr, &REQUIRED_NAMES, &VALID_NAMES)?;
        Ok(Self {
            pad_byte: match kv_list.find_by_name("pad_byte") {
                Some(pair) => Some(pair.try_into()?),
                None => None
            }
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
struct FieldAttrNone;

#[derive(Clone, Debug, PartialEq)]
struct FieldAttrEnumStr {
    collection: String,
}

impl TryFrom<&Attribute> for FieldAttrEnumStr {
    type Error = syn::Error;

    fn try_from(attr: &Attribute) -> Result<Self, <Self as TryFrom<&Attribute>>::Error> {
        static REQUIRED_NAMES : [&str; 1] = ["collection"];
        static VALID_NAMES : [&str; 1] = ["collection"];

        let kv_list  = parse_and_validate_kv_list(attr, &REQUIRED_NAMES, &VALID_NAMES)?;
        Ok(Self {
            collection: kv_list.find_by_name("collection").expect("Guarded above.").try_into()?
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
struct FieldAttrTD0Decimal {
    min: Decimal,
    max: Decimal,
}

impl TryFrom<&Attribute> for FieldAttrTD0Decimal {
    type Error = syn::Error;

    fn try_from(attr: &Attribute) -> Result<Self, <Self as TryFrom<&Attribute>>::Error> {
        static REQUIRED_NAMES : [&str; 2] = ["min", "max"];
        static VALID_NAMES : [&str; 2] = ["min", "max"];

        let kv_list  = parse_and_validate_kv_list(attr, &REQUIRED_NAMES, &VALID_NAMES)?;
        let min: i64 = kv_list.find_by_name("min").expect("Guarded above.").try_into()?;
        let max: i64 = kv_list.find_by_name("max").expect("Guarded above.").try_into()?;
        Ok(Self {
            min: Decimal::from(min),
            max: Decimal::from(max),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
enum FieldAttr {
    FieldAttrTD0Decimal(FieldAttrTD0Decimal),
    FieldAttrEnumStr(FieldAttrEnumStr),
    FieldAttrI16(FieldAttrNone),
    FieldAttrText(FieldAttrText),
    FieldAttrU16(FieldAttrNone),
    FieldAttrU8(FieldAttrNone),
    FieldAttrVolume(FieldAttrNone),
}

#[derive(Clone, Debug, PartialEq)]
struct TD0Field {
    ident: Ident,
    attr: FieldAttr,
}

impl TD0Field {
    pub fn ident_to_string(&self) -> String {
        self.ident.to_string()
    }

    pub fn ident_ref_mut(&mut self) -> &mut Ident {
        &mut self.ident
    }

    pub fn ident_ref(&self) -> &Ident {
        &self.ident
    }
}

impl TryFrom<&Field> for TD0Field {
    type Error = syn::Error;

    fn try_from(field: &Field) -> Result<Self, <Self as TryFrom<&Field>>::Error> {
        let attr = get_named_attr("td0_field", field).ok_or(syn::Error::new_spanned(field, "Field is not a td0_field"))?.clone();
        let kv_list  = parse_kv_list(&attr)?;
        validate_required_names(&kv_list, &["field_type"], &attr)?;

        let ident = field.ident.clone().ok_or(syn::Error::new_spanned(field, "Field can't be anonymous"))?;

        let field_type = kv_list.find_by_name("field_type").expect("Verified above.");
        let field_type_name : String = field_type.try_into()?;

        let parsed_field_attr: FieldAttr = match field_type_name.as_str() {
            "Text" => FieldAttr::FieldAttrText(FieldAttrText::try_from(&attr)?),
            "EnumStr" => FieldAttr::FieldAttrEnumStr(FieldAttrEnumStr::try_from(&attr)?),
            "TD0Decimal" => FieldAttr::FieldAttrTD0Decimal(FieldAttrTD0Decimal::try_from(&attr)?),
            "U8" => FieldAttr::FieldAttrVolume(FieldAttrNone{}),
            "I16" => FieldAttr::FieldAttrVolume(FieldAttrNone{}),
            "U16" => FieldAttr::FieldAttrVolume(FieldAttrNone{}),
            "Volume" => FieldAttr::FieldAttrVolume(FieldAttrNone{}),
            _ => return Err(syn::Error::new_spanned(&field_type.val, "Unknown field_type: `{field_type_name}`"))
        };
        Ok(Self { ident: ident, attr: parsed_field_attr })
    }
}


/*
    TD0Decimal(IntEncodedDecimal),
    EnumStr(&'static str),
    Text(String),
    U16(u16),
    I16(i16),
    U8(u8),
    Volume(Volume),

*/

fn get_type_path(typeobj: &Type) -> &Path {
    match typeobj {
        Type::Path(tpath) => &tpath.path,
        Type::Array(apath) => { get_type_path(&*apath.elem) },
        _ => unimplemented!("get_type_path: Type: {:#?}", typeobj)
    }
}

fn default_tokenstream_for_field_type(field: &Field) -> TokenStream {
    let mut output : TokenStream = TokenStream::new();

    for segment in get_type_path(&field.ty).segments.iter() {
        let raw_value = match segment.ident.to_string().as_str() {
            "String" => { quote!(String::new()) },
            "bool" => { quote!(false) },
            "double" => { quote!(0.0) },
            "float" => { quote!(0.0) },
            "u16" => { quote!(0) },
            "u32" => { quote!(0) },
            "u8" => { quote!(0) },
            "U16" => { quote!(U16::from(0))},
            "I16" => { quote!(I16::from(0))},
            _ => unimplemented!("Field Path ident: {}", segment.ident.to_string())
        };

        let def_value = match &field.ty {
            Type::Path(_tpath) => { raw_value },
            Type::Array(apath) => {
                let count = &apath.len;
                quote!([0; #count])
            },
            _ => unimplemented!("default_tokenstream_for_field_type: Type: {:#?}", &field.ty)
        };
        output.extend(def_value);
    }
    output
}

fn td0_struct_gen_impl_default(source: &ItemStruct) -> TokenStream {
    let ident = &source.ident;
    let mut field_defs : Vec<TokenStream> = Vec::new();
    for field in &source.fields {
        let field_ident = field.ident.clone().expect("Fields must be named.");
        let default_val = default_tokenstream_for_field_type(&field);
        field_defs.push(quote!{#field_ident: #default_val});
    }

    quote!{
        impl Default for #ident {
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
    let mut output : String = String::new();
    let val_string = val.to_string();
    let mut val_chars = val_string.chars();
    let mut last_char = val_chars.nth(0).unwrap();

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

fn td0_struct_gen_get_fields(source: &ItemStruct) -> TokenStream {
    let mut field_tokenstreams: Vec<String> = Vec::new();
    let mut field_idents: Vec<Ident> = Vec::new();
    let source_ident = &source.ident;
    let mut fields_attrs : HashMap<Ident, HashMap<String, Lit>> = HashMap::new();

    for field in source.fields.iter() {
        match &field.ident {
            Some(ident) => {
                field_tokenstreams.push(ident.into_token_stream().to_string());
                field_idents.push(ident.clone());
            },
            None => ()
        }
        match get_td0_field_attrs(field) {
            Ok(Some(kv_list)) => {
                fields_attrs.insert(
                    field.ident.clone().expect("Field is named"),
                    attr_list_to_mapping(&kv_list)
                );
            }
            Ok(None) => (),
            Err(err) => { return err.into_compile_error(); }
        }
    }

    let num_fields : Literal = Literal::usize_unsuffixed(field_tokenstreams.len());
    let const_field_array_name = format_ident!("{}_FIELDS", const_name_from_ident(source_ident));

    let mut output : TokenStream = quote!{
        pub const #const_field_array_name: [&str; #num_fields] = [
            #( #field_tokenstreams ),*
        ];
    };

    let fn_get_value : ImplItemFn = build_chunkitem_get_value(&fields_attrs, &field_idents);

    // Create an impl with get_fields().
    output.extend(quote!(
        impl ChunkItem for #source_ident {
            fn get_fields(&self) -> &'static [&'static str] { &#const_field_array_name }
            #fn_get_value
            fn set_value(&mut self, field: &str, value: &ChunkItemValue) -> TD0Result<()> {
                todo!()
            }
            fn get_value_raw(&self, field: &str) -> Option<ChunkItemValueRaw> {
                todo!()
            }
            fn set_value_raw(&mut self, field: &str, value: &ChunkItemValueRaw) -> TD0Result<()> {
                todo!()
            }
        }
    ));

    output
}

fn build_getter_expr(ident: &Ident, fields_attrs: &HashMap<Ident, HashMap<String, Lit>>) -> Expr {
    let Some(attrs) = fields_attrs.get(ident) else {
        return parse_quote!(None)
    };
    let Some(Lit::Str(field_type)) = attrs.get("field_type") else {
        return parse_quote!(None)
    };

    match field_type.to_token_stream().to_string().replace("\"", "").as_str() {
        "Text" => {
            let default_pad : Lit = parse_quote!(0);
            let lit_pad_char : &Lit = attrs.get("pad_char").unwrap_or(&default_pad);
            let pad_val : u8 = match lit_pad_char {
                Lit::Int(lit_int) => lit_int.base10_parse().unwrap_or(0u8),
                _ => 0
            };

            parse_quote!(
                Some(
                    ChunkItemValue::text_from_u8_array(&self.#ident, &#pad_val)
                )
            )
        },
        "U16" => parse_quote!(Some(ChunkItemValue::U16(self.#ident.get()))),
        "I16" => parse_quote!(Some(ChunkItemValue::I16(self.#ident.get()))),
        "U8" => parse_quote!(Some(ChunkItemValue::U8(self.#ident))),
        "Volume" => parse_quote!(
            match Volume::try_from(self.#ident.get()) {
                Ok(vol) => Some(ChunkItemValue::Volume(vol)),
                Err(_) => None
            }
        ),
        "EnumStr" => {
            let Lit::Str(collection_name ) = attrs.get("collection").expect("`collection` is required.") else {
                panic!("`collection` is required.");
            };
            let collection_ident = format_ident!("{}", collection_name.to_token_stream().to_string().replace("\"", ""));
            parse_quote!(Some(ChunkItemValue::EnumStr(
                *#collection_ident
                    .get(self.#ident as usize)
                    .unwrap_or_else(|| &"INVALID")
            )))
        },
        _ => parse_quote!(None)
    }
}

fn build_chunkitem_get_value(fields_attrs: &HashMap<Ident, HashMap<String, Lit>>, field_order: &[Ident]) -> ImplItemFn {
    let mut fn_skel : ImplItemFn = parse_quote!(
        fn get_value(&self, field: &str) -> Option<ChunkItemValue> {
            match field { }
        }
    );

    let mut exprs: Vec<Expr> = Vec::new();
    for ident in field_order.iter() {
        exprs.push(build_getter_expr(ident, fields_attrs));
    }

    let mut get_fields_clauses: Vec<Arm> = field_order.iter().zip(exprs).map(|(fld, expr)| {
        let fld_name : String = fld.to_string(); 
        parse_quote!(#fld_name => #expr)
    }).collect();
    get_fields_clauses.push(parse_quote!(_ => None));


    for arm in get_fields_clauses.iter() {
        push_arm_to_fn_match(&mut fn_skel, &arm);
    }
    fn_skel
}

fn validate_attr_list_attr_names(kv_list: &AttrKeyValueList, valid_names: &[&str]) -> syn::Result<()> {
    /*
    for pair in kv_list.pairs.iter() {
        if !valid_names.contains(&pair.key.to_string().as_str()) {
            return Err(syn::Error::new_spanned(&pair.key, "Unexpected key"));
        }
    }
    Ok(())
    */
    match kv_list.pairs.iter().find(|pair| { !valid_names.contains(&pair.key.to_string().as_str()) }) {
        Some(pair) => Err(syn::Error::new_spanned(&pair.key, "Unexpected key")),
        None => Ok(())
    }
}

fn validate_attr_list_required_attrs(kv_list: &AttrKeyValueList, required_names: &[&str], source_attr: &Attribute) -> syn::Result<()> {
    let missing_keys : Vec<String> = required_names.iter().filter_map(|name| {
        match kv_list.find_by_name(name) {
            Some(_) => None,
            None => Some(name.to_string())
        }
    }).collect();

    if !missing_keys.is_empty() {
        let msg = format!("Missing required key(s) {missing_keys:?}");
        return Err(syn::Error::new_spanned(source_attr, msg));
    }

    Ok(())
}

struct LitDiscriminantComp {
    value: Discriminant<Lit>,
    pretty_name: &'static str,
}

// fn validate_attr_kv_types(kv_list: &AttrKeyValueList, expected_types: &HashMap<&str, &LitDiscriminantComp>) -> syn::Result<()> {
fn validate_attr_kv_types(kv_list: &AttrKeyValueList, expected_types: &HashMap<&str, LitDiscriminantComp>) -> syn::Result<()> {
    for kv_pair in kv_list.pairs.iter() {
        match expected_types.get(kv_pair.key.to_string().as_str()) {
            Some(expected) => {
                if !(std::mem::discriminant(&kv_pair.val) == expected.value) {
                    // let msg = format!("Value expected type '{}'", expected.pretty_name);
                    let msg = format!("{} must be a {}.",
                        kv_pair.key.to_string(),
                        expected.pretty_name,
                    );
                    return Err(syn::Error::new_spanned(&kv_pair.val, msg));
                }
            },
            None => ()
        }
    }

    Ok(())
}

/*
 * TODO: Using these statics is nicer than the get_lit_FOO_discriminant() functions,
 *       but it breaks rust-analyzer's proc-macro server :(
 *       Determine if there's a way to have it all.
static LIT_STR : std::sync::LazyLock<LitDiscriminantComp> = std::sync::LazyLock::new(|| {
    LitDiscriminantComp {
        value: std::mem::discriminant(&parse_quote!("")),
        pretty_name: "String"
    }
});

static LIT_INT : std::sync::LazyLock<LitDiscriminantComp> = std::sync::LazyLock::new(|| {
    LitDiscriminantComp {
        value: std::mem::discriminant(&parse_quote!(0)),
        pretty_name: "Integer"
    }
});
*/

fn get_lit_int_discriminant() -> LitDiscriminantComp {
    LitDiscriminantComp {
        value: std::mem::discriminant(&parse_quote!(0)),
        pretty_name: "integer"
    }
}

fn get_lit_str_discriminant() -> LitDiscriminantComp {
    LitDiscriminantComp {
        value: std::mem::discriminant(&parse_quote!("")),
        pretty_name: "string"
    }
}

fn validate_td0_field_attr_list(kv_list: &AttrKeyValueList, source_attr: &Attribute) -> syn::Result<()> {
    let valid_attrs = ["field_type", "pad_byte", "collection"];
    let required_attrs = ["field_type"];

    validate_attr_list_attr_names(kv_list, &valid_attrs)?;
    validate_attr_list_required_attrs(kv_list, &required_attrs, source_attr)?;

    let expected_types : HashMap<&str, LitDiscriminantComp> = HashMap::from([
        ("field_type", get_lit_str_discriminant()),
        ("pad_byte", get_lit_int_discriminant()),
        ("collection", get_lit_str_discriminant()),
    ]);

    /*
    let expected_types : HashMap<&str, &LitDiscriminantComp> = HashMap::from([
        ("field_type", &*LIT_STR),
        ("pad_byte", &*LIT_INT),
        ("collection", &*LIT_STR),
    ]);
    */

    validate_attr_kv_types(kv_list, &expected_types)?;

    Ok(())
}

fn parse_td0_field_attr(attr: &Attribute) -> syn::Result<AttrKeyValueList> {
    let kv_list = attr.parse_args::<AttrKeyValueList>()?;
    let _ = validate_td0_field_attr_list(&kv_list, &attr)?;
    Ok(kv_list)
}

fn push_arm_to_fn_match(func: &mut ImplItemFn, arm: &Arm) {
    for stmt in func.block.stmts.iter_mut() {
        if let Stmt::Expr(expr, _) = stmt {
            if let Expr::Match(match_stmt) = expr {
                match_stmt.arms.push(arm.clone());
            }
        };
    }
}

fn get_named_attr<'a>(attr_name: &'static str, field: &'a Field) -> Option<&'a Attribute> {
    field.attrs.iter().find(|attr|
        match &attr.meta {
            Meta::List(mlist) => mlist.path.is_ident(attr_name),
            _ => false
        }
    )
}

fn get_td0_field_attrs(field: &Field) -> syn::Result<Option<AttrKeyValueList>> {
    let mut field_attr : Option<Attribute> = None;
    for attr in field.attrs.iter() {
        if let Meta::List(mlist) = &attr.meta && mlist.path.is_ident("td0_field") {
            field_attr = Some(attr.clone());
            break
        }
    }
    if field_attr.is_none() { return Ok(None); }

    Ok(Some(parse_td0_field_attr(&field_attr.expect("Guarded above."))?))
}

#[proc_macro_derive(TD0ChunkItem, attributes(td0_field))]
pub fn td0_struct_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let source : ItemStruct = parse2(input).expect("Error parsing input struct!");

    let mut output : TokenStream = td0_struct_gen_impl_default(&source);
    output.extend(td0_struct_gen_get_fields(&source));
    // output.extend(td0_struct_gen_chunkitem_impl(&source));
    output.into()
}

#[derive(Clone, Debug, PartialEq)]
struct AttrKeyValue {
    key: Ident,
    val: Lit,
}

impl Parse for AttrKeyValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key : Ident = input.parse()?;
        let _ : Token![=] = input.parse()?;
        let val: Lit = input.parse()?;
        Ok(Self{
            key: key,
            val: val,
        })
    }
}

impl TryFrom<&AttrKeyValue> for u8 {
    type Error = syn::Error;

    fn try_from(kv_pair: &AttrKeyValue) -> Result<Self, <Self as TryFrom<&AttrKeyValue>>::Error> {
        match &kv_pair.val {
            Lit::Int(lit_int) => {
                lit_int.base10_parse()
                       .map_err(|err| syn::Error::new_spanned(lit_int, err.to_string()))
            },
            _ => Err(syn::Error::new_spanned(&kv_pair.val, "Must be an Integer type."))
        } 
    }
}

impl TryFrom<&AttrKeyValue> for i64 {
    type Error = syn::Error;

    fn try_from(kv_pair: &AttrKeyValue) -> Result<Self, <Self as TryFrom<&AttrKeyValue>>::Error> {
        match &kv_pair.val {
            Lit::Int(lit_int) => {
                lit_int.base10_parse()
                       .map_err(|err| syn::Error::new_spanned(lit_int, err.to_string()))
            },
            _ => Err(syn::Error::new_spanned(&kv_pair.val, "Must be an Integer type."))
        } 
    }
}

impl TryFrom<&AttrKeyValue> for String {
    type Error = syn::Error;

    fn try_from(kv_pair: &AttrKeyValue) -> Result<Self, <Self as TryFrom<&AttrKeyValue>>::Error> {
        match &kv_pair.val {
            Lit::Str(lit_text) => {
                Ok(lit_text.to_token_stream().to_string().replace("\"", ""))
            },
            _ => Err(syn::Error::new_spanned(&kv_pair.val, "Must be an string type."))
        } 
    }
}

#[derive(Default, Clone)]
struct AttrKeyValueList {
    pub pairs: Vec<AttrKeyValue>,
}

impl Parse for AttrKeyValueList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let pairs: Punctuated::<AttrKeyValue, Token![,]> = Punctuated::parse_separated_nonempty(input)?;
        Ok(Self{
            pairs: pairs.into_iter().collect()
        })
    }
}

impl AttrKeyValueList {
    pub fn find_by_name(&self, name: &str) -> Option<&AttrKeyValue> {
        self.pairs.iter().find(|pair| { 
            pair.key.to_string().as_str() == name
        })
    }
}

fn validate_repeat_attr_list(kv_list: &AttrKeyValueList, source_attr: &Attribute) -> syn::Result<()> {
    let valid_attrs = ["count", "format"];
    let required_attrs = ["count", "format"];

    validate_attr_list_attr_names(kv_list, &valid_attrs)?;
    validate_attr_list_required_attrs(kv_list, &required_attrs, source_attr)?;

    let expected_types : HashMap<&str, LitDiscriminantComp> = HashMap::from([
        ("count", get_lit_int_discriminant()),
        ("format", get_lit_str_discriminant()),
    ]);
    validate_attr_kv_types(kv_list, &expected_types)?;

    Ok(())
}

fn attr_list_to_mapping(kv_list: &AttrKeyValueList) -> HashMap<String, Lit> {
    kv_list.pairs
        .iter()
        .map(|pair| { (pair.key.to_string(), pair.val.clone()) })
        .collect::<HashMap<String, Lit>>()
}

fn parse_repeat_attr(attr: &Attribute) -> syn::Result<AttrKeyValueList> {
    let kv_list = attr.parse_args::<AttrKeyValueList>()?;
    let _ = validate_repeat_attr_list(&kv_list, &attr)?;
    Ok(kv_list)
}

#[proc_macro_attribute]
pub fn repeat_fields(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let mut source : ItemStruct = parse2(input).expect("Error parsing input struct!");
    //let mut fields_repeats : Vec<FieldRepeatSpec> = Vec::new();

    if let syn::Fields::Named(ref mut fields) = source.fields {
        // TODO: Can this be done more efficiently than clone() and clearing
        //       all that data?
        let mut new_fields_named = fields.named.clone();
        new_fields_named.clear();

        for field in fields.named.iter() {
            let mut target_attr : Option<Attribute> = None;
            let mut new_attrs = field.attrs.clone();
            new_attrs.retain(|attr| {
                if attr.path().is_ident("repeat") {
                    target_attr = Some(attr.clone());
                    false
                }
                else {
                    true
                }
            });

            // Early out: The "repeat" attr was not set, add the field and
            // move to the next iteration.
            if target_attr.is_none() {
                new_fields_named.push(field.clone());
                continue
            }

            let repeat_attr_list = match parse_repeat_attr(&target_attr.unwrap()) {
                Ok(val) => val,
                Err(err) => {
                    return err.into_compile_error().into();
                }
            };
            let repeat_attrs = attr_list_to_mapping(&repeat_attr_list);

            let format = repeat_attrs.get("format").expect("Must exist here.").to_token_stream().to_string();
            let count : u8 = repeat_attrs.get("count").expect("Must exist here.").to_token_stream().to_string().parse().expect("`repeat(N)` N must be a u8-able number!");

            for count in 1..count+1 {
                let mut new_field = field.clone();
                new_field.attrs = new_attrs.clone();

                // TODO: Why are there superfluous quotes here? (removed by the final .replace())
                let ident_str = format.replace("{}", count.to_string().as_str()).replace("\"", "");
                new_field.ident = format_ident!("{}", ident_str).into();
                // fields.named.insert(fspec.index + 1, new_field);
                new_fields_named.push(new_field);
            }
        }

        fields.named = new_fields_named;
    }

    source.into_token_stream().into()
}