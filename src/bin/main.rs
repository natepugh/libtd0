use clap::{Parser, Subcommand};
use libtd0::{TD0Error, TD0File, parse_td0_file};

use std::fs;
use std::path::PathBuf;
use std::process::exit;

mod commands;
use commands::{
    IndexedItemValues, command_compare_chunk_items, command_dump_chunk, command_dump_chunk_item,
    command_dump_chunk_values,
};

mod format;
use format::{
    OutputFormat, create_hexout_settings, fmt_chunk_item_compare_result, fmt_chunk_values_result,
    fmt_hex_dump,
};

const ERR_INVALID_TD0_FILE: Option<i32> = Some(1);
const ERR_INVALID_CHUNK: Option<i32> = Some(2);
const ERR_INVALID_ITEM_INDEX: Option<i32> = Some(3);
const ERR_UNKNOWN: Option<i32> = Some(-1);

fn exit_td0_err(err: &TD0Error, code: Option<i32>) -> ! {
    exit_err(&err.to_string(), code);
}

fn exit_err(err: &str, code: Option<i32>) -> ! {
    eprintln!("{err}");
    exit(code.unwrap_or(ERR_UNKNOWN.unwrap()));
}

fn parse_td0_file_or_exit(file_path: &PathBuf) -> Box<dyn TD0File> {
    let bytes: Vec<u8> = fs::read(file_path).expect("Could not read input file.");
    match parse_td0_file(&bytes) {
        Ok(td0file) => td0file,
        Err(err) => exit_td0_err(&err, ERR_INVALID_TD0_FILE),
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[arg(long, short, value_name = "FILE")]
    file: PathBuf,
    #[command(subcommand)]
    command: Commands,
    #[arg(long, value_enum, default_value_t, global = true)]
    output_format: OutputFormat,
}

fn reindex_chunk_values_for_output(mut values: IndexedItemValues) -> IndexedItemValues {
    // Add 1 to each index from the raw data to get the user-friendly index.
    let mut reindexed = IndexedItemValues::new();
    values.reverse();
    while !values.is_empty() {
        let (idx, item) = values.pop().unwrap();
        reindexed.insert(idx + 1, item);
    }
    reindexed
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
        item_num: Option<usize>,
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<String>>,
    },
    DumpChunk {
        chunk_name: String,
        #[arg(short, long)]
        item_num: Option<usize>,
    },
    ListFields {
        chunk_name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::DisplayManifest {} => {
            // TODO: Determine the output format and support CSV.
            if cli.output_format == OutputFormat::CSV {
                exit_err(
                    "--output-format=csv not supported for display-manifest.",
                    ERR_UNKNOWN,
                );
            }
            let td0file = parse_td0_file_or_exit(&cli.file);
            let manifest_data = td0file.manifest();
            let text_output = match cli.output_format {
                OutputFormat::JSON => {
                    serde_json::to_string(&manifest_data).expect("Could not serialize output.")
                }
                OutputFormat::CSV => {
                    todo!();
                }
                OutputFormat::Text => format!("{manifest_data}"),
            };
            println!("{text_output}");
        }
        Commands::ChunkItemCompare {
            chunk_name_1,
            chunk_name_2,
            item_num,
        } => {
            let Some(inum_reindexed) = item_num.checked_sub(1) else {
                exit_err(
                    "Item index out of bounds. Must be 1 - ? inclusive.",
                    ERR_INVALID_ITEM_INDEX,
                );
            };
            let td0file = parse_td0_file_or_exit(&cli.file);
            let diff =
                command_compare_chunk_items(&*td0file, chunk_name_1, chunk_name_2, inum_reindexed)
                    .unwrap_or_else(|err| {
                        let code = match err {
                            TD0Error::UnknownChunk | TD0Error::InvalidChunk(..) => {
                                ERR_INVALID_CHUNK
                            }
                            TD0Error::OutOfRange => ERR_INVALID_ITEM_INDEX,
                            _ => ERR_UNKNOWN,
                        };
                        exit_td0_err(&err, code);
                    });

            let output = fmt_chunk_item_compare_result(&diff, &cli.output_format)
                .unwrap_or_else(|err| exit_err(err, ERR_UNKNOWN));

            println!("{}", output);
        }
        Commands::ChunkValues {
            chunk_name,
            item_num,
            fields,
        } => {
            let bytes: Vec<u8> = fs::read(cli.file).expect("Could not read input file.");
            let chunk_data_result = match item_num {
                Some(inum) => {
                    // TD0 devices show the user 1-based indexes, while libtd0 uses 0-based indexes.
                    // Convert the indexing to match the lib call here.
                    let Some(inum_reindexed) = inum.checked_sub(1) else {
                        exit_err(
                            "Item index out of bounds. Must be 1 - ? inclusive.",
                            ERR_INVALID_ITEM_INDEX,
                        );
                    };
                    command_dump_chunk_values(
                        &bytes,
                        chunk_name,
                        Some(inum_reindexed..inum_reindexed + 1),
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
                            exit_err(
                                "Item index out of bounds. Must be 1 - ? inclusive.",
                                ERR_INVALID_ITEM_INDEX,
                            );
                        }
                        _ => {
                            exit_td0_err(&e, ERR_INVALID_ITEM_INDEX);
                        }
                    };
                }
            };

            // Reindex chunk_data, consuming the original.
            let chunk_data = reindex_chunk_values_for_output(chunk_data);

            let output =
                fmt_chunk_values_result(&chunk_data, &cli.output_format).unwrap_or_else(|err| {
                    exit_err(err, ERR_UNKNOWN);
                });
            println!("{}", output);
        }
        Commands::DumpChunk {
            chunk_name,
            item_num,
        } => {
            let td0file = parse_td0_file_or_exit(&cli.file);
            let Some(chunk) = td0file.get_chunk(chunk_name) else {
                exit_td0_err(&TD0Error::UnknownChunk, ERR_INVALID_CHUNK);
            };

            // TD0 devices show the user 1-based indexes, while libtd0 uses 0-based indexes.
            // Convert the indexing to match the lib call here.
            let chunk_data_result = match item_num {
                Some(inum) => {
                    let Some(inum_reindexed) = inum.checked_sub(1) else {
                        exit_err(
                            "Item index out of bounds. Must be 1 - ? inclusive.",
                            ERR_INVALID_ITEM_INDEX,
                        );
                    };
                    command_dump_chunk_item(&*td0file, chunk_name, inum_reindexed)
                }
                None => command_dump_chunk(&*td0file, chunk_name),
            };

            let chunk_data = match chunk_data_result {
                Ok(data) => data,
                Err(e) => {
                    exit_td0_err(&e, ERR_INVALID_CHUNK);
                }
            };

            if cli.output_format != OutputFormat::Text {
                eprintln!(
                    "Warning: --output-format option ignored, dump-* commands are text-only."
                );
            }
            println!(
                "{}",
                fmt_hex_dump(chunk_data, &create_hexout_settings(&*chunk))
            );
        }
        Commands::ListFields { chunk_name } => {
            let td0file = parse_td0_file_or_exit(&cli.file);
            let Some(chunk_item) = td0file.get_chunk_item_default(chunk_name) else {
                exit_td0_err(&TD0Error::UnknownChunk, ERR_INVALID_CHUNK);
            };
            let text_output = match cli.output_format {
                OutputFormat::JSON => serde_json::to_string(&chunk_item.list_fields())
                    .expect("Could not serialize output."),
                OutputFormat::CSV => {
                    let mut writer = csv::Writer::from_writer(std::io::stdout());
                    writer
                        .write_record(chunk_item.list_fields().iter())
                        .expect("Could not serialize output.");
                    writer.flush().expect("Could not flush output to stdout.");
                    exit(0);
                }
                OutputFormat::Text => format!("{:?}", chunk_item.list_fields()),
            };
            println!("{}", text_output);
        }
    }
}
