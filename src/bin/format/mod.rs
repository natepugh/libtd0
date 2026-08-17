use hexout::{HexOutSettings, hex_out};

use super::commands::IndexedItemValues;
use crate::commands::ChunkFieldDiffItem;
use clap::ValueEnum;
use libtd0_core::TD0Chunk;

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum OutputFormat {
    Csv,
    #[default]
    Text,
    Json,
}

type FormatResult = core::result::Result<String, &'static str>;

pub fn fmt_hex_dump(bytes: &[u8], hexout_settings: &HexOutSettings) -> String {
    hex_out(bytes, hexout_settings, 0, 0, 0).expect("incorrect hexout invocation.")
}

pub fn create_hexout_settings(chunk: &dyn TD0Chunk) -> HexOutSettings {
    HexOutSettings {
        address_origin: chunk.pos(),
        address_width: if chunk.pos() + chunk.size() <= 0xFFFF {
            4
        } else {
            8
        },
        show_ascii: true,      // Show ASCII representation
        show_centerline: true, // Add space in the middle

        ..Default::default()
    }
}

impl core::fmt::Display for ChunkFieldDiffItem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match (&self.left, &self.right) {
            (Some(_), None) => write!(f, "`{}`: Field not present on right.", self.field),
            (None, Some(_)) => write!(f, "`{}`: Field not present on left.", self.field),
            (Some(left), Some(right)) => {
                if left == right {
                    write!(f, "`{}`: Both sides are identical.", self.field)
                } else {
                    write!(
                        f,
                        "`{}`: left = '{}' | right = '{}'",
                        self.field, left, right
                    )
                }
            }
            (None, None) => write!(f, "{}: Field not present in left or right.", self.field),
        }
    }
}

pub fn fmt_chunk_item_compare_result(
    diffs: &[ChunkFieldDiffItem],
    format: &OutputFormat,
) -> FormatResult {
    match format {
        OutputFormat::Text => {
            if diffs.is_empty() {
                Ok("No differences".to_string())
            } else {
                Ok(diffs
                    .iter()
                    .map(|diff| diff.to_string())
                    .collect::<Vec<String>>()
                    .join("\n"))
            }
        }
        OutputFormat::Json => {
            Ok(serde_json::to_string(&diffs).map_err(|_| "unable to format JSON.")?)
        }
        _ => Err("--output-format: Only `text` and `json` are supported for this command."),
    }
}

fn header_and_rows_from_values_result<'a>(
    vals: &'a IndexedItemValues,
) -> (Vec<&'a str>, Vec<Vec<String>>) {
    static IDX_HEADER: &str = "index";

    let mut header: Vec<&'a str> = Vec::new();
    let mut rows: Vec<Vec<String>> = Vec::new();

    if vals.is_empty() {
        return (header, rows);
    }

    header.push(IDX_HEADER);
    let (_, first) = vals.first().unwrap();
    header.extend(first.keys().map(|key| key.as_str()));

    for (idx, item) in vals.iter() {
        let mut row: Vec<String> = Vec::new();
        row.push(idx.to_string());
        for value in item.values() {
            row.push(value.to_string());
        }
        rows.push(row);
    }

    (header, rows)
}

pub fn fmt_chunk_values_result(vals: &IndexedItemValues, format: &OutputFormat) -> FormatResult {
    match format {
        OutputFormat::Text => {
            let mut lines: Vec<String> = Vec::new();
            for (idx, item) in vals.iter() {
                // let item_str : String = item.iter().map(|(field, val)| format!("{field}={val}").collect::<Vec<String>>().join("  "));
                let item_string = item
                    .iter()
                    .map(|(field, val)| format!("{field}={val}"))
                    .collect::<Vec<String>>()
                    .join("  ");
                lines.push(format!("{idx}: {item_string}"));
            }
            Ok(lines.join(","))
        }
        OutputFormat::Json => {
            Ok(serde_json::to_string(&vals).map_err(|_| "unable to format JSON.")?)
        }
        OutputFormat::Csv => {
            let (header, rows) = header_and_rows_from_values_result(vals);
            let mut output: Vec<u8> = Vec::new();
            let mut writer = csv::Writer::from_writer(&mut output);
            writer
                .write_record(&header)
                .map_err(|_| "unable to write csv header.")?;
            for row in rows {
                writer
                    .write_record(&row)
                    .map_err(|_| "unable to write all csv rows.")?;
            }
            writer.flush().map_err(|_| "could not flush csv output.")?;
            Ok(String::from_utf8_lossy(writer.get_ref()).to_string())
        }
    }
}
