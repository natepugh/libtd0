// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later
#![cfg(test)]

use super::td0chunk::{ChunkHeader, TD0Chunk};
use super::td0file::{
    ERR_MANIFEST_SIZE, ERR_MANIFEST_TAG_DUPLICATE, ERR_MANIFEST_TAG_INVALID, TD0File,
};
use super::*;
use crate::{TD0BackupType, TD0ChunkItem, TD0DeviceModel, TD0Manifest};
use td0_derive::TD0ChunkItemDerive;

use hexout::{HexOutSettings, hex_out};
use md5::Digest as _;
use pretty_assertions::assert_eq;
use std::assert_matches;
use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;
use zerocopy::{IntoBytes, LittleEndian, U16, U32};
use zerocopy_derive::{FromBytes, Immutable, KnownLayout};

#[derive(Clone, Copy, Debug, FromBytes, Immutable, IntoBytes, KnownLayout, TD0ChunkItemDerive)]
#[repr(C, packed)]
#[td0_struct(td0_core_crate = "crate")]
pub struct TGLaItem {
    /* Represents a tag in the tag library. */
    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    name: [u8; 16],
}

fn get_test_td0(test_filename: &str) -> Vec<u8> {
    let mut test_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("test/data/");
    test_path.push(test_filename);
    fs::read(test_path).unwrap()
}

static FACTORY_BACKUP_FIRMWARE_BYTES: LazyLock<Vec<u8>> =
    LazyLock::new(|| get_test_td0("minimal_factory_backup.TD0"));
static KNOWN_FIRMWARE_BYTES: LazyLock<Vec<u8>> = LazyLock::new(|| get_test_td0("new_v2_00.TD0"));
static UNKNOWN_FIRMWARE_BYTES: LazyLock<Vec<u8>> =
    LazyLock::new(|| get_test_td0("valid_unknown_firmware.TD0"));

#[test]
fn test_create_td0_file_new() {
    let _td0file = TD0File::new();
}

#[test]
fn test_load_file_known_firmware() {
    let myfile = TD0File::try_from(KNOWN_FIRMWARE_BYTES.to_vec());
    assert_matches!(myfile, Ok(TD0File { .. }))
}

#[test]
fn test_load_file_unknown_firmware() {
    let myfile = TD0File::try_from(UNKNOWN_FIRMWARE_BYTES.to_vec());
    assert!(myfile.is_ok())
}

#[test]
fn test_file_parse_firmware() {
    let myfile = TD0File::try_from(KNOWN_FIRMWARE_BYTES.to_vec()).unwrap();
    assert_eq!(
        myfile.firmware_version(),
        "2.00",
        "parses regular firmware string."
    );

    let myfile = TD0File::try_from(FACTORY_BACKUP_FIRMWARE_BYTES.to_vec()).unwrap();
    assert_eq!(
        myfile.firmware_version(),
        "1.02",
        "parses factory backup firmware string."
    );
}

