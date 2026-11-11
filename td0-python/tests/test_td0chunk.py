# SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
#
# SPDX-License-Identifier: GPL-3.0-or-later

"""Tests for libtd0.TD0Chunk"""
from decimal import Decimal
from pathlib import Path

from libtd0 import  TD0ValueType, FieldDef, TD0Chunk, TD0File

import pytest


DATA_PATH = Path(__file__).parent / 'data'

_EXAMPLE_TGL_ITEMS_BYTES : bytes|None = None
_MINIMAL_V2_00_W_WVP_BYTES : bytes|None = None
_MINIMAL_W_KIT_BYTES : bytes|None = None


@pytest.fixture
def td0_bytes_exampl_tgl_items() -> bytes:
    global _EXAMPLE_TGL_ITEMS_BYTES

    if _EXAMPLE_TGL_ITEMS_BYTES is None:
        _EXAMPLE_TGL_ITEMS_BYTES = (DATA_PATH / 'example_tgl_items.TD0').read_bytes()

    return _EXAMPLE_TGL_ITEMS_BYTES


@pytest.fixture
def td0_bytes_minimal_w_kit() -> bytes:
    global _MINIMAL_W_KIT_BYTES

    if _MINIMAL_W_KIT_BYTES is None:
        _MINIMAL_W_KIT_BYTES = (DATA_PATH / 'minimal_w_kit.TD0').read_bytes()

    return _MINIMAL_W_KIT_BYTES


@pytest.fixture
def td0_bytes_minimal_v2_00_w_wvp() -> bytes:
    global _MINIMAL_V2_00_W_WVP_BYTES

    if _MINIMAL_V2_00_W_WVP_BYTES is None:
        _MINIMAL_V2_00_W_WVP_BYTES = (DATA_PATH / 'minimal_v2_00_w_wvp.TD0').read_bytes()

    return _MINIMAL_V2_00_W_WVP_BYTES


def test_describe_fields(td0_bytes_minimal_w_kit: bytes) -> None:
    """Does describe_fields return FieldDefs?"""
    td0file : TD0File = TD0File(td0_bytes_minimal_w_kit)
    kit_chunk : TD0Chunk|None = td0file.chunk('KITa')
    assert isinstance(kit_chunk, TD0Chunk)

    desc : dict[str, FieldDef] = {desc.name: desc for desc in kit_chunk.describe_fields()}
    expected : dict[str, tuple[TD0ValueType, tuple[str, dict]]] = {
        'name': (TD0ValueType.String, ('string_constraints', {'max_len': 16})),
        'memo': (TD0ValueType.String, ('string_constraints', {'max_len': 64})),
        'volume': (TD0ValueType.Decimal, ('decimal_constraints', {'min': Decimal('-60.0'), 'max': Decimal('6.0')})),
        'tempo': (TD0ValueType.Decimal, ('decimal_constraints', {'min': Decimal('20.0'), 'max': Decimal('260.0')})),
        'click_setting': (TD0ValueType.String, ('string_constraints', {'options': ['SYSTEM', 'KIT']})),
        'unknown_2': (TD0ValueType.Bytes, ('bytes_constraints', {'len': 3})),
        'click_sample_slot': (TD0ValueType.Int, ('int_constraints', {'min': 0, 'max': 20_000})),
        'click_pan': (TD0ValueType.Int, ('int_constraints', {'min': -15, 'max': 15})),
        'click_beats': (TD0ValueType.Int, ('int_constraints', {'min': 1, 'max': 9})),
    }

    for field, desc_values in expected.items():
        field_type, constraint = desc_values
        constraint_field, constraint_data = constraint

        assert field in desc, f'`{field}` in fields.'
        actual = desc[field]
        assert actual.name == field, f'`{field}` name is `{field}`'
        assert actual.field_type == field_type, f'`{field}` type is `{field_type}`'
        actual_constraint = getattr(actual, constraint_field, None)
        assert actual_constraint is not None, f'`{field}` has constraint `{constraint_field}` set.'
        for key, val in constraint_data.items():
            assert getattr(actual_constraint, key, None) == val, f'`{field}` constraint `{constraint_field}` `{key}` == `{val}`'


def test_size(td0_bytes_minimal_w_kit: bytes) -> None:
    """Does size() return the chunk size in bytes?"""
    td0file : TD0File = TD0File(td0_bytes_minimal_w_kit)
    kit_chunk : TD0Chunk|None = td0file.chunk('KITa')
    assert isinstance(kit_chunk, TD0Chunk)

    assert kit_chunk.size() == 3472


def test_item(td0_bytes_exampl_tgl_items: bytes) -> None:
    """Does it retrieve an item at a specific index?"""
    td0file : TD0File = TD0File(td0_bytes_exampl_tgl_items)
    tgl_chunk : TD0Chunk|None = td0file.chunk('TGLa')
    assert isinstance(tgl_chunk, TD0Chunk)

    assert tgl_chunk.item(1).field_value('name') == 'Kick'
    assert tgl_chunk.item(9).field_value('name') == 'HiHat'


def test_item_size(td0_bytes_minimal_w_kit: bytes) -> None:
    """Does size() return the chunk item size in bytes?"""
    td0file : TD0File = TD0File(td0_bytes_minimal_w_kit)
    kit_chunk : TD0Chunk|None = td0file.chunk('KITa')
    assert isinstance(kit_chunk, TD0Chunk)

    assert kit_chunk.item_size() == 3456


