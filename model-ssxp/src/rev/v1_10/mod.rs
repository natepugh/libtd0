pub mod chunk;

use crate::common::HDRaItem;
use chunk::{CURaItem, KITaItem, STLaItem, STPaItem, TGLaItem, TRGaItem, WVPaItem};
use libtd0_core::TD0ChunkItem;
use libtd0_core::result::{TD0Error, TD0Result};
use zerocopy::{FromBytes, TryFromBytes};

static CHUNK_NAMES: [&str; 8] = [
    "HDRa", "KITa", "CURa", "STLa", "STPa", "TGLa", "TRGa", "WVPa",
];

pub fn chunk_item_from_bytes<'b>(
    chunk_name: &str,
    bytes: &'b [u8],
) -> TD0Result<&'b dyn TD0ChunkItem> {
    match chunk_name {
        "HDRa" => Ok(HDRaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?),
        "KITa" => Ok(KITaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?),
        "CURa" => Ok(CURaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?),
        "STLa" => Ok(STLaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?),
        "STPa" => Ok(STPaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?),
        "TGLa" => Ok(TGLaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?),
        "TRGa" => Ok(TRGaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?),
        "WVPa" => Ok(WVPaItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?),
        _ => Err(TD0Error::UnknownChunk),
    }
}

pub fn chunk_item_from_bytes_mut<'b>(
    chunk_name: &str,
    bytes: &'b mut [u8],
) -> TD0Result<&'b mut dyn TD0ChunkItem> {
    match chunk_name {
        "HDRa" => Ok(HDRaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?),
        "KITa" => Ok(KITaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?),
        "CURa" => Ok(CURaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?),
        "STLa" => Ok(STLaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?),
        "STPa" => Ok(STPaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?),
        "TGLa" => Ok(TGLaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?),
        "TRGa" => Ok(TRGaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?),
        "WVPa" => Ok(WVPaItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?),
        _ => Err(TD0Error::UnknownChunk),
    }
}

pub fn chunk_item_from_bytes_owned(
    chunk_name: &str,
    bytes: &[u8],
) -> TD0Result<Box<dyn TD0ChunkItem>> {
    match chunk_name {
        "HDRa" => Ok(Box::new(
            HDRaItem::read_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?,
        )),
        "KITa" => Ok(Box::new(
            KITaItem::read_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?,
        )),
        "CURa" => Ok(Box::new(
            CURaItem::read_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?,
        )),
        "STLa" => Ok(Box::new(
            STLaItem::read_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?,
        )),
        "STPa" => Ok(Box::new(
            STPaItem::read_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?,
        )),
        "TGLa" => Ok(Box::new(
            TGLaItem::read_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?,
        )),
        "TRGa" => Ok(Box::new(
            TRGaItem::read_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?,
        )),
        "WVPa" => Ok(Box::new(
            WVPaItem::read_from_bytes(bytes).map_err(|_| TD0Error::InvalidChunkItem)?,
        )),
        _ => Err(TD0Error::UnknownChunk),
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

pub fn get_chunk_items(
    chunk_name: &str,
    bytes: &mut [u8],
    item_count: usize,
) -> TD0Result<Vec<Box<dyn TD0ChunkItem>>> {
    match chunk_name {
        "HDRa" => {
            let elems = <[HDRaItem]>::mut_from_bytes_with_elems(bytes, item_count)
                .map_err(|_| TD0Error::InvalidChunkItem)?;
            Ok(elems
                .iter()
                .map(|elem| Box::new(*elem) as Box<dyn TD0ChunkItem>)
                .collect())
        }
        "KITa" => {
            let elems = <[KITaItem]>::mut_from_bytes_with_elems(bytes, item_count)
                .map_err(|_| TD0Error::InvalidChunkItem)?;
            Ok(elems
                .iter()
                .map(|elem| Box::new(*elem) as Box<dyn TD0ChunkItem>)
                .collect())
        }
        "CURa" => {
            let elems = <[CURaItem]>::mut_from_bytes_with_elems(bytes, item_count)
                .map_err(|_| TD0Error::InvalidChunkItem)?;
            Ok(elems
                .iter()
                .map(|elem| Box::new(*elem) as Box<dyn TD0ChunkItem>)
                .collect())
        }
        "STLa" => {
            let elems = <[STLaItem]>::mut_from_bytes_with_elems(bytes, item_count)
                .map_err(|_| TD0Error::InvalidChunkItem)?;
            Ok(elems
                .iter()
                .map(|elem| Box::new(*elem) as Box<dyn TD0ChunkItem>)
                .collect())
        }
        "STPa" => {
            let elems = <[STPaItem]>::mut_from_bytes_with_elems(bytes, item_count)
                .map_err(|_| TD0Error::InvalidChunkItem)?;
            Ok(elems
                .iter()
                .map(|elem| Box::new(*elem) as Box<dyn TD0ChunkItem>)
                .collect())
        }
        "TGLa" => {
            let elems = <[TGLaItem]>::mut_from_bytes_with_elems(bytes, item_count)
                .map_err(|_| TD0Error::InvalidChunkItem)?;
            Ok(elems
                .iter()
                .map(|elem| Box::new(*elem) as Box<dyn TD0ChunkItem>)
                .collect())
        }
        "TRGa" => {
            let elems = <[TRGaItem]>::mut_from_bytes_with_elems(bytes, item_count)
                .map_err(|_| TD0Error::InvalidChunkItem)?;
            Ok(elems
                .iter()
                .map(|elem| Box::new(*elem) as Box<dyn TD0ChunkItem>)
                .collect())
        }
        "WVPa" => {
            let elems = <[WVPaItem]>::mut_from_bytes_with_elems(bytes, item_count)
                .map_err(|_| TD0Error::InvalidChunkItem)?;
            Ok(elems
                .iter()
                .map(|elem| Box::new(*elem) as Box<dyn TD0ChunkItem>)
                .collect())
        }
        _ => Err(TD0Error::UnknownChunk),
    }
}
