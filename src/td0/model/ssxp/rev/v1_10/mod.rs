pub(crate) mod chunk;

use crate::td0::model::ssxp::common::HDRaItem;
use chunk::{CURaItem, KITaItem, STLaItem, STPaItem, TGLaItem, TRGaItem, WVPaItem};
use libtd0_core::TD0ChunkItem;
use libtd0_core::result::{TD0Error, TD0Result};

pub(crate) fn chunk_item_from_bytes(
    chunk_name: &str,
    bytes: &[u8],
) -> TD0Result<Box<dyn TD0ChunkItem>> {
    match chunk_name {
        "HDRa" => Ok(Box::new(HDRaItem::try_from(bytes)?)),
        "KITa" => Ok(Box::new(KITaItem::try_from(bytes)?)),
        "CURa" => Ok(Box::new(CURaItem::try_from(bytes)?)),
        "STLa" => Ok(Box::new(STLaItem::try_from(bytes)?)),
        "STPa" => Ok(Box::new(STPaItem::try_from(bytes)?)),
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
        "CURa" => Some(Box::new(CURaItem::default())),
        "STLa" => Some(Box::new(STLaItem::default())),
        "STPa" => Some(Box::new(STPaItem::default())),
        "TGLa" => Some(Box::new(TGLaItem::default())),
        "TRGa" => Some(Box::new(TRGaItem::default())),
        "WVPa" => Some(Box::new(WVPaItem::default())),
        _ => None,
    }
}
