// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use core::ops::Range;
use indexmap::{IndexMap, IndexSet};
use serde::Serialize;
use td0::{
    FieldType, TD0Chunk, TD0ChunkItem, TD0Error, TD0File, TD0Result, TD0Value, parse_td0_file,
};

pub type ItemValues = IndexMap<String, TD0Value>;
pub type IndexedItemValues = IndexMap<usize, ItemValues>;

fn validate_chunk_item_range(
    td0file: &TD0File,
    range: &Range<usize>,
    chunk_name: &str,
) -> TD0Result<()> {
    let chunk = td0file
        .chunk(chunk_name)
        .ok_or_else(|| TD0Error::unknown_chunk_error(chunk_name))?;
    let num_items = chunk.num_items();
    if range.end > num_items {
        Err(TD0Error::out_of_range_error(
            0,
            i64::try_from(num_items).map_err(|err| TD0Error::InvalidInput(format!("{err}")))?,
        ))
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
    let requested_fields: IndexSet<&str> = fields.iter().map(String::as_str).collect();

    let unknown_fields = requested_fields.difference(&all_fields);
    let unknown_fields_str = unknown_fields.copied().collect::<Vec<&str>>().join(",");

    if !unknown_fields_str.is_empty() {
        return Err(TD0Error::InvalidInput(format!(
            "Unknown {chunk_name} fields: [{unknown_fields_str}]"
        )));
    }
    Ok(())
}

#[expect(
    clippy::ref_option,
    reason = "Accepts &Option directly from clap command."
)]
fn requested_fields_or_all<'a>(
    td0file: &'a TD0File,
    chunk_name: &str,
    fields: &'a Option<Vec<String>>,
) -> TD0Result<Vec<&'a str>> {
    let chunk = td0file
        .chunk(chunk_name)
        .ok_or_else(|| TD0Error::unknown_chunk_error(chunk_name))?;
    let default_item = chunk.default_item();

    match fields {
        Some(flds) => {
            validate_fields(&*default_item, flds, chunk_name)?;
            Ok(flds.iter().map(String::as_str).collect())
        }
        None => Ok(default_item.list_fields().into()),
    }
}

pub fn command_dump_chunk_item<'a>(
    td0file: &'a TD0File,
    chunk_name: &'a str,
    item_num: usize,
) -> TD0Result<&'a [u8]> {
    let chunk = td0file
        .chunk(chunk_name)
        .ok_or_else(|| TD0Error::unknown_chunk_error(chunk_name))?;
    chunk.item_at_ref(item_num).map(|it| it.as_bytes())
}

pub fn command_dump_chunk(td0file: &TD0File, chunk_name: &str) -> TD0Result<Vec<u8>> {
    let chunk = td0file
        .chunk(chunk_name)
        .ok_or_else(|| TD0Error::unknown_chunk_error(chunk_name))?;
    Ok(Vec::from(chunk))
}

