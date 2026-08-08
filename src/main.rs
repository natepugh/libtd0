use clap::{Parser, Subcommand};
use core::fmt;
use hexout::{HexOutSettings, hex_out};
use libtd0::ManifestData;
use libtd0::td0::chunks::{ChunkItem, ChunkItemValue};
use libtd0::td0::header::TD0ManifestTag;
use libtd0::td0::{TD0File, get_chunk_item_fields};
use libtd0_core::result::{TD0Error, TD0Result};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long, short, value_name = "FILE")]
    file: PathBuf,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    DisplayManifest {},
    ChunkItemCompare {
        chunk_name_1: String,
        chunk_name_2: String,
        #[arg(short, long)]
        item_num: usize,
    },
    ChunkValues {
        chunk_name: String,
        #[arg(short, long)]
        item_num: Option<u32>,
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<String>>,
    },
    DumpChunk {
        chunk_name: String,
        #[arg(short, long)]
        item_num: Option<u32>,
    },
    ListFields {
        chunk_name: String,
    },
}

fn create_hexout_settings(chunk_tag: &TD0ManifestTag) -> HexOutSettings {
    HexOutSettings {
        address_origin: chunk_tag.get_pos() as usize,
        address_width: if chunk_tag.get_pos() as usize + chunk_tag.get_length() as usize <= 0xFFFF {
            4
        } else {
            8
        },
        show_ascii: true,      // Show ASCII representation
        show_centerline: true, // Add space in the middle

        ..Default::default()
    }
}

fn command_dump_chunk_item(bytes: &[u8], chunk_name: &str, item_num: usize) -> TD0Result<String> {
    let td0file = TD0File::from_bytes(bytes.to_vec())?;
    let Some(chunk_tag) = td0file.get_tag(chunk_name) else {
        return Err(TD0Error::InvalidChunkError {
            chunk_name: chunk_name.to_string(),
        });
    };

    let item_result = td0file.get_chunk_item_raw(chunk_name, item_num);

    let Some(item_data) = item_result else {
        // The index was out of bounds if we've arrived here.
        // Retrieve the chunk_header in order to provide the actual number of items to
        let Ok(Some(chunk_header)) = td0file.get_chunk_header(&chunk_tag) else {
            return Err(TD0Error::InvalidChunkError {
                chunk_name: chunk_name.to_string(),
            });
        };
        return Err(TD0Error::ItemIndexError {
            len: chunk_header.get_num_items() as usize,
            index: item_num as usize,
        });
    };

    let hexout_settings = create_hexout_settings(&chunk_tag);
    let dump = hex_out(&item_data, &hexout_settings, 0, 0, 0).unwrap();
    Ok(format!("{chunk_tag}\n{}", dump))
}

fn validate_fields(td0file: &TD0File, chunk_name: &str, fields: &Vec<String>) -> TD0Result<()> {
    let all_fields: HashSet<String> = chunk_item_fields_as_strings(td0file, chunk_name)?
        .into_iter()
        .collect();
    let param_fields: HashSet<String> = fields.clone().into_iter().collect();

    let unknown_fields = param_fields.difference(&all_fields);
    let unknown_fields_str = unknown_fields
        .map(|it| it.to_string())
        .collect::<Vec<String>>()
        .join(",");
    if unknown_fields_str.len() > 0 {
        return Err(TD0Error::InputError {
            message: format!("Unknown {chunk_name} fields: [{unknown_fields_str}]"),
        });
    }
    Ok(())
}

fn chunk_item_fields_as_strings(td0file: &TD0File, chunk_name: &str) -> TD0Result<Vec<String>> {
    Ok(td0file
        .get_chunk_item_default(chunk_name)
        .ok_or_else(|| TD0Error::InvalidChunkError {
            chunk_name: chunk_name.to_string(),
        })?
        .get_fields()
        .iter()
        .map(|fld| fld.to_string())
        .collect())
}

