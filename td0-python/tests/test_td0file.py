# SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
#
# SPDX-License-Identifier: GPL-3.0-or-later

"""Tests for libtd0.TD0File"""
from pathlib import Path

from libtd0 import ChunkManifest, TD0Chunk, TD0ChunkItem, TD0File
import pytest

DATA_PATH = Path(__file__).parent / 'data'
_MINIMAL_V1_10_BYTES : bytes|None = None
MINIMAL_V1_10_CHECKUSM : str = 'd4690edab87afa219f9c02c26bd64d4b'

_MINIMAL_W_TWO_CHUNKS_BYTES : bytes|None = None
_MINIMAL_V2_00_W_WVP_BYTES : bytes|None = None

@pytest.fixture
def td0_bytes_minimal() -> bytes:
    global _MINIMAL_V1_10_BYTES

    if _MINIMAL_V1_10_BYTES is None:
        _MINIMAL_V1_10_BYTES = (DATA_PATH / 'minimal_v1_10.TD0').read_bytes()

    return _MINIMAL_V1_10_BYTES


@pytest.fixture
def td0_bytes_minimal_w_two_chunks() -> bytes:
    global _MINIMAL_W_TWO_CHUNKS_BYTES

    if _MINIMAL_W_TWO_CHUNKS_BYTES is None:
        _MINIMAL_W_TWO_CHUNKS_BYTES = (DATA_PATH / 'minimal_w_two_chunks.TD0').read_bytes()

    return _MINIMAL_W_TWO_CHUNKS_BYTES


@pytest.fixture
def td0_bytes_minimal_v2_00_w_wvp() -> bytes:
    global _MINIMAL_V2_00_W_WVP_BYTES

    if _MINIMAL_V2_00_W_WVP_BYTES is None:
        _MINIMAL_V2_00_W_WVP_BYTES = (DATA_PATH / 'minimal_v2_00_w_wvp.TD0').read_bytes()

    return _MINIMAL_V2_00_W_WVP_BYTES


def test_load_file(td0_bytes_minimal: bytes) -> None:
    """Does it parse a file from bytes?"""
    td0file : TD0File = TD0File(td0_bytes_minimal)
    assert td0file.read_checksum() == MINIMAL_V1_10_CHECKUSM


def test_new_file() -> None:
    """Does it successfully create a new file?"""
    td0file : TD0File = TD0File()
    assert td0file.list_chunks() == ['HDRa']


def test_add_chunk(td0_bytes_minimal: bytes) -> None:
    """Can a chunk be added?"""
    td0file : TD0File = TD0File(td0_bytes_minimal)
    assert td0file.list_chunks() == ['HDRa']
    result = td0file.add_chunk('TGLa', 5)
    assert td0file.list_chunks() == ['HDRa', 'TGLa']
    assert isinstance(result, TD0Chunk)


def test_add_chunk_raises_on_duplicate(td0_bytes_minimal: bytes) -> None:
    """Does it raise an error when adding a duplicate chunk?"""
    td0file : TD0File = TD0File(td0_bytes_minimal)
    assert td0file.list_chunks() == ['HDRa']

    with pytest.raises(ValueError):
        td0file.add_chunk('HDRa', 1)


def test_add_chunk_raises_on_invalid_chunk_name(td0_bytes_minimal: bytes) -> None:
    """Does it raise an error when adding a duplicate chunk?"""
    td0file : TD0File = TD0File(td0_bytes_minimal)
    assert td0file.list_chunks() == ['HDRa']

    for invalid in ('FÖOa', 'hDRa', 'HDRA', '0123', '..$@' 'HDRFOOa'):
        with pytest.raises(ValueError):
            td0file.add_chunk(invalid, 1)


def test_calc_checksum(td0_bytes_minimal: bytes) -> None:
    """Does calc_checksum() return the expected checksum?"""
    td0file : TD0File = TD0File(td0_bytes_minimal)
    assert td0file.calc_checksum() == MINIMAL_V1_10_CHECKUSM


def test_retrieve_chunk(td0_bytes_minimal_w_two_chunks: bytes) -> None:
    """Can it retrieve a chunk?"""
    # This example file has two chunks: "HDRa" and "TGLa".
    td0file : TD0File = TD0File(td0_bytes_minimal_w_two_chunks)
    assert isinstance(td0file.chunk("TGLa"), TD0Chunk)


def test_retrieve_chunk_returns_none_on_invalid_name(td0_bytes_minimal_w_two_chunks: bytes) -> None:
    """Does it raise a KeyError on an invalid name?"""
    # This example file has two chunks: "HDRa" and "TGLa".
    td0file : TD0File = TD0File(td0_bytes_minimal_w_two_chunks)

    assert td0file.chunk('FOOa') is None
    assert td0file.chunk('FÖOa') is None


