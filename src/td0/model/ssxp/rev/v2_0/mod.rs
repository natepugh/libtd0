pub(crate) mod chunk;

use super::v1_10::chunk::{CURaItem, KITaItem, STLaItem, STPaItem, TGLaItem, TRGaItem, WVPaItem};
use crate::td0::model::ssxp::common::HDRaItem;
use chunk::{KITbItem, PVRaItem, STPbItem};
use libtd0_core::TD0ChunkItem;
use libtd0_core::result::{TD0Error, TD0Result};

pub(crate) fn chunk_item_from_bytes(
    chunk_name: &str,
    bytes: &[u8],
) -> TD0Result<Box<dyn TD0ChunkItem>> {
    match chunk_name {
        "HDRa" => Ok(Box::new(HDRaItem::try_from(bytes)?)),
        "KITa" => Ok(Box::new(KITaItem::try_from(bytes)?)),
        "KITb" => Ok(Box::new(KITbItem::try_from(bytes)?)),
        "CURa" => Ok(Box::new(CURaItem::try_from(bytes)?)),
        "PVRa" => Ok(Box::new(PVRaItem::try_from(bytes)?)),
        "STLa" => Ok(Box::new(STLaItem::try_from(bytes)?)),
        "STPa" => Ok(Box::new(STPaItem::try_from(bytes)?)),
        "STPb" => Ok(Box::new(STPbItem::try_from(bytes)?)),
        "TGLa" => Ok(Box::new(TGLaItem::try_from(bytes)?)),
        "TRGa" => Ok(Box::new(TRGaItem::try_from(bytes)?)),
        "WVPa" => Ok(Box::new(WVPaItem::try_from(bytes)?)),
        _ => Err(TD0Error::UnknownChunk),
    }
}

pub(crate) fn get_default_chunk_item(chunk_name: &str) -> Option<Box<dyn TD0ChunkItem>> {
    match chunk_name {
        "HDRa" => Some(Box::new(HDRaItem::default())),
        "KITa" => Some(Box::new(KITaItem::default())),
        "KITb" => Some(Box::new(KITbItem::default())),
        "CURa" => Some(Box::new(CURaItem::default())),
        "PVRa" => Some(Box::new(PVRaItem::default())),
        "STLa" => Some(Box::new(STLaItem::default())),
        "STPa" => Some(Box::new(STPaItem::default())),
        "STPb" => Some(Box::new(STPbItem::default())),
        "TGLa" => Some(Box::new(TGLaItem::default())),
        "TRGa" => Some(Box::new(TRGaItem::default())),
        "WVPa" => Some(Box::new(WVPaItem::default())),
        _ => None,
    }
}