fn command_dump_chunk_values(
    bytes: &[u8],
    chunk_name: &str,
    item_num: Option<usize>,
    fields: &Option<Vec<String>>,
) -> TD0Result<String> {
    let td0file = TD0File::from_bytes(bytes.to_vec())?;
    let Some(chunk_tag) = td0file.get_tag(chunk_name) else {
        return Err(TD0Error::InvalidChunkError {
            chunk_name: chunk_name.to_string(),
        });
    };

    let fields = match fields {
        Some(flds) => {
            validate_fields(&td0file, &chunk_name, &flds)?;
            flds.to_owned()
        }
        None => chunk_item_fields_as_strings(&td0file, chunk_name)?,
    };

    let mut output_lines: Vec<String> = Vec::new();

    let item_range = match item_num {
        Some(num) => num..num + 1,
        None => {
            let chunk_hdr = (td0file.get_chunk_header(chunk_tag)?).unwrap();
            0..chunk_hdr.get_num_items() as usize
        }
    };

    for idx in item_range {
        let item_result = td0file.get_chunk_item(&chunk_tag, idx)?;

        if item_result.is_some() {
            let item = item_result.expect("Item is Some");
            let mut vals: Vec<String> = Vec::new();
            for field in fields.iter() {
                vals.push(format!(
                    "{}={}",
                    field,
                    item.get_value(field)
                        .expect("get_value failed for an existing field.")
                ));
            }
            output_lines.push(format!("{}: {}", idx + 1, vals.join(" ")));
        }
    }

    let output = output_lines.join("\n");
    Ok(output)
}

struct ChunkFieldDiffItem {
    index: usize,
    field: &'static str,
    left: Option<ChunkItemValue>,
    right: Option<ChunkItemValue>,
}

impl fmt::Display for ChunkFieldDiffItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.left, &self.right) {
            (Some(_), None) => write!(
                f,
                "{} `{}`: Field not present on right.",
                self.index, self.field
            ),
            (None, Some(_)) => write!(
                f,
                "{} `{}`: Field not present on left.",
                self.index, self.field
            ),
            (Some(left), Some(right)) => {
                if left == right {
                    write!(
                        f,
                        "{} `{}`: Both sides are identical.",
                        self.index, self.field
                    )
                } else {
                    write!(
                        f,
                        "{} `{}`: left = '{}' | right = '{}'",
                        self.index, self.field, left, right
                    )
                }
            }
            (None, None) => write!(
                f,
                "{} {}: Field not present in left or right.",
                self.index, self.field
            ),
        }
    }
}

fn get_chunk_item(
    td0file: &TD0File,
    chunk_name: &str,
    item_num: usize,
) -> TD0Result<Box<dyn ChunkItem>> {
    let Some(chunk_tag) = td0file.get_tag(chunk_name) else {
        return Err(TD0Error::InvalidChunkError {
            chunk_name: chunk_name.to_string(),
        });
    };

    let Some(item) = td0file.get_chunk_item(chunk_tag, item_num)? else {
        let Ok(Some(chunk_header)) = td0file.get_chunk_header(&chunk_tag) else {
            return Err(TD0Error::InvalidChunkError {
                chunk_name: chunk_name.to_string(),
            });
        };
        return Err(TD0Error::ItemIndexError {
            len: chunk_header.get_num_items() as usize,
            index: item_num as usize,
        });
    };
    Ok(item)
}