def test_chunk_info(td0_bytes_minimal_w_two_chunks: bytes) -> None:
    # This example file has two chunks: 'HDRa' and 'TGLa'.
    td0file : TD0File = TD0File(td0_bytes_minimal_w_two_chunks)
    chunk_info : ChunkManifest|None = td0file.chunk_info('TGLa')
    assert isinstance(chunk_info, ChunkManifest)
    assert chunk_info.name == 'TGLa'
    assert chunk_info.item_size == 16
    assert chunk_info.item_count == 3
    assert chunk_info.pos == 128


@pytest.mark.parametrize(
    ('file_bytes_fixture_name', 'expected_firmware'),
    (
        ('td0_bytes_minimal', '1.10'),
        ('td0_bytes_minimal_v2_00_w_wvp', '2.00'),
    ),
)
def test_firmware_version(file_bytes_fixture_name: str, expected_firmware: str, request) -> None:
    """Does it retrieve the proper firmware version from the file?"""
    file_bytes = request.getfixturevalue(file_bytes_fixture_name)
    td0file : TD0File = TD0File(file_bytes)
    assert td0file.firmware_version() == expected_firmware


def test_has_chunk(td0_bytes_minimal_v2_00_w_wvp: bytes) -> None:
    """Does has_chunk produce the expected results?"""
    td0file : TD0File = TD0File(td0_bytes_minimal_v2_00_w_wvp)
    assert td0file.has_chunk('HDRa')
    assert td0file.has_chunk('WVPa')
    assert not td0file.has_chunk('FOOa')


def test_list_chunks(td0_bytes_minimal_v2_00_w_wvp: bytes) -> None:
    """Does has_chunk produce the expected results?"""
    # This file has two chunks: ['HDRa', and 'WVPa']
    td0file : TD0File = TD0File(td0_bytes_minimal_v2_00_w_wvp)
    assert td0file.list_chunks() == ['HDRa', 'WVPa']


def test_read_checksum(td0_bytes_minimal: bytes) -> None:
    """Does calc_checksum() return the expected checksum?"""
    td0file : TD0File = TD0File(td0_bytes_minimal)
    assert td0file.read_checksum() == MINIMAL_V1_10_CHECKUSM


def test_remove_chunk(td0_bytes_minimal_v2_00_w_wvp: bytes) -> None:
    """Does it remove a chunk from the file?"""
    # This file has two chunks: ['HDRa', and 'WVPa']
    td0file : TD0File = TD0File(td0_bytes_minimal_v2_00_w_wvp)
    assert td0file.list_chunks() == ['HDRa', 'WVPa']

    td0file.remove_chunk('WVPa')
    assert td0file.list_chunks() == ['HDRa']


def test_remove_chunk_raises_on_removing_hdra_chunk(td0_bytes_minimal_v2_00_w_wvp: bytes) -> None:
    """Does it raise instead of removing the HDRa chunk?"""
    # This file has two chunks: ['HDRa', and 'WVPa']
    td0file : TD0File = TD0File(td0_bytes_minimal_v2_00_w_wvp)
    assert td0file.list_chunks() == ['HDRa', 'WVPa']

    with pytest.raises(RuntimeError):
        td0file.remove_chunk('HDRa')

    assert td0file.list_chunks() == ['HDRa', 'WVPa']


def test_replace_chunk(td0_bytes_minimal_w_two_chunks: bytes) -> None:
    """Does it replace a td0file chunk with a new supplied chunk?"""
    td0file : TD0File = TD0File(td0_bytes_minimal_w_two_chunks)
    chunk : TD0Chunk|None = td0file.chunk('TGLa')
    assert isinstance(chunk, TD0Chunk)
    tgl_item : TD0ChunkItem = chunk.item(0)
    tgl_item.set_field_value('name', 'This is a test')
    chunk.replace_item(0, tgl_item)
    td0file.replace_chunk('TGLa', chunk)

    chunk_again : TD0Chunk|None = td0file.chunk('TGLa')
    assert isinstance(chunk_again, TD0Chunk)
    tgl_item_again : TD0ChunkItem = chunk_again.item(0)
    assert tgl_item_again.field_value('name') == 'This is a test'


def test_set_firmware_version(td0_bytes_minimal: bytes) -> None:
    """Does it replace the firmware version?"""
    td0file : TD0File = TD0File(td0_bytes_minimal)
    assert td0file.firmware_version() == "1.10"
    td0file.set_firmware_version("2.00")
    assert td0file.firmware_version() == "2.00"


def test_size(td0_bytes_minimal) -> None:
    """Does it properly report the file size?"""
    td0file : TD0File = TD0File(td0_bytes_minimal)
    assert td0file.size() == 128
