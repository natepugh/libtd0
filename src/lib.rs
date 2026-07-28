pub mod td0;
use md5::{Md5,Digest};
use td0::result::TD0Result;
// use anyhow::{Ok, Result};

use crate::td0::{header::TD0ManifestTag, result::TD0Error};

const SZ_MD5_DIGEST: usize = 16;

#[derive(Debug)]
pub enum TD0BackupType {
    Kit,
    System,
    Unknown,
}

#[derive(Debug)]
pub enum TD0DeviceModel {
    SPDSXPro,
    Unknown,
}

pub struct ManifestData {
    backup_size: usize,
    backup_type: TD0BackupType,
    checksum_actual: [u8; 16],
    checksum_calculated: [u8; 16],
    device_model: TD0DeviceModel,
    size_actual: usize,
    size_calculated: usize,
    chunks: Vec<TD0ManifestTag>,
}

impl ManifestData {
    pub fn from_bytes(bytes: &[u8]) -> TD0Result<Self> {
        let backup_len = bytes.len() - SZ_MD5_DIGEST;
        // let td0header = td0::header::TD0Header::from_bytes(&bytes)?;
        let td0file = td0::TD0File::from_bytes(bytes.to_vec())?;

        let mut checksum_actual: [u8; SZ_MD5_DIGEST] = [0; SZ_MD5_DIGEST];
        checksum_actual.clone_from_slice(&bytes[backup_len..bytes.len()]);
        let mut hasher = Md5::new();
        hasher.update(&bytes[0..backup_len]);
        let checksum_calculated = hasher.finalize();
        //let Some(last_chunk_tag) = td0header.manifest.last() else {
        let Some(last_chunk_tag) = td0file.manifest.last() else {
            return Err(TD0Error::InvalidTD0TagError);
        };
        let mut chunks: Vec<TD0ManifestTag> = td0file.manifest.clone();
        chunks.copy_from_slice(&td0file.manifest[..]);

        Ok(ManifestData{
            backup_size: 0,
            backup_type: TD0BackupType::Unknown,
            checksum_actual: checksum_actual,
            checksum_calculated: Into::into(checksum_calculated),
            device_model: TD0DeviceModel::Unknown,
            size_actual: bytes.len(),
            size_calculated: last_chunk_tag.get_pos() as usize + last_chunk_tag.get_length() as usize + SZ_MD5_DIGEST,
            chunks: chunks,
        })
    }
}

impl std::fmt::Display for ManifestData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Backup Size: {}\n\
            Backup Type: {:?}\n\
            Checksum from File   : {}\n\
            Checksum (calculated): {}\n\
            Device Model: {:?}\n\
            Actual Size: {}\n\
            Calculated Size: {}\n\
            Chunks:
    {}",
            self.backup_size,
            self.backup_type,
            self.checksum_actual
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect::<String>(),
            self.checksum_calculated
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect::<String>(),
            self.device_model,
            self.size_actual,
            self.size_calculated,
            self.chunks
                .iter()
                .map(|tag| format!("{}", tag))
                .collect::<Vec<String>>()
                .join("\n    "),
        )
    }
}
