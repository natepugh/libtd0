use libtd0::ManifestData;
use libtd0::td0::{TD0File, get_chunk_item_fields};
use libtd0::td0::header::TD0ManifestTag;
use libtd0::td0::result::{TD0Error,TD0Result};
use std::process::exit;
use std::{fs};
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use hexout::{HexOutSettings,hex_out};


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
    ChunkValues {
        chunk_name: String,
        #[arg(short, long)]
        item_num: Option<u32>,
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
        address_width: if chunk_tag.get_pos() as usize + chunk_tag.get_length() as usize <= 0xFFFF { 4 } else { 8 },
        show_ascii: true,        // Show ASCII representation
        show_centerline: true,   // Add space in the middle

        ..Default::default()
    }
}

fn command_dump_chunk_item(bytes: &[u8], chunk_name: &str, item_num: usize) -> TD0Result<String> {
    let td0file = TD0File::from_bytes(bytes.to_vec())?;
    let Some(chunk_tag) = td0file.get_tag(chunk_name) else {
        return Err(TD0Error::InvalidChunkError { chunk_name: chunk_name.to_string() });
    };

    let item_result = td0file.get_chunk_item_raw(chunk_name, item_num);

    let Some(item_data) = item_result else {
        // The index was out of bounds if we've arrived here.
        // Retrieve the chunk_header in order to provide the actual number of items to 
        let Ok(Some(chunk_header )) = td0file.get_chunk_header(&chunk_tag) else {
            return Err(TD0Error::InvalidChunkError { chunk_name: chunk_name.to_string() });
        };
        return Err(TD0Error::ItemIndexError { len: chunk_header.get_num_items() as usize, index: item_num as usize}); 
    };

    let hexout_settings = create_hexout_settings(&chunk_tag);
    let dump = hex_out(&item_data, &hexout_settings, 0, 0, 0).unwrap();
    Ok(format!("{chunk_tag}\n{}", dump))
}

fn command_dump_chunk_values(bytes: &[u8], chunk_name: &str, item_num: Option<usize>) -> TD0Result<String> {
    let td0file = TD0File::from_bytes(bytes.to_vec())?;
    let Some(chunk_tag) = td0file.get_tag(chunk_name) else {
        return Err(TD0Error::InvalidChunkError { chunk_name: chunk_name.to_string() });
    };

    let mut output_lines : Vec<String> = Vec::new();

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
            let item = item_result.unwrap();
            let mut vals: Vec<String> = Vec::new();
            for field in item.get_fields() {
                vals.push(format!("{}={}", *field, item.get_value(*field).unwrap()));
            }
            output_lines.push(format!("{}: {}", idx + 1, vals.join(" ")));
        }
    }

    let output = output_lines.join("\n");
    Ok(output)
}

fn command_dump_chunk(bytes: &[u8], chunk_name: &str) -> TD0Result<String> {
    let td0file = TD0File::from_bytes(bytes.to_vec())?;
    let Some(chunk_tag) = td0file.get_tag(chunk_name) else {
        return Err(TD0Error::InvalidChunkError { chunk_name: chunk_name.to_string() });
    };

    let chunk_result = td0file.get_chunk_raw(chunk_name);

    let Some(chunk_data ) = chunk_result else {
        return Err(TD0Error::InvalidChunkError { chunk_name: chunk_name.to_string() });
    };

    let hexout_settings = create_hexout_settings(&chunk_tag);
    let dump = hex_out(&chunk_data, &hexout_settings, 0, 0, 0).unwrap();
    Ok(format!("{chunk_tag}\n{}", dump))
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::DisplayManifest{}) => {
            let bytes: Vec<u8> = fs::read(cli.file).expect("Could not read input file.");
            let manifest_data = ManifestData::from_bytes(&bytes).unwrap();
            println!("{}", manifest_data);
        },
        Some(Commands::ChunkValues { chunk_name, item_num }) => {
            let bytes: Vec<u8> = fs::read(cli.file).expect("Could not read input file.");
            let chunk_data_result = match item_num {
                Some(inum) => {
                    // TD0 devices show the user 1-based indexes, while libtd0 uses 0-based indexes.
                    // Convert the indexing to match the lib call here.
                    let Some(inum_reindexed) = inum.checked_sub(1) else {
                        eprintln!("Item index out of bounds. Must be 1 - ? inclusive.");
                        exit(1);
                    };
                    command_dump_chunk_values(&bytes, chunk_name,Some(inum_reindexed as usize)) 
                },
                None => { 
                    command_dump_chunk_values(&bytes, chunk_name, None)
                },
            };

            let chunk_data = match chunk_data_result {
                Ok(data) => data,
                Err(e) => { 
                    match e {
                        TD0Error::ItemIndexError{len, .. } => {
                            eprintln!("Item index out of bounds. Must be 1 - {len} inclusive.");
                            exit(1);
                        },
                        _ => { eprintln!("{}", e); exit(1); }
                    };
                }
            };
            println!("{}", chunk_data);
        }
        Some(Commands::DumpChunk { chunk_name , item_num }) => {
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
                },
                None => command_dump_chunk(&bytes, chunk_name),
            };

            let chunk_data = match chunk_data_result {
                Ok(data) => data,
                Err(e) => { 
                    match e {
                        TD0Error::ItemIndexError{len, .. } => {
                            eprintln!("Item index out of bounds. Must be 1 - {len} inclusive.");
                            exit(1);
                        },
                        _ => { eprintln!("{}", e); exit(1); }
                    };
                }
            };
            println!("{}", chunk_data);
        },
        Some(Commands::ListFields { chunk_name }) => {
            let Some(fields) = get_chunk_item_fields(chunk_name) else {
                eprintln!("Unknown chunk tag: {chunk_name}");
                exit(1);
            };
            println!("{:?}", fields);
        },
        None => {
            println!("Hello, world!");
        }
    }
}

//:  &Option<u32>