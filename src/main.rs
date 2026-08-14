use clap::{Parser, Subcommand};
use core::fmt;
use hexout::{HexOutSettings, hex_out};
use libtd0::{TD0Error, TD0File, TD0Result, TD0Value, parse_td0_file};
use libtd0_core::TD0Chunk;

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

const ERR_INVALID_TD0_FILE: Option<i32> = Some(1);
const ERR_INVALID_CHUNK: Option<i32> = Some(2);

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

fn get_chunk_or_error(td0file: &dyn TD0File, chunk_name: &str) -> TD0Result<Box<dyn TD0Chunk>> {
    match td0file.get_chunk(chunk_name) {
        Some(chunk) => Ok(chunk),
        None => Err(TD0Error::UnknownChunk),
    }
}

fn create_hexout_settings(chunk: &dyn TD0Chunk) -> HexOutSettings {
    HexOutSettings {
        address_origin: chunk.pos(),
        address_width: if chunk.pos() + chunk.size() as usize <= 0xFFFF {
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
    // let td0file = TD0File::from_bytes(bytes.to_vec())?;
    let td0file = parse_td0_file(bytes)?;
    let chunk = get_chunk_or_error(&*td0file, chunk_name)?;
    let item_raw = td0file.get_chunk_item_raw(chunk_name, item_num)?;
    let hexout_settings = create_hexout_settings(&*chunk);
    let dump = hex_out(item_raw, &hexout_settings, 0, 0, 0).unwrap();
    Ok(format!("{}\n{}", dump, chunk_name))
}

fn validate_fields(td0file: &dyn TD0File, chunk_name: &str, fields: &[String]) -> TD0Result<()> {
    let Some(chunk_item) = td0file.get_chunk_item_default(chunk_name) else {
        exit_err(&TD0Error::UnknownChunk, ERR_INVALID_CHUNK);
    };
    let all_fields: HashSet<&str> = chunk_item.list_fields().iter().map(|f| *f).collect();
    let requested_fields: HashSet<&str> = fields.iter().map(|fld| fld.as_str()).collect();

    let unknown_fields = requested_fields.difference(&all_fields);
    let unknown_fields_str = unknown_fields.map(|f| *f).collect::<Vec<&str>>().join(",");

    if !unknown_fields_str.is_empty() {
        return Err(TD0Error::InvalidInputWithMessage(format!(
            "Unknown {chunk_name} fields: [{unknown_fields_str}]"
        )));
    }
    Ok(())
}

fn command_dump_chunk_values(
    bytes: &[u8],
    chunk_name: &str,
    item_num: Option<usize>,
    fields: &Option<Vec<String>>,
) -> TD0Result<String> {
    let td0file = parse_td0_file(bytes)?;
    let chunk = get_chunk_or_error(&*td0file, chunk_name)?;

    let fields: Vec<&str> = match fields {
        Some(flds) => {
            validate_fields(&*td0file, chunk_name, &flds)?;
            flds.iter().map(|st| st.as_str()).collect()
        }
        None => td0file
            .get_chunk_item_default(chunk_name)
            .expect("chunk exists.")
            .list_fields()
            .into(),
    };

    let mut output_lines: Vec<String> = Vec::new();

    let item_range = match item_num {
        Some(num) => num..num + 1,
        None => 0..chunk.num_items(),
    };

    for idx in item_range {
        if let Ok(item) = td0file.get_chunk_item(chunk_name, idx) {
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
    left: Option<TD0Value>,
    right: Option<TD0Value>,
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

fn command_compare_chunk_items(
    bytes: &[u8],
    chunk_name_1: &str,
    chunk_name_2: &str,
    item_num: usize,
) -> TD0Result<String> {
    let td0file = parse_td0_file(bytes)?;

    let chunk_1_item = td0file.get_chunk_item(chunk_name_1, item_num)?;
    let chunk_2_item = td0file.get_chunk_item(chunk_name_2, item_num)?;

    let mut diffs: Vec<ChunkFieldDiffItem> = Vec::new();

    // TODO: Currently this won't show fields from chunk_2 that aren't in chunk_1.
    for (idx, fld_name) in chunk_1_item.list_fields().iter().enumerate() {
        let c1_val = chunk_1_item.get_value(fld_name);
        let c2_val = chunk_2_item.get_value(fld_name);
        if c1_val != c2_val {
            diffs.push(ChunkFieldDiffItem {
                index: idx,
                field: fld_name,
                left: c1_val,
                right: c2_val,
            });
        }
    }

    let output: String = if diffs.is_empty() {
        "No differences".into()
    } else {
        diffs
            .iter()
            .map(|diff| diff.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    };

    Ok(output)
}

fn command_dump_chunk(bytes: &[u8], chunk_name: &str) -> TD0Result<String> {
    let td0file = parse_td0_file(bytes)?;
    let chunk = get_chunk_or_error(&*td0file, chunk_name)?;
    let Some(chunk_raw) = td0file.get_chunk_raw(chunk_name) else {
        return Err(TD0Error::UnknownChunk);
    };

    let hexout_settings = create_hexout_settings(&*chunk);
    let dump = hex_out(chunk_raw, &hexout_settings, 0, 0, 0).unwrap();
    Ok(format!("{chunk_name}\n{}", dump))
}

fn exit_err(err: &TD0Error, code: Option<i32>) -> ! {
    println!("{err}");
    exit(code.unwrap_or(-1));
}

fn parse_td0_file_or_exit(file_path: &PathBuf) -> Box<dyn TD0File> {
    let bytes: Vec<u8> = fs::read(file_path).expect("Could not read input file.");
    match parse_td0_file(&bytes) {
        Ok(td0file) => td0file,
        Err(err) => exit_err(&err, ERR_INVALID_TD0_FILE),
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::DisplayManifest {}) => {
            let td0file = parse_td0_file_or_exit(&cli.file);
            let manifest_data = td0file.manifest();
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
                        TD0Error::OutOfRange => {
                            eprintln!(
                                "Item index out of bounds. Must be 1 - num_chunk_items inclusive."
                            );
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
                        TD0Error::OutOfRange => {
                            eprintln!(
                                "Item index out of bounds. Must be 1 - num_chunk_items inclusive."
                            );
                        }
                        _ => {
                            eprintln!("{}", e);
                        }
                    };
                    exit(1);
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
                        TD0Error::OutOfRange => {
                            eprintln!(
                                "Item index out of bounds. Must be 1 - num_chunk_items inclusive."
                            );
                        }
                        _ => {
                            eprintln!("{}", e);
                        }
                    };
                    exit(1);
                }
            };
            println!("{}", chunk_data);
        }
        Some(Commands::ListFields { chunk_name }) => {
            let td0file = parse_td0_file_or_exit(&cli.file);
            let Some(chunk_item) = td0file.get_chunk_item_default(chunk_name) else {
                exit_err(&TD0Error::UnknownChunk, ERR_INVALID_CHUNK);
            };
            println!("{:?}", chunk_item.list_fields());
        }
        None => {
            println!("Hello, world!");
        }
    }
}
