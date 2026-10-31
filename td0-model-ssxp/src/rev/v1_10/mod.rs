// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod chunk;

use chunk::{CURaItem, KITaItem, STLaItem, STPaItem, TGLaItem, TRGaItem, WVPaItem};
use td0_core::{ChunkItemAccessorCallbacks, HDRaItem, TD0Error};
use zerocopy::TryFromBytes;

pub(crate) static CHUNK_NAMES: [&str; 8] = [
    "HDRa", "KITa", "CURa", "STLa", "STPa", "TGLa", "TRGa", "WVPa",
];

pub fn build_chunk_callbacks(chunk_name: &str) -> Option<ChunkItemAccessorCallbacks> {
    match chunk_name {
        "HDRa" => Some(ChunkItemAccessorCallbacks {
            chunk_item_ref_from_bytes_cb: |bytes: &[u8]| {
                Ok(HDRaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?)
            },
            chunk_item_mut_from_bytes_cb: |bytes: &mut [u8]| {
                Ok(HDRaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?)
            },
            chunk_item_owned_from_bytes_cb: |bytes: &[u8]| {
                Ok(Box::new(
                    HDRaItem::try_read_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?,
                ))
            },
            default_chunk_item_cb: || Box::new(HDRaItem::default()),
        }),
        "KITa" => Some(ChunkItemAccessorCallbacks {
            chunk_item_ref_from_bytes_cb: |bytes: &[u8]| {
                Ok(KITaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?)
            },
            chunk_item_mut_from_bytes_cb: |bytes: &mut [u8]| {
                Ok(KITaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?)
            },
            chunk_item_owned_from_bytes_cb: |bytes: &[u8]| {
                Ok(Box::new(
                    KITaItem::try_read_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?,
                ))
            },
            default_chunk_item_cb: || Box::new(KITaItem::default()),
        }),
        "CURa" => Some(ChunkItemAccessorCallbacks {
            chunk_item_ref_from_bytes_cb: |bytes: &[u8]| {
                Ok(CURaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?)
            },
            chunk_item_mut_from_bytes_cb: |bytes: &mut [u8]| {
                Ok(CURaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?)
            },
            chunk_item_owned_from_bytes_cb: |bytes: &[u8]| {
                Ok(Box::new(
                    CURaItem::try_read_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?,
                ))
            },
            default_chunk_item_cb: || Box::new(CURaItem::default()),
        }),
        "STLa" => Some(ChunkItemAccessorCallbacks {
            chunk_item_ref_from_bytes_cb: |bytes: &[u8]| {
                Ok(STLaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?)
            },
            chunk_item_mut_from_bytes_cb: |bytes: &mut [u8]| {
                Ok(STLaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?)
            },
            chunk_item_owned_from_bytes_cb: |bytes: &[u8]| {
                Ok(Box::new(
                    STLaItem::try_read_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?,
                ))
            },
            default_chunk_item_cb: || Box::new(STLaItem::default()),
        }),
        "STPa" => Some(ChunkItemAccessorCallbacks {
            chunk_item_ref_from_bytes_cb: |bytes: &[u8]| {
                Ok(STPaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?)
            },
            chunk_item_mut_from_bytes_cb: |bytes: &mut [u8]| {
                Ok(STPaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?)
            },
            chunk_item_owned_from_bytes_cb: |bytes: &[u8]| {
                Ok(Box::new(
                    STPaItem::try_read_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?,
                ))
            },
            default_chunk_item_cb: || Box::new(STPaItem::default()),
        }),
        "TGLa" => Some(ChunkItemAccessorCallbacks {
            chunk_item_ref_from_bytes_cb: |bytes: &[u8]| {
                Ok(TGLaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?)
            },
            chunk_item_mut_from_bytes_cb: |bytes: &mut [u8]| {
                Ok(TGLaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?)
            },
            chunk_item_owned_from_bytes_cb: |bytes: &[u8]| {
                Ok(Box::new(
                    TGLaItem::try_read_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?,
                ))
            },
            default_chunk_item_cb: || Box::new(TGLaItem::default()),
        }),
        "TRGa" => Some(ChunkItemAccessorCallbacks {
            chunk_item_ref_from_bytes_cb: |bytes: &[u8]| {
                Ok(TRGaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?)
            },
            chunk_item_mut_from_bytes_cb: |bytes: &mut [u8]| {
                Ok(TRGaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?)
            },
            chunk_item_owned_from_bytes_cb: |bytes: &[u8]| {
                Ok(Box::new(
                    TRGaItem::try_read_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?,
                ))
            },
            default_chunk_item_cb: || Box::new(TRGaItem::default()),
        }),
        "WVPa" => Some(ChunkItemAccessorCallbacks {
            chunk_item_ref_from_bytes_cb: |bytes: &[u8]| {
                Ok(WVPaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?)
            },
            chunk_item_mut_from_bytes_cb: |bytes: &mut [u8]| {
                Ok(WVPaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?)
            },
            chunk_item_owned_from_bytes_cb: |bytes: &[u8]| {
                Ok(Box::new(
                    WVPaItem::try_read_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?,
                ))
            },
            default_chunk_item_cb: || Box::new(WVPaItem::default()),
        }),
        _ => None,
    }
}