fn command_compare_chunk_items(
    bytes: &[u8],
    chunk_name_1: &str,
    chunk_name_2: &str,
    item_num: usize,
) -> TD0Result<String> {
    let td0file = TD0File::from_bytes(bytes.to_vec())?;

    let chunk_1_item = get_chunk_item(&td0file, chunk_name_1, item_num)?;
    let chunk_2_item = get_chunk_item(&td0file, chunk_name_2, item_num)?;

    let mut diffs: Vec<ChunkFieldDiffItem> = Vec::new();

    // TODO: Currently this won't show fields from chunk_2 that aren't in chunk_1.
    for (idx, fld_name) in chunk_1_item.get_fields().iter().enumerate() {
        let c1_val = chunk_1_item.get_value(*fld_name);
        let c2_val = chunk_2_item.get_value(*fld_name);
        if c1_val != c2_val {
            diffs.push(ChunkFieldDiffItem {
                index: idx,
                field: *fld_name,
                left: c1_val,
                right: c2_val,
            });
        }
    }

    let output: String = if diffs.len() > 0 {
        diffs
            .iter()
            .map(|diff| diff.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    } else {
        "No differences".into()
    };

    Ok(output)
}

fn command_dump_chunk(bytes: &[u8], chunk_name: &str) -> TD0Result<String> {
    let td0file = TD0File::from_bytes(bytes.to_vec())?;
    let Some(chunk_tag) = td0file.get_tag(chunk_name) else {
        return Err(TD0Error::InvalidChunkError {
            chunk_name: chunk_name.to_string(),
        });
    };

    let chunk_result = td0file.get_chunk_raw(chunk_name);

    let Some(chunk_data) = chunk_result else {
        return Err(TD0Error::InvalidChunkError {
            chunk_name: chunk_name.to_string(),
        });
    };

    let hexout_settings = create_hexout_settings(&chunk_tag);
    let dump = hex_out(&chunk_data, &hexout_settings, 0, 0, 0).unwrap();
    Ok(format!("{chunk_tag}\n{}", dump))
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::DisplayManifest {}) => {
            let bytes: Vec<u8> = fs::read(cli.file).expect("Could not read input file.");
            let manifest_data = ManifestData::from_bytes(&bytes).unwrap();
            println!("{}", manifest_data);
        }
        Some(Commands::ChunkItemCompare {
            chunk_name_1,
            chunk_name_2,
            item_num,
        }) => {
            let bytes: Vec<u8> = fs::read(cli.file).expect("Could not read input file.");
            let Some(inum_reindexed) = item_num.checked_sub(1) else {
                eprintln!("Item index out of bounds. Must be 1 - ? inclusive.");
                exit(1);
            };

            let compare_result = match command_compare_chunk_items(
                &bytes,
                chunk_name_1,
                chunk_name_2,
                inum_reindexed,
            ) {
                Ok(data) => data,
                Err(e) => {
                    match e {
                        TD0Error::ItemIndexError { len, .. } => {
                            eprintln!("Item index out of bounds. Must be 1 - {len} inclusive.");
                        }
                        _ => {
                            eprintln!("{}", e);
                        }
                    };
                    exit(1);
                }
            };
            println!("{compare_result}");
        }
        Some(Commands::ChunkValues {
            chunk_name,
            item_num,
            fields,
        }) => {
            let bytes: Vec<u8> = fs::read(cli.file).expect("Could not read input file.");
            let chunk_data_result = match item_num {
                Some(inum) => {
                    // TD0 devices show the user 1-based indexes, while libtd0 uses 0-based indexes.
                    // Convert the indexing to match the lib call here.
                    let Some(inum_reindexed) = inum.checked_sub(1) else {
                        eprintln!("Item index out of bounds. Must be 1 - ? inclusive.");
                        exit(1);
                    };
                    command_dump_chunk_values(
                        &bytes,
                        chunk_name,
                        Some(inum_reindexed as usize),
                        fields,
                    )
                }
                None => command_dump_chunk_values(&bytes, chunk_name, None, fields),
            };

            let chunk_data = match chunk_data_result {
                Ok(data) => data,
                Err(e) => {
                    match e {
                        TD0Error::ItemIndexError { len, .. } => {
                            eprintln!("Item index out of bounds. Must be 1 - {len} inclusive.");
                            exit(1);
                        }
                        _ => {
                            eprintln!("{}", e);
                            exit(1);
                        }
                    };
                }
            };
            println!("{}", chunk_data);
        }
        Some(Commands::DumpChunk {
            chunk_name,
            item_num,
        }) => {
            let bytes: Vec<u8> = fs::read(cli.file).expect("Could not read input file.");
            // TD0 devices show the user 1-based indexes, while libtd0 uses 0-based indexes.
            // Convert the indexing to match the lib call here.
            let chunk_data_result = match item_num {
                Some(inum) => {
                    let Some(inum_reindexed) = inum.checked_sub(1) else {
                        eprintln!("Item index out of bounds. Must be 1 - ? inclusive.");
                        exit(1);
                    };
                    command_dump_chunk_item(&bytes, chunk_name, inum_reindexed as usize)
                }
                None => command_dump_chunk(&bytes, chunk_name),
            };

            let chunk_data = match chunk_data_result {
                Ok(data) => data,
                Err(e) => {
                    match e {
                        TD0Error::ItemIndexError { len, .. } => {
                            eprintln!("Item index out of bounds. Must be 1 - {len} inclusive.");
                            exit(1);
                        }
                        _ => {
                            eprintln!("{}", e);
                            exit(1);
                        }
                    };
                }
            };
            println!("{}", chunk_data);
        }
        Some(Commands::ListFields { chunk_name }) => {
            let Some(fields) = get_chunk_item_fields(chunk_name) else {
                eprintln!("Unknown chunk tag: {chunk_name}");
                exit(1);
            };
            println!("{:?}", fields);
        }
        None => {
            println!("Hello, world!");
        }
    }
}

//:  &Option<u32>