#[test]
fn test_parse_fails_on_invalid_td0_id_chunk() {
    // Wrong format header:
    const BAD_BYTES: [u8; 16] = [
        0x0e, 0, b'N', b'E', b'S', 0x1a, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let result = TD0File::try_from(BAD_BYTES.to_vec());
    assert_matches!(result, Err(TD0Error::FileParse(..)));
}

#[test]
fn test_parse_fails_on_invalid_manifest_size() {
    #[rustfmt::skip]
    const BAD_BYTES : [u8; 30] = [
        0x1c, 0, b'T', b'D', b'0', b'a', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        b'H', b'D', b'R', b'a', b'S', b'S', b'X', b'P', 32, 0, 0, 0, 0x50, 0,
    ];

    let mut buf: [u8; 256] = [0; 256];
    buf[..30].copy_from_slice(&BAD_BYTES);
    update_bytes_checksum(&mut buf);

    let result = TD0File::try_from(buf.to_vec());
    assert_eq!(
        result,
        Err(TD0Error::file_parse_error(ERR_MANIFEST_SIZE)),
        "should error on invalid manifest size."
    );
}

#[test]
fn test_parse_fails_on_invalid_manifest_tag() {
    #[rustfmt::skip]
    const TD0_BYTES : [u8; 16] = [
        0x1e, 0, b'T', b'D', b'0', b'a', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    #[rustfmt::skip]
    let tests: [([u8; 16], &str); 3] = [
        (
            [31, b'D', b'R', b'a', b'S', b'S', b'X', b'P', 32, 0, 0, 0, 0x50, 0, 0, 0,],
            "should error on tag chars not in ASCII range.",
        ),
        (
            [b'H', b'D', b'r', b'a', b'S', b'S', b'X', b'P', 32, 0, 0, 0, 0x50, 0, 0, 0,],
            "should error on tag chars[0..3] not uppercase.",
        ),
        (
            [b'H', b'D', b'R', b'A', b'S', b'S', b'X', b'P', 32, 0, 0, 0, 0x50, 0, 0, 0,],
            "should error on tag chars[3] not lowercase.",
        ),
    ];

    let mut buf: [u8; 256] = [0; 256];
    buf[..16].copy_from_slice(&TD0_BYTES);
    for (slice, message) in tests {
        buf[16..32].copy_from_slice(&slice);
        update_bytes_checksum(&mut buf);

        let result = TD0File::try_from(buf.as_ref());
        assert_eq!(
            result,
            Err(TD0Error::file_parse_error(ERR_MANIFEST_TAG_INVALID)),
            "{message}"
        );
    }
}

#[test]
fn test_parse_fails_on_duplicate_manifest_tag() {
    #[rustfmt::skip]
    const BAD_BYTES : [u8; 48] = [
        0x2e, 0, b'T', b'D', b'0', b'a', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        b'H', b'D', b'R', b'a', b'S', b'S', b'X', b'P', 0x30, 0, 0, 0, 0x50, 0, 0, 0,
        b'H', b'D', b'R', b'a', b'S', b'S', b'X', b'P', 0x80, 0, 0, 0, 0x50, 0, 0, 0,
    ];
    // New commnent
    let mut buf: [u8; 256] = [0; 256];
    buf[..48].copy_from_slice(&BAD_BYTES);
    update_bytes_checksum(&mut buf);

    let result = TD0File::try_from(buf.as_ref());
    assert_eq!(
        result,
        Err(TD0Error::file_parse_error(ERR_MANIFEST_TAG_DUPLICATE)),
        "should error on duplicate manifest tag."
    );
}

#[test]
fn test_parse_fails_on_bad_backup_tag() {
    #[rustfmt::skip]
    const MANIFEST_BYTES : [u8; 32] = [
        0x1e, 0,    b'T', b'D', b'0', b'a', 0,    0,    0,    0, 0, 0, 0,    0, 0, 0,
        b'H', b'D', b'R', b'a', b'S', b'S', b'X', b'P', 0x20, 0, 0, 0, 0x50, 0, 0, 0,
    ];

    // A good header chunk.
    #[rustfmt::skip]
    const HDR_A_CHUNK: [u8; 64] = [
        1,    0,    0,    0,    0x40, 0,    0,    0,    0x10, 0,    0,    0,    0,    0,    0,    0,
        b'S', b'S', b'X', b'P', b'R', b'O', b'K', b'T', 1,    1,    0,    0,    0,    0,    0,    0,
        b'U', b'S', b'E', b'R', b' ', b'K', b'I', b'T', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
        b'1', b'.', b'2', b'0', b'0', b'0', b'1', b'2', b'X', b'X', b'X', b'X', b'X', b'X', b'X', b' ',
    ];

    // Write manifest to buffer.
    let mut buf: [u8; 128] = [0; 128];
    buf[..32].copy_from_slice(&MANIFEST_BYTES);

    // Write valid header chunk to buffer.
    buf[32..96].copy_from_slice(&HDR_A_CHUNK);
    update_bytes_checksum(&mut buf);

    // This should succeed, fits the maximum extent of the buffer.
    let result = TD0File::try_from(buf.as_slice());
    assert!(result.is_ok(), "can parse a valid backup header.");

    // Change the first char of the backup header tag to a lowercase:
    buf[48] = b's';
    update_bytes_checksum(&mut buf);

    let result = TD0File::try_from(buf.as_slice());
    assert_matches!(
        result,
        Err(TD0Error::ValidationFailed(..)),
        "should error on invalid backup tag."
    );

    buf[48] = b'S';
    buf[64] = 127; // Set start of backup name to a non-printable char.
    update_bytes_checksum(&mut buf);

    let result = TD0File::try_from(buf.as_slice());
    assert_matches!(
        result,
        Err(TD0Error::ValidationFailed(..)),
        "should error on invalid backup name."
    );

    buf[64] = b'U';
    buf[80] = b'Y'; // Set the first char of the version # to a letter.
    update_bytes_checksum(&mut buf);

    let result = TD0File::try_from(buf.as_slice());
    assert_matches!(
        result,
        Err(TD0Error::ValidationFailed(..)),
        "should error on invalid version number."
    );

    buf[80] = b'1';
    buf[88] = b'x'; // Set the first char of the serial # to a lowercase letter.
    update_bytes_checksum(&mut buf);

    let result = TD0File::try_from(buf.as_slice());
    assert_matches!(
        result,
        Err(TD0Error::ValidationFailed(..)),
        "should error on invalid serial number."
    );
}

fn get_test_data(rel_path: PathBuf) -> Vec<u8> {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("test/data/");
    test_path.push(rel_path);
    fs::read(test_path).unwrap()
}

fn calc_and_append_checksum(buf: &mut Vec<u8>) {
    // In place calc a checksum and add it to the buffer.
    let mut hasher: md5::Md5 = md5::Md5::new();
    md5::Digest::update(&mut hasher, &buf);
    buf.extend_from_slice(hasher.finalize().as_bytes());
}

fn update_bytes_checksum(buf: &mut [u8]) {
    const SZ_MD5_DIGEST: usize = 16;

    let mut hasher: md5::Md5 = md5::Md5::new();
    hasher.update(&buf[..buf.len() - 16]);
    let hash = hasher.finalize();
    let buflen = buf.len();
    buf[buflen - SZ_MD5_DIGEST..].copy_from_slice(&hash);
}

fn fmt_hex(data: &[u8]) -> String {
    static HEXOUT_SETTINGS: LazyLock<HexOutSettings> = LazyLock::new(|| HexOutSettings {
        address_width: 4,
        ..Default::default()
    });

    hex_out(data, &HEXOUT_SETTINGS, 0, 0, 0).unwrap()
}

#[test]
fn test_ssxp_td0file_new() {
    let td0file = TD0File::new();

    // NOTE: The following (commented) line loads the bytes that the
    //       annotated code below generates. This test will generate
    //       the values manually from first principles to illustrate
    //       exactly what the bytes in the file represent.
    //
    // let expected = get_test_data(PathBuf::from("new_v1_10.TD0"));

    let mut expected: Vec<u8> = Vec::new();

    // TD0 ID chunk:
    // Byte 0 - 1: Number of bytes remaining in file header. u16 - little endian
    expected.extend_from_slice(U16::<LittleEndian>::new(30u16).as_bytes());
    // Byte 2 - 5: Constant 0x54 0x44 0x30 0x61 (ASCII "TD0a").
    // Byte 6 - 16: Unknown data or padding. Observed to be all 0x00 bytes.
    expected.extend_from_slice(b"TD0a\0\0\0\0\0\0\0\0\0\0".as_slice());

    // NOTE: TD0 manifest presumably can appear an any order, though
    //      all files observed by the author have the HDRa Manifest tag
    //      starting at byte 17. The offsets shown below are relative to
    //      the starting offset of the Manifest tag itself, instead of
    //      absolute to the file.

    // TD0 Manifest Tag for SSXP Backup header chunk ("HDRa" chunk.)
    //
    // (Relative to start of the Manifest tag)
    // Byte 0 - 3  : Constant 0x48 0x44 0x52 0x61 (ASCII "HDRa")
    // Byte 4 - 7  : Model identifier. 0x53, 0x53, 0x58, 0x50 (ASCI "SSXP")
    //               for SPD-SX Pro devices.
    expected.extend_from_slice(b"HDRaSSXP".as_slice());
    // Byte 8 - 11 : Chunk position (absolute) in file. u32 - little endian.
    expected.extend_from_slice(U32::<LittleEndian>::new(32u32).as_bytes());
    // Byte 12 - 15: Chunk size including Chunk Header. u32 - little endian.
    expected.extend_from_slice(U32::<LittleEndian>::new(80u32).as_bytes());

    // (End of header)

    // HDRa chunk, consisting of
    // - 16-byte chunk header
    // - 48 bytes of backup manifest data
    // - 16 bytes of literal 0xFF, perhaps an end-of-chunk or
    //   end-of-backup-header marker.
    //
    //  Chunk Header
    // Byte 0 - 3  : Item count. u32 - little endian.
    expected.extend_from_slice(U32::<LittleEndian>::new(1u32).as_bytes());
    // Byte 4 - 7  : Item size in bytes. u32 - little endian.
    //               The 16-byte "0xFF" marker section is included in
    //               the "item" byte count here.
    expected.extend_from_slice(U32::<LittleEndian>::new(64u32).as_bytes());
    // Byte 8 - 11 : First item offset relative to byte 0 of this header.
    //               u32 - little endian.
    expected.extend_from_slice(U32::<LittleEndian>::new(16u32).as_bytes());
    // Byte 12 - 16: Unknown data/padding. Observed to be all 0x00 bytes.
    expected.extend_from_slice(U32::<LittleEndian>::new(0u32).as_bytes());
    // Chunk Data
    // Byte 17 - 23: 8-byte backup type identifier. Values seen:
    //               - "SSXPROKT": single-kit backup
    //               - "SSXPROBK": whole-system backup
    expected.extend_from_slice(b"SSXPROKT".as_slice());
    // Byte 24 - 31: Unknown data.
    // Observed to be 0x01 0x01 0x00 0x00 0x00 0x00 0x00 0x00
    expected.append(&mut vec![0x01u8; 2]);
    expected.extend_from_slice(b"\0\0\0\0\0\0".as_slice());
    // Byte 32 - 47: Backup display name. ASCII string space-padded.
    expected.extend_from_slice(b"USER KIT        ".as_slice());
    // Byte 48 - 51: Firmware revision. ASCII string.
    expected.extend_from_slice(b"2.00".as_slice());
    // Byte 52 - 55: Firmware build number. ASCII string.
    expected.extend_from_slice(b"0000".as_slice());
    // Byte 56 - 63: Device serial number/identifier.
    //               ASCII string, space-padded.
    //               Observed values are alphanumeric, 7 characters long.
    expected.extend_from_slice(b"XXXXXXX ".as_slice());
    // Byte 64 - 80: End of backup header / chunk marker. All 0xFF.
    expected.append(&mut vec![0xFFu8; 16]);

    // File checksum:
    calc_and_append_checksum(&mut expected);

    let file_bytes: Vec<u8> = td0file.into();

    assert_eq!(fmt_hex(&file_bytes), fmt_hex(expected.as_bytes()));
}

#[test]
fn test_ssxp_td0file_add_chunk() {
    const NUM_ITEMS: usize = 3;

    let mut expected: Vec<u8> = Vec::new();

    // TD0 ID chunk:
    expected.extend_from_slice(U16::<LittleEndian>::new(46u16).as_bytes());
    expected.extend_from_slice(b"TD0a\0\0\0\0\0\0\0\0\0\0".as_slice());

    // Backup Header manifest
    expected.extend_from_slice(b"HDRaSSXP".as_slice());
    expected.extend_from_slice(U32::<LittleEndian>::new(48u32).as_bytes());
    expected.extend_from_slice(U32::<LittleEndian>::new(80u32).as_bytes());

    // TGLa
    let default_item = TGLaItem::default();
    let default_item_bytes = TD0ChunkItem::as_bytes(&default_item);
    expected.extend_from_slice(b"TGLaSSXP".as_slice());
    expected.extend_from_slice(U32::<LittleEndian>::new(128u32).as_bytes());
    expected.extend_from_slice(
        U32::<LittleEndian>::new(
            u32::try_from(size_of::<ChunkHeader>() + (default_item_bytes.len() * NUM_ITEMS))
                .unwrap(),
        )
        .as_bytes(),
    );

    // (End of header)

    // HDRa chunk
    expected.extend_from_slice(U32::<LittleEndian>::new(1u32).as_bytes());
    expected.extend_from_slice(U32::<LittleEndian>::new(64u32).as_bytes());
    expected.extend_from_slice(U32::<LittleEndian>::new(16u32).as_bytes());
    expected.extend_from_slice(U32::<LittleEndian>::new(0u32).as_bytes());
    expected.extend_from_slice(b"SSXPROKT".as_slice());
    expected.append(&mut vec![0x01u8; 2]);
    expected.extend_from_slice(b"\0\0\0\0\0\0".as_slice());
    expected.extend_from_slice(b"USER KIT        ".as_slice());
    expected.extend_from_slice(b"2.00".as_slice());
    expected.extend_from_slice(b"0000".as_slice());
    expected.extend_from_slice(b"XXXXXXX ".as_slice());
    expected.append(&mut vec![0xFFu8; 16]);

    // TGLa chunk:
    //  TGLa header
    expected
        .extend_from_slice(U32::<LittleEndian>::new(u32::try_from(NUM_ITEMS).unwrap()).as_bytes()); // num items
    expected.extend_from_slice(
        U32::<LittleEndian>::new(u32::try_from(default_item_bytes.len()).unwrap()).as_bytes(),
    ); // Item size
    expected.extend_from_slice(U32::<LittleEndian>::new(16u32).as_bytes()); // Header size / first item offset
    expected.extend_from_slice(U32::<LittleEndian>::new(0u32).as_bytes());
    //  TGLa data
    for _ in 0..3 {
        expected.extend_from_slice(default_item_bytes);
    }

    // File checksum:
    calc_and_append_checksum(&mut expected);

    //
    let mut td0file = TD0File::new();
    td0file
        .add_chunk_arbitrary(
            ChunkTagStr::from_str_unchecked("TGLa"),
            NUM_ITEMS,
            &default_item,
        ) // "TGLa", NUM_ITEMS)
        .expect("can successfully add a chunk.");

    let file_bytes: Vec<u8> = td0file.into();
    assert_eq!(fmt_hex(&file_bytes), fmt_hex(expected.as_bytes()));
}

#[test]
fn test_add_chunk_and_retrieve_items() {
    let mut td0file = TD0File::new();
    let default_tgl_item = TGLaItem::default();
    let chunk_name: &ChunkTagStr = ChunkTagStr::from_str_unchecked("TGLa");
    td0file
        .add_chunk_arbitrary(chunk_name, 3, &default_tgl_item)
        .expect("can successfully add a chunk.");
    let new_chunk: &TD0Chunk = td0file
        .chunk(chunk_name)
        .expect("can retrieve newly added chunk.");
    assert_matches!(
        new_chunk.item_at_ref(0),
        Ok(_),
        "can retrieve a chunk item after add_chunk."
    );
    assert_matches!(
        new_chunk.item_at_ref(2),
        Ok(_),
        "can retrieve the last chunk item after add_chunk."
    );
    assert_matches!(
        new_chunk.item_at_ref(3),
        Err(TD0Error::OutOfRange(..)),
        "can't retrieve spurious chunk items."
    );
}

#[test]
fn _01_test_replace_chunk_item() {
    let expected = get_test_data(PathBuf::from("_01_after_add_chunk.TD0"));
    let mut td0file = TD0File::new();
    let default_tgl_item = TGLaItem::default();
    let chunk_name: &ChunkTagStr = ChunkTagStr::from_str_unchecked("TGLa");
    td0file
        .add_chunk_arbitrary(chunk_name, 3, &default_tgl_item)
        .unwrap();

    let file_bytes: Vec<u8> = td0file.clone().into();
    assert_eq!(
        fmt_hex(&file_bytes),
        fmt_hex(expected.as_bytes()),
        "test setup successful."
    );

    let chunk = td0file.chunk_mut(chunk_name).unwrap();
    let mut new_tag = default_tgl_item;
    let _ = new_tag.set_field_value("name", TD0Value::new_text_value("This is a test !"));
    assert_eq!(
        new_tag.field_value("name"),
        Some(TD0Value::Text("This is a test !".to_string()))
    );
    assert_eq!(chunk.item_replace(1, &new_tag), Ok(()));

    let expected2 = get_test_data(PathBuf::from("_01_after_replace_item.TD0"));
    let actual: Vec<u8> = td0file.into();
    assert_eq!(
        fmt_hex(&actual),
        fmt_hex(&expected2),
        "replace item successful."
    );
}

#[test]
fn test_remove_chunk() {
    let initial_bytes = get_test_data(PathBuf::from("_01_after_add_chunk.TD0"));
    let expected = get_test_data(PathBuf::from("new_v2_00.TD0"));

    let mut td0file: TD0File = TD0File::try_from(initial_bytes).expect("or test is broken");

    assert_eq!(
        td0file.remove_chunk(&ChunkTagString::from_string_unchecked("TGLa")),
        Ok(()),
        "DefaultTD0File.remove_chunk() succeeds."
    );

    let actual: Vec<u8> = td0file.into();
    assert_eq!(
        fmt_hex(&actual),
        fmt_hex(expected.as_bytes()),
        "remove chunk produced correct output."
    );
}

#[test]
fn test_manifest() {
    let td0file = TD0File::try_from(get_test_data(PathBuf::from("new_v2_00.TD0")))
        .expect("or test is broken");
    let manifest: TD0Manifest = td0file.manifest().unwrap();
    assert_eq!(manifest.backup_name().trim_end(), "USER KIT");
    assert_eq!(manifest.backup_type(), TD0BackupType::Kit);
    assert_eq!(manifest.checksum_actual(), manifest.checksum_calculated());
    assert_eq!(
        manifest.checksum_actual().as_str(),
        "c034248a46632431b5a2f5c5b075f056"
    );
    assert_eq!(manifest.device_model(), TD0DeviceModel::SPDSXPro);
    assert_eq!(manifest.device_firmware_version().as_str(), "2.00");
    assert_eq!(manifest.device_firmware_build().as_str(), "0000");
    assert_eq!(manifest.device_serial().as_str(), "XXXXXXX");
    assert_eq!(manifest.size_actual(), 128);
    assert_eq!(manifest.size_calculated(), 128);
}
