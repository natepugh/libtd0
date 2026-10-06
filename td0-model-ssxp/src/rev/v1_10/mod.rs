// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod chunk;

use crate::common::HDRaItem;
use chunk::{CURaItem, KITaItem, STLaItem, STPaItem, TGLaItem, TRGaItem, WVPaItem};
use td0_core::{TD0ChunkItem, TD0Error, TD0Result};
use zerocopy::{FromBytes, TryFromBytes};

static _CHUNK_NAMES: [&str; 8] = [
    "HDRa", "KITa", "CURa", "STLa", "STPa", "TGLa", "TRGa", "WVPa",
];

pub fn valid_chunk_names() -> &'static [&'static str] {
    &_CHUNK_NAMES
}

pub fn chunk_item_from_bytes<'b>(
    chunk_name: &str,
    bytes: &'b [u8],
) -> TD0Result<&'b dyn TD0ChunkItem> {
    match chunk_name {
        "HDRa" => Ok(HDRaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?),
        "KITa" => Ok(KITaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?),
        "CURa" => Ok(CURaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?),
        "STLa" => Ok(STLaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?),
        "STPa" => Ok(STPaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?),
        "TGLa" => Ok(TGLaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?),
        "TRGa" => Ok(TRGaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?),
        "WVPa" => Ok(WVPaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?),
        _ => Err(TD0Error::unknown_chunk_error(chunk_name)),
    }
}

pub fn chunk_item_from_bytes_mut<'b>(
    chunk_name: &str,
    bytes: &'b mut [u8],
) -> TD0Result<&'b mut dyn TD0ChunkItem> {
    match chunk_name {
        "HDRa" => Ok(HDRaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?),
        "KITa" => Ok(KITaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?),
        "CURa" => Ok(CURaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?),
        "STLa" => Ok(STLaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?),
        "STPa" => Ok(STPaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?),
        "TGLa" => Ok(TGLaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?),
        "TRGa" => Ok(TRGaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?),
        "WVPa" => Ok(WVPaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?),
        _ => Err(TD0Error::unknown_chunk_error(chunk_name)),
    }
}

pub fn chunk_item_from_bytes_owned(
    chunk_name: &str,
    bytes: &[u8],
) -> TD0Result<Box<dyn TD0ChunkItem>> {
    match chunk_name {
        "HDRa" => Ok(Box::new(
            HDRaItem::read_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?,
        )),
        "KITa" => Ok(Box::new(
            KITaItem::read_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?,
        )),
        "CURa" => Ok(Box::new(
            CURaItem::read_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?,
        )),
        "STLa" => Ok(Box::new(
            STLaItem::read_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?,
        )),
        "STPa" => Ok(Box::new(
            STPaItem::read_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?,
        )),
        "TGLa" => Ok(Box::new(
            TGLaItem::read_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?,
        )),
        "TRGa" => Ok(Box::new(
            TRGaItem::read_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?,
        )),
        "WVPa" => Ok(Box::new(
            WVPaItem::read_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?,
        )),
        _ => Err(TD0Error::unknown_chunk_error(chunk_name)),
    }
}

pub fn get_default_chunk_item(chunk_name: &str) -> Option<Box<dyn TD0ChunkItem>> {
    match chunk_name {
        "HDRa" => Some(Box::new(HDRaItem::default())),
        "KITa" => Some(Box::new(KITaItem::default())),
        "CURa" => Some(Box::new(CURaItem::default())),
        "STLa" => Some(Box::new(STLaItem::default())),
        "STPa" => Some(Box::new(STPaItem::default())),
        "TGLa" => Some(Box::new(TGLaItem::default())),
        "TRGa" => Some(Box::new(TRGaItem::default())),
        "WVPa" => Some(Box::new(WVPaItem::default())),
        _ => None,
    }
}
