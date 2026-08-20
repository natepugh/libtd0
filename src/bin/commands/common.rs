use core::ops::Range;
use indexmap::{IndexMap, IndexSet};
use libtd0::{TD0ChunkItem, TD0Error, TD0File, TD0Result, TD0Value, parse_td0_file};
use libtd0_core::TD0Chunk;
use serde::Serialize;

pub type ItemValues = IndexMap<String, TD0Value>;
pub type IndexedItemValues = IndexMap<usize, ItemValues>;

fn get_chunk_or_error(td0file: &dyn TD0File, chunk_name: &str) -> TD0Result<Box<dyn TD0Chunk>> {
    match td0file.get_chunk(chunk_name) {
        Some(chunk) => Ok(chunk),
        None => Err(TD0Error::UnknownChunk),
    }
}

fn validate_chunk_item_range(range: &Range<usize>, chunk: &dyn TD0Chunk) -> TD0Result<()> {
    if range.end > chunk.num_items() {
        Err(TD0Error::OutOfRange)
    } else {
        Ok(())
    }
}

fn validate_fields(
    chunk_item: &dyn TD0ChunkItem,
    fields: &[String],
    chunk_name: &str,
) -> TD0Result<()> {
    let all_fields: IndexSet<&str> = chunk_item.list_fields().iter().copied().collect();
    let requested_fields: IndexSet<&str> = fields.iter().map(|fld| fld.as_str()).collect();

    let unknown_fields = requested_fields.difference(&all_fields);
    let unknown_fields_str = unknown_fields.copied().collect::<Vec<&str>>().join(",");

    if !unknown_fields_str.is_empty() {
        return Err(TD0Error::InvalidInputWithMessage(format!(
            "Unknown {chunk_name} fields: [{unknown_fields_str}]"
        )));
    }
    Ok(())
}

fn requested_fields_or_all<'a>(
    td0file: &'a dyn TD0File,
    chunk_name: &str,
    fields: &'a Option<Vec<String>>,
) -> TD0Result<Vec<&'a str>> {
    let Some(default_item) = td0file.get_chunk_item_default(chunk_name) else {
        return Err(TD0Error::UnknownChunk);
    };

    match fields {
        Some(flds) => {
            validate_fields(&*default_item, flds, chunk_name)?;
            Ok(flds.iter().map(|st| st.as_str()).collect())
        }
        None => Ok(default_item.list_fields().into()),
    }
}

pub fn command_dump_chunk_item<'a>(
    td0file: &'a dyn TD0File,
    chunk_name: &'a str,
    item_num: usize,
) -> TD0Result<&'a [u8]> {
    td0file.get_chunk_item_raw(chunk_name, item_num)
}

pub fn command_dump_chunk<'a>(td0file: &'a dyn TD0File, chunk_name: &str) -> TD0Result<&'a [u8]> {
    td0file
        .get_chunk_raw(chunk_name)
        .ok_or(TD0Error::UnknownChunk)
}

pub fn command_dump_chunk_values(
    bytes: &[u8],
    chunk_name: &str,
    item_range: Option<Range<usize>>,
    fields: &Option<Vec<String>>,
) -> TD0Result<IndexedItemValues> {
    let td0file = parse_td0_file(bytes)?;
    let chunk = get_chunk_or_error(&*td0file, chunk_name)?;
    let fields: Vec<&str> = requested_fields_or_all(&*td0file, chunk_name, fields)?;
    let item_range = item_range.unwrap_or_else(|| 0..chunk.num_items());
    validate_chunk_item_range(&item_range, &*chunk)?;

    let mut output: IndexedItemValues = IndexedItemValues::new();

    for idx in item_range {
        if let Ok(item) = td0file.get_chunk_item(chunk_name, idx) {
            let mut vals: ItemValues = ItemValues::new();
            for field in fields.iter() {
                vals.insert(
                    field.to_string(),
                    item.get_value(field)
                        .expect("get_value failed for an existing field."),
                );
            }
            // output.push(vals);
            output.insert(idx, vals);
        }
    }

    Ok(output)
}

#[derive(Serialize)]
pub struct ChunkFieldDiffItem {
    pub field: String,
    pub left: Option<TD0Value>,
    pub right: Option<TD0Value>,
    pub left_name: Option<String>,
    pub right_name: Option<String>,
}

