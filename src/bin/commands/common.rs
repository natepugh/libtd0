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
    let all_fields: IndexSet<&str> = chunk_item.list_fields().iter().map(|f| *f).collect();
    let requested_fields: IndexSet<&str> = fields.iter().map(|fld| fld.as_str()).collect();

    let unknown_fields = requested_fields.difference(&all_fields);
    let unknown_fields_str = unknown_fields.map(|f| *f).collect::<Vec<&str>>().join(",");

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
    Ok(td0file.get_chunk_item_raw(chunk_name, item_num)?)
}

pub fn command_dump_chunk<'a>(td0file: &'a dyn TD0File, chunk_name: &str) -> TD0Result<&'a [u8]> {
    Ok(td0file
        .get_chunk_raw(chunk_name)
        .ok_or(TD0Error::UnknownChunk)?)
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
    pub field: &'static str,
    pub left: Option<TD0Value>,
    pub right: Option<TD0Value>,
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
        IndexSet::from_iter(chunk_1_item.list_fields().iter().map(|fld| *fld));
    let chunk_2_fields: IndexSet<&str> =
        IndexSet::from_iter(chunk_2_item.list_fields().iter().map(|fld| *fld));

    let mut all_fields: IndexSet<&str> = chunk_1_fields.clone();
    all_fields.extend(chunk_2_fields.iter());

    let mut diffs: Vec<ChunkFieldDiffItem> = Vec::new();
    for field in all_fields.iter() {
        let c1_val = chunk_1_item.get_value(*field);
        let c2_val = chunk_2_item.get_value(*field);
        if c1_val != c2_val {
            diffs.push(ChunkFieldDiffItem {
                field: *field,
                left: c1_val,
                right: c2_val,
            });
        }
    }

    Ok(diffs)
}