def test_copy_item(td0_bytes_exampl_tgl_items: bytes) -> None:
    """Does it copy an item to a new index, overwriting it?"""
    td0file : TD0File = TD0File(td0_bytes_exampl_tgl_items)
    tgl_chunk : TD0Chunk|None = td0file.chunk('TGLa')
    assert isinstance(tgl_chunk, TD0Chunk)

    assert tgl_chunk.item(0).field_value('name') == 'OFF'
    assert tgl_chunk.item(7).field_value('name') == 'Tom'

    tgl_chunk.copy_item(0, 7)
    assert tgl_chunk.item(0).field_value('name') == 'OFF'
    assert tgl_chunk.item(7).field_value('name') == 'OFF'


def test_copy_item_raises_on_invalid_index(td0_bytes_exampl_tgl_items: bytes) -> None:
    """Does it raise when given in invalid source or target index?"""
    td0file : TD0File = TD0File(td0_bytes_exampl_tgl_items)
    tgl_chunk : TD0Chunk|None = td0file.chunk('TGLa')
    assert isinstance(tgl_chunk, TD0Chunk)

    # This file has 10 TGLa items, the last valid index is 9.
    with pytest.raises(IndexError):
        tgl_chunk.copy_item(0, 10)

    with pytest.raises(OverflowError):
        tgl_chunk.copy_item(0, -1)

    with pytest.raises(OverflowError):
        tgl_chunk.copy_item(-1, 5)

    with pytest.raises(IndexError):
        tgl_chunk.copy_item(10, 5)


def test_replace_item(td0_bytes_exampl_tgl_items: bytes) -> None:
    """Does it replace an item at the specified index?"""
    td0file : TD0File = TD0File(td0_bytes_exampl_tgl_items)
    tgl_chunk : TD0Chunk|None = td0file.chunk('TGLa')
    assert isinstance(tgl_chunk, TD0Chunk)

    assert tgl_chunk.item(5).field_value('name') == 'Cross Stick'
    default_item = tgl_chunk.default_item()
    default_item.set_field_value('name', 'Triangle')

    tgl_chunk.replace_item(5, default_item)
    assert tgl_chunk.item(5).field_value('name') == 'Triangle'


def test_swap_items(td0_bytes_exampl_tgl_items: bytes) -> None:
    """Does it swap the items?"""
    td0file : TD0File = TD0File(td0_bytes_exampl_tgl_items)
    tgl_chunk : TD0Chunk|None = td0file.chunk('TGLa')
    assert isinstance(tgl_chunk, TD0Chunk)

    assert tgl_chunk.item(0).field_value('name') == 'OFF'
    assert tgl_chunk.item(7).field_value('name') == 'Tom'

    tgl_chunk.swap_items(0, 7)
    assert tgl_chunk.item(0).field_value('name') == 'Tom'
    assert tgl_chunk.item(7).field_value('name') == 'OFF'


def test_swap_items_only_affects_selected_items(td0_bytes_exampl_tgl_items: bytes) -> None:
    """Does it swap the items without affecting the other items?"""
    td0file : TD0File = TD0File(td0_bytes_exampl_tgl_items)
    tgl_chunk : TD0Chunk|None = td0file.chunk('TGLa')
    assert isinstance(tgl_chunk, TD0Chunk)

    orig_items = [tgl_chunk.item(idx).field_value('name') for idx in range(tgl_chunk.item_count())]
    expected_items = orig_items[:]
    expected_items[0], expected_items[7] = expected_items[7], expected_items[0]

    tgl_chunk.swap_items(0, 7)
    actual_items = [tgl_chunk.item(idx).field_value('name') for idx in range(tgl_chunk.item_count())]
    assert actual_items == expected_items


def test_reorder_items(td0_bytes_exampl_tgl_items: bytes) -> None:
    """Does it swap the items?"""
    td0file : TD0File = TD0File(td0_bytes_exampl_tgl_items)
    tgl_chunk : TD0Chunk|None = td0file.chunk('TGLa')
    assert isinstance(tgl_chunk, TD0Chunk)

    existing_order = [
        (0, 'OFF'),
        (1, 'Kick'),
        (2, 'Kick Proc/Elec'),
        (3, 'Snare'),
        (4, 'Snare Proc/Elec'),
        (5, 'Cross Stick'),
        (6, 'Clap'),
        (7, 'Tom'),
        (8, 'Tom Proc/Elec'),
        (9, 'HiHat'),
    ]

    # Ensure that the existing order matches expectations:
    for idx, expected in existing_order:
        assert tgl_chunk.item(idx).field_value('name') == expected

    desired_order = [
        # old index, expected value
        (6, 'Clap'),
        (5, 'Cross Stick'),
        (9, 'HiHat'),
        (2, 'Kick Proc/Elec'),
        (1, 'Kick'),
        (0, 'OFF'),
        (4, 'Snare Proc/Elec'),
        (3, 'Snare'),
        (8, 'Tom Proc/Elec'),
        (7, 'Tom'),
    ]

    tgl_chunk.reorder_items(tuple(item[0] for item in desired_order))

    for idx, expected_idx_and_name in enumerate(desired_order):
        _, expected_name = expected_idx_and_name
        assert tgl_chunk.item(idx).field_value('name') == expected_name


def test_item_count(td0_bytes_exampl_tgl_items: bytes) -> None:
    """Does it swap the items?"""
    td0file : TD0File = TD0File(td0_bytes_exampl_tgl_items)
    tgl_chunk : TD0Chunk|None = td0file.chunk('TGLa')
    assert isinstance(tgl_chunk, TD0Chunk)

    # tgl_chunk in this file has 10 items.
    assert tgl_chunk.item_count() == 10