#[expect(
    clippy::ref_option,
    reason = "Accepts &Option directly from clap command."
)]
pub fn command_dump_chunk_values(
    bytes: &[u8],
    chunk_name: &str,
    item_range: Option<Range<usize>>,
    fields: &Option<Vec<String>>,
) -> TD0Result<IndexedItemValues> {
    let td0file = parse_td0_file(bytes)?;
    let chunk = td0file
        .chunk(chunk_name)
        .ok_or_else(|| TD0Error::unknown_chunk_error(chunk_name))?;
    //let chunk = get_chunk_or_error(&*td0file, chunk_name)?;
    let fields: Vec<&str> = requested_fields_or_all(&td0file, chunk_name, fields)?;
    let item_range = item_range.unwrap_or_else(|| 0..chunk.num_items());
    validate_chunk_item_range(&td0file, &item_range, chunk_name)?;

    let mut output: IndexedItemValues = IndexedItemValues::new();

    for idx in item_range {
        if let Ok(item) = chunk.item_at_ref(idx) {
            let mut vals: ItemValues = ItemValues::new();
            for field in &fields {
                vals.insert(
                    field.to_string(),
                    item.field_value(field)
                        .expect("field_value failed for an existing field."),
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

pub fn parse_chunk_item_field_value(
    chunk_item: &dyn TD0ChunkItem,
    field: &str,
    val: &str,
) -> TD0Result<TD0Value> {
    match chunk_item.field_type(field) {
        Some(FieldType::Decimal { .. }) => {
            Ok(TD0Value::Decimal(val.parse().map_err(|_| {
                TD0Error::InvalidInput(format!("invalid value for field '{field}'"))
            })?))
        }
        Some(FieldType::I16 { .. }) => {
            Ok(TD0Value::I16(val.parse().map_err(|_| {
                TD0Error::InvalidInput(format!("invalid value for field '{field}'"))
            })?))
        }
        Some(FieldType::AsciiText { .. } | FieldType::EnumText { .. }) => {
            Ok(TD0Value::Text(val.to_string()))
        }
        Some(FieldType::I8 { .. }) => {
            Ok(TD0Value::I8(val.parse().map_err(|_| {
                TD0Error::InvalidInput(format!("invalid value for field '{field}'"))
            })?))
        }
        // Slice parsing from String not implemented.
        Some(FieldType::U8 { .. }) => {
            Ok(TD0Value::U8(val.parse().map_err(|_| {
                TD0Error::InvalidInput(format!("invalid value for field '{field}'"))
            })?))
        }
        Some(FieldType::U16 { .. }) => {
            Ok(TD0Value::U16(val.parse().map_err(|_| {
                TD0Error::InvalidInput(format!("invalid value for field '{field}'"))
            })?))
        }
        Some(FieldType::U32 { .. }) => {
            Ok(TD0Value::U32(val.parse().map_err(|_| {
                TD0Error::InvalidInput(format!("invalid value for field '{field}'"))
            })?))
        }
        _ => Err(TD0Error::DataType(format!(
            "invalid data type for field '{field}"
        ))),
    }
}

pub fn command_set_chunk_item_values(
    td0file: &mut TD0File,
    chunk_name: &str,
    item_num: usize,
    values: &[(String, String)],
) -> TD0Result<Vec<ChunkFieldDiffItem>> {
    // let chunk_name = &ChunkTagString::try_from(chunk_name)?;
    let chunk: &mut TD0Chunk = td0file
        .chunk_mut(chunk_name)
        .ok_or_else(|| TD0Error::unknown_chunk_error(chunk_name))?;
    //let Some(chunk) = td0file.chunk_mut(&chunk_name);
    let chunk_item = chunk.item_at_mut(item_num)?;
    let mut diffs: Vec<ChunkFieldDiffItem> = Vec::new();
    for (field, value) in values {
        let parsed_value = parse_chunk_item_field_value(&*chunk_item, field, value)?;
        let Some(before) = chunk_item.field_value(field) else {
            return Err(TD0Error::InvalidField(field.clone()));
        };

        chunk_item.set_field_value(field, parsed_value)?;
        let Some(after) = chunk_item.field_value(field) else {
            return Err(TD0Error::InvalidField(field.clone()));
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
    td0file: &mut TD0File,
    chunk_name: &str,
    new_order: &[usize],
    display_field: &str,
) -> TD0Result<Vec<ChunkFieldDiffItem>> {
    let chunk: &mut TD0Chunk = td0file
        .chunk_mut(chunk_name)
        .ok_or_else(|| TD0Error::unknown_chunk_error(chunk_name))?;
    let mut before: Vec<TD0Value> = Vec::new();
    let num_items = chunk.num_items();

    for idx in 0..num_items {
        before.push(
            chunk
                .item_at_ref(idx)?
                .field_value(display_field)
                .ok_or(TD0Error::InvalidField(display_field.to_string()))?,
        );
    }

    chunk.items_reorder(new_order)?;

    let mut after: Vec<TD0Value> = Vec::new();
    for idx in 0..num_items {
        after.push(
            chunk
                .item_at_ref(idx)?
                .field_value(display_field)
                .ok_or(TD0Error::InvalidField(display_field.to_string()))?,
        );
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
