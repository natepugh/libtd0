// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod common;
pub mod rev;
pub mod strings;

#[cfg(test)]
mod tests {
    use std::assert_matches;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::LazyLock;

    use hexout::{HexOutSettings, hex_out};
    use md5::Digest as _;
    use pretty_assertions::assert_eq;
    use zerocopy::{IntoBytes, LittleEndian, U16, U32};

    use super::common::SSXPTD0File;
    use super::rev::v1_10::get_default_chunk_item;
    use crate::common::ChunkHeader;
    use td0_core::{TD0File, TD0Value, result::TD0Error};

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

    fn fmt_hex(data: &[u8]) -> String {
        static HEXOUT_SETTINGS: LazyLock<HexOutSettings> = LazyLock::new(|| HexOutSettings {
            address_width: 4,
            ..Default::default()
        });

        hex_out(data, &HEXOUT_SETTINGS, 0, 0, 0).unwrap()
    }

    #[test]
    fn test_ssxp_td0file_new() {
        let td0file = SSXPTD0File::new();

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
        expected.extend_from_slice(b"1.10".as_slice());
        // Byte 52 - 55: Firmware build number. ASCII string.
        expected.extend_from_slice(b"0083".as_slice());
        // Byte 56 - 63: Device serial number/identifier.
        //               ASCII string, space-padded.
        //               Observed values are alphanumeric, 7 characters long.
        expected.extend_from_slice(b"XXXXXXX ".as_slice());
        // Byte 64 - 80: End of backup header / chunk marker. All 0xFF.
        expected.append(&mut vec![0xFFu8; 16]);

        // File checksum:
        calc_and_append_checksum(&mut expected);

        assert_eq!(
            fmt_hex(td0file.try_into_bytes().unwrap().as_bytes()),
            fmt_hex(expected.as_bytes())
        );
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
        let default_item = get_default_chunk_item("TGLa").unwrap();
        let default_item_bytes = default_item.as_bytes();
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
        expected.extend_from_slice(b"1.10".as_slice());
        expected.extend_from_slice(b"0083".as_slice());
        expected.extend_from_slice(b"XXXXXXX ".as_slice());
        expected.append(&mut vec![0xFFu8; 16]);

        // TGLa chunk:
        //  TGLa header
        expected.extend_from_slice(
            U32::<LittleEndian>::new(u32::try_from(NUM_ITEMS).unwrap()).as_bytes(),
        ); // num items
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
        let mut td0file = SSXPTD0File::new();
        td0file
            .add_chunk("TGLa", NUM_ITEMS)
            .expect("can successfully add a chunk.");
        td0file.finalize().expect("finalize works");
        assert_eq!(
            fmt_hex(td0file.try_into_bytes().unwrap().as_bytes()),
            fmt_hex(expected.as_bytes())
        );
    }

    #[test]
    fn test_add_chunk_and_retrieve_items() {
        let mut td0file = SSXPTD0File::new();
        td0file
            .add_chunk("TGLa", 3)
            .expect("can successfully add a chunk.");
        assert_matches!(
            td0file.chunk_item_ref("TGLa", 0),
            Ok(_),
            "can retrieve a chunk item after add_chunk."
        );
        assert_matches!(
            td0file.chunk_item_ref("TGLa", 2),
            Ok(_),
            "can retrieve the last chunk item after add_chunk."
        );
        assert_matches!(
            td0file.chunk_item_ref("TGLa", 3),
            Err(TD0Error::ChunkItemParse),
            "can't retrieve spurious chunk items."
        );
    }

    #[test]
    fn _01_test_replace_chunk_item() {
        let expected = get_test_data(PathBuf::from("_01_after_add_chunk.TD0"));
        let mut td0file = SSXPTD0File::new();
        td0file.add_chunk("TGLa", 3).unwrap();
        let _ = td0file.finalize();

        assert_eq!(
            fmt_hex(td0file.to_bytes().as_bytes()),
            fmt_hex(expected.as_bytes()),
            "test setup successful."
        );

        let mut new_tag = td0file.chunk_item_default("TGLa").unwrap();
        let _ = new_tag.set_field_value("name", &TD0Value::Text("This is a test !".to_string()));
        assert_eq!(
            new_tag.field_value("name"),
            Some(TD0Value::Text("This is a test !".to_string()))
        );
        assert_eq!(td0file.chunk_item_replace("TGLa", 1, &*new_tag), Ok(()));
        let _ = td0file.finalize();

        let expected2 = get_test_data(PathBuf::from("_01_after_replace_item.TD0"));
        assert_eq!(
            fmt_hex(td0file.to_bytes().as_bytes()),
            fmt_hex(expected2.as_bytes()),
            "replace item successful."
        );
    }

    #[test]
    fn test_remove_chunk() {
        let initial_bytes = get_test_data(PathBuf::from("_01_after_add_chunk.TD0"));
        let expected = get_test_data(PathBuf::from("new_v1_10.TD0"));

        let mut td0file: SSXPTD0File =
            SSXPTD0File::try_from_bytes(&initial_bytes).expect("or test is broken");

        assert_eq!(
            td0file.remove_chunk("TGLa"),
            Ok(()),
            "SSXPTD0File.remove_chunk() succeeds."
        );
        let _ = td0file.finalize();

        assert_eq!(
            fmt_hex(td0file.to_bytes().as_bytes()),
            fmt_hex(expected.as_bytes()),
            "remove chunk produced correct output."
        );
    }

    #[test]
    fn test_ssxp_td0file_new_is_valid() {
        let td0file = SSXPTD0File::new();
        assert_eq!(Ok(()), td0file.validate_load());
    }
}