pub fn command_compare_chunk_items(
    td0file: &dyn TD0File,
    chunk_name_1: &str,
    chunk_name_2: &str,
    item_num: usize,
) -> TD0Result<Vec<ChunkFieldDiffItem>> {
    let chunk_1_item = td0file.get_chunk_item(chunk_name_1, item_num)?;
    let chunk_2_item = td0file.get_chunk_item(chunk_name_2, item_num)?;

    let chunk_1_fields: IndexSet<&str> =
        IndexSet::from_iter(chunk_1_item.list_fields().iter().copied());
    let chunk_2_fields: IndexSet<&str> =
        IndexSet::from_iter(chunk_2_item.list_fields().iter().copied());

    let mut all_fields: IndexSet<&str> = chunk_1_fields.clone();
    all_fields.extend(chunk_2_fields.iter());

    let mut diffs: Vec<ChunkFieldDiffItem> = Vec::new();
    for field in all_fields.iter() {
        let c1_val = chunk_1_item.get_value(field);
        let c2_val = chunk_2_item.get_value(field);
        if c1_val != c2_val {
            diffs.push(ChunkFieldDiffItem {
                field: field.to_string(),
                left: c1_val,
                right: c2_val,
                left_name: Some(chunk_name_1.to_string()),
                right_name: Some(chunk_name_2.to_string()),
            });
        }
    }

    Ok(diffs)
}

pub fn parse_chunk_item_field_value(
    chunk_item: &dyn TD0ChunkItem,
    field: &str,
    val: &str,
) -> TD0Result<TD0Value> {
    match chunk_item.get_field_type(field) {
        Some("TD0Decimal") | Some("Volume") => Ok(TD0Value::Decimal(
            val.parse().map_err(|_| TD0Error::InvalidInput)?,
        )),
        Some("I16") => Ok(TD0Value::I16(
            val.parse().map_err(|_| TD0Error::InvalidInput)?,
        )),
        Some("Text") | Some("EnumStr") => Ok(TD0Value::Text(val.to_string())),
        Some("I8") => Ok(TD0Value::I8(
            val.parse().map_err(|_| TD0Error::InvalidInput)?,
        )),
        // Slice parsing from String not implemented.
        Some("U8") => Ok(TD0Value::U8(
            val.parse().map_err(|_| TD0Error::InvalidInput)?,
        )),
        Some("U16") => Ok(TD0Value::U16(
            val.parse().map_err(|_| TD0Error::InvalidInput)?,
        )),
        Some("U32") => Ok(TD0Value::U32(
            val.parse().map_err(|_| TD0Error::InvalidInput)?,
        )),
        _ => Err(TD0Error::InvalidFieldOrType),
    }
}

pub fn command_set_chunk_item_values(
    td0file: &mut dyn TD0File,
    chunk_name: &str,
    item_num: usize,
    values: &[(String, String)],
) -> TD0Result<Vec<ChunkFieldDiffItem>> {
    let chunk_item = td0file.get_chunk_item_mut(chunk_name, item_num)?;
    let mut diffs: Vec<ChunkFieldDiffItem> = Vec::new();
    for (field, value) in values.iter() {
        let parsed_value = parse_chunk_item_field_value(&*chunk_item, field, value)?;
        let Some(before) = chunk_item.get_value(field) else {
            return Err(TD0Error::InvalidFieldOrType);
        };

        chunk_item.set_value(field, &parsed_value)?;
        let Some(after) = chunk_item.get_value(field) else {
            return Err(TD0Error::InvalidFieldOrType);
        };
        diffs.push(ChunkFieldDiffItem {
            field: chunk_name.to_string(),
            left: Some(before),
            right: Some(after),
            left_name: Some("before".to_string()),
            right_name: Some("after".to_string()),
        });
    }
    Ok(diffs)
}

pub fn command_reorder_chunk_items(
    td0file: &mut dyn TD0File,
    chunk_name: &str,
    new_order: &[usize],
    display_field: &str,
) -> TD0Result<Vec<ChunkFieldDiffItem>> {
    let chunk = td0file
        .get_chunk(chunk_name)
        .ok_or(TD0Error::UnknownChunk)?;
    let mut before: Vec<TD0Value> = Vec::new();
    for idx in 0..chunk.num_items() {
        before.push(
            td0file
                .get_chunk_item(chunk_name, idx)?
                .get_value(&display_field)
                .ok_or_else(|| TD0Error::InvalidChunkItem)?,
        )
    }

    td0file.chunk_items_reorder(chunk_name, new_order)?;

    let mut after: Vec<TD0Value> = Vec::new();
    for idx in 0..chunk.num_items() {
        after.push(
            td0file
                .get_chunk_item(chunk_name, idx)?
                .get_value(&display_field)
                .ok_or_else(|| TD0Error::InvalidChunkItem)?,
        )
    }

    let mut diffs: Vec<ChunkFieldDiffItem> = Vec::new();
    for (idx, (before, after)) in before.iter().zip(&after).enumerate() {
        diffs.push(ChunkFieldDiffItem {
            field: idx.to_string(),
            left: Some(before.clone()),
            right: Some(after.clone()),
            left_name: Some("before".to_string()),
            right_name: Some("after".to_string()),
        });
    }
    Ok(diffs)
}
