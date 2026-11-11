// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::{Parser, Subcommand};
use td0::{ChunkTagString, TD0Chunk, TD0Error, TD0File, TD0Result, parse_td0_file};

use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

mod commands;
use commands::{
    IndexedItemValues, command_dump_chunk, command_dump_chunk_item, command_dump_chunk_values,
    command_reorder_chunk_items, command_set_chunk_item_values,
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

fn parse_td0_file_or_exit(file_path: &PathBuf) -> TD0File {
    let bytes: Vec<u8> = fs::read(file_path).expect("Could not read input file.");
    match parse_td0_file(&bytes) {
        Ok(td0file) => td0file,
        Err(err) => exit_td0_err(&err, ERR_INVALID_TD0_FILE),
    }
}

fn get_chunk_or_exit<'a>(td0file: &'a TD0File, chunk_name: &str) -> &'a TD0Chunk {
    let chunk_name = ChunkTagString::try_from(chunk_name)
        .unwrap_or_else(|_| exit_err("invalid chunk name", ERR_INVALID_CHUNK));

    td0file
        .chunk(&chunk_name)
        .unwrap_or_else(|| exit_err("invalid chunk", ERR_INVALID_CHUNK))
}

fn get_chunk_or_exit_mut<'a>(td0file: &'a mut TD0File, chunk_name: &str) -> &'a mut TD0Chunk {
    let chunk_name: ChunkTagString = ChunkTagString::try_from(chunk_name)
        .unwrap_or_else(|_| exit_err("invalid chunk name", ERR_INVALID_CHUNK));

    td0file
        .chunk_mut(&chunk_name)
        .unwrap_or_else(|| exit_err("invalid chunk", ERR_INVALID_CHUNK))
}

fn reindex_item_num_or_exit(val: usize) -> usize {
    // TD0 devices show the user 1-based indexes, while libtd0 uses 0-based indexes.
    // Convert the indexing to match the lib call here.
    if val == 0 {
        exit_err(
            "Item index out of bounds. Must be 1 - ? inclusive.",
            ERR_INVALID_ITEM_INDEX,
        );
    }
    val - 1
}

fn write_td0_file(td0file: &TD0File, dest: &PathBuf) -> TD0Result<()> {
    let output_bytes: Vec<u8> = td0file.into();

    fs::write(dest, output_bytes).map_err(|err| TD0Error::ValidationFailed(err.to_string()))
}

fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("no `=` found in `{s}`"))?;
    #[expect(clippy::string_slice, reason = "Position is retrieved above.")]
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
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
    ChunkItemsReorder {
        chunk_name: String,
        display_field: String,
        #[arg(value_delimiter = ',')]
        new_order: Vec<usize>,
        #[arg(short, long)]
        output_file: PathBuf,
    },
    ChunkItemsSwap {
        chunk_name: String,
        item_1: usize,
        item_2: usize,
        #[arg(short, long)]
        output_file: PathBuf,
    },
    SetChunkItemValues {
        chunk_name: String,
        #[arg(short, long)]
        item_num: usize,
        #[arg(short = 'v', long = "values", value_name = "key=value", value_parser = parse_key_val::<String, String>)]
        values: Vec<(String, String)>,
        output_file: PathBuf,
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

#[expect(clippy::too_many_lines)]
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::DisplayManifest {} => {
            // TODO: Determine the output format and support CSV.
            if cli.output_format == OutputFormat::Csv {
                exit_err(
                    "--output-format=csv not supported for display-manifest.",
                    ERR_UNKNOWN,
                );
            }
            let td0file = parse_td0_file_or_exit(&cli.file);
            let manifest_data = td0file.manifest();
            let text_output = match cli.output_format {
                OutputFormat::Json => {
                    serde_json::to_string(&manifest_data).expect("Could not serialize output.")
                }
                OutputFormat::Csv => {
                    unreachable!();
                }
                OutputFormat::Text => format!("{manifest_data}"),
            };
            println!("{text_output}");
        }
        Commands::ChunkValues {
            chunk_name,
            item_num,
            fields,
        } => {
            let bytes: Vec<u8> = fs::read(cli.file).expect("Could not read input file.");
            let chunk_data_result = item_num.map_or_else(
                || command_dump_chunk_values(&bytes, chunk_name, None, fields),
                |inum| {
                    let inum_reindexed = reindex_item_num_or_exit(inum);

                    command_dump_chunk_values(
                        &bytes,
                        chunk_name,
                        Some(inum_reindexed..inum_reindexed + 1),
                        fields,
                    )
                },
            );

            let chunk_data = match chunk_data_result {
                Ok(data) => data,
                Err(e) => {
                    match e {
                        TD0Error::OutOfRange(err) => {
                            exit_err(
                                format!("Item index {err}.").as_str(),
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
                fmt_chunk_values_result(&chunk_data, cli.output_format).unwrap_or_else(|err| {
                    exit_err(err, ERR_UNKNOWN);
                });
            println!("{output}");
        }
        Commands::DumpChunk {
            chunk_name,
            item_num,
        } => {
            let td0file = parse_td0_file_or_exit(&cli.file);

            // TD0 devices show the user 1-based indexes, while libtd0 uses 0-based indexes.
            // Convert the indexing to match the lib call here.
            let chunk_data_result = item_num.as_ref().map_or_else(
                || command_dump_chunk(&td0file, chunk_name),
                |inum| {
                    let Some(inum_reindexed) = inum.checked_sub(1) else {
                        exit_err(
                            "Item index out of bounds. Must be 1 - ? inclusive.",
                            ERR_INVALID_ITEM_INDEX,
                        );
                    };
                    command_dump_chunk_item(&td0file, chunk_name, inum_reindexed)
                        .map(<[u8]>::to_vec)
                },
            );

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
                fmt_hex_dump(
                    chunk_data.as_slice(),
                    &create_hexout_settings(&td0file, chunk_name)
                )
            );
        }
        Commands::ListFields { chunk_name } => {
            let td0file = parse_td0_file_or_exit(&cli.file);
            let chunk = get_chunk_or_exit(&td0file, chunk_name);

            let chunk_item = chunk.default_item();

            let text_output = match cli.output_format {
                OutputFormat::Json => serde_json::to_string(&chunk_item.list_fields())
                    .expect("Could not serialize output."),
                OutputFormat::Csv => {
                    let mut writer = csv::Writer::from_writer(std::io::stdout());
                    writer
                        .write_record(chunk_item.list_fields().iter())
                        .expect("Could not serialize output.");
                    writer.flush().expect("Could not flush output to stdout.");
                    exit(0);
                }
                OutputFormat::Text => format!("{:?}", chunk_item.list_fields()),
            };
            println!("{text_output}");
        }
        Commands::SetChunkItemValues {
            chunk_name,
            item_num,
            values,
            output_file,
        } => {
            let mut td0file = parse_td0_file_or_exit(&cli.file);
            let inum_reindexed = reindex_item_num_or_exit(*item_num);
            let diffs =
                command_set_chunk_item_values(&mut td0file, chunk_name, inum_reindexed, values)
                    .unwrap_or_else(|err| {
                        exit_td0_err(&err, ERR_UNKNOWN);
                    });

            let output = fmt_chunk_item_compare_result(&diffs, cli.output_format)
                .unwrap_or_else(|err| exit_err(err, ERR_UNKNOWN));

            println!("{output}");

            write_td0_file(&td0file, output_file)
                .map_err(|err| exit_td0_err(&err, ERR_INVALID_TD0_FILE));
            println!("Modified data written to file: {}", output_file.display());
        }
        Commands::ChunkItemsReorder {
            chunk_name,
            display_field,
            new_order,
            output_file,
        } => {
            let mut td0file = parse_td0_file_or_exit(&cli.file);
            let diffs =
                command_reorder_chunk_items(&mut td0file, chunk_name, new_order, display_field)
                    .unwrap_or_else(|err| {
                        exit_td0_err(&err, ERR_UNKNOWN);
                    });

            let output = fmt_chunk_item_compare_result(&diffs, cli.output_format)
                .unwrap_or_else(|err| exit_err(err, ERR_UNKNOWN));

            println!("{output}");

            write_td0_file(&td0file, output_file)
                .map_err(|err| exit_td0_err(&err, ERR_INVALID_TD0_FILE));
            println!("Modified data written to file: {}", output_file.display());
        }
        Commands::ChunkItemsSwap {
            chunk_name,
            item_1,
            item_2,
            output_file,
        } => {
            let mut td0file = parse_td0_file_or_exit(&cli.file);
            let chunk: &mut TD0Chunk = get_chunk_or_exit_mut(&mut td0file, chunk_name);
            let inum_1_reindexed = reindex_item_num_or_exit(*item_1);
            let inum_2_reindexed = reindex_item_num_or_exit(*item_2);

            chunk
                .swap_items(inum_1_reindexed, inum_2_reindexed)
                .map_err(|err| exit_td0_err(&err, ERR_INVALID_TD0_FILE));

            write_td0_file(&td0file, output_file)
                .map_err(|err| exit_td0_err(&err, ERR_INVALID_TD0_FILE));
            println!("Modified data written to file: {}", output_file.display());
        }
    }
}
