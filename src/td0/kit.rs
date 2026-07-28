use zerocopy::{LittleEndian, U16, I16};
use zerocopy_derive::{FromBytes, IntoBytes, KnownLayout};
use libtd0_derive::{repeat_fields,TD0ChunkItem};
use crate::td0::chunk::{
    ChunkItem, ChunkItemValue, ChunkItemValueRaw, Volume,
};
use crate::td0::result::{TD0Result};
use rust_decimal::Decimal;

/*
pub const FIELDS_KIT_A_ITEM: [&str; 70] = [
    "name", "memo",

    // Kit settings fields:
    "unknown_1",
    "tempo",
    "unknown_2",
    "mfx_1_routing",
    "mfx_2_routing",
    "mfx_3_routing",
    "mfx_4_routing",
    "mfx_1_direct_out_routing",
    "mfx_2_direct_out_routing",
    "mfx_3_direct_out_routing",
    "mfx_4_direct_out_routing",
    "sidechain_routing",
    "sidechain_direct_out_routing",
    "unknown_3",
    "pad_sequence_pad",
    "pad_sequence_1",
    "pad_sequence_2",
    "pad_sequence_3",
    "pad_sequence_4",
    "pad_sequence_5",
    "pad_sequence_6",
    "pad_sequence_7",
    "pad_sequence_8",
    "pad_sequence_9",
    "pad_sequence_10",
    "pad_sequence_11",
    "pad_sequence_12",
    "pad_sequence_13",
    "pad_sequence_14",
    "pad_sequence_15",
    "pad_sequence_16",
    "unknown_4",
    "pad_1_led_mode",
    "pad_2_led_mode",
    "pad_3_led_mode",
    "pad_4_led_mode",
    "pad_5_led_mode",
    "pad_6_led_mode",
    "pad_7_led_mode",
    "pad_8_led_mode",
    "pad_9_led_mode",
    "unknown_5",
    "led_1",
    "led_2",
    "led_3",
    "led_4",
    "led_5",
    "led_6",
    "led_7",
    "led_8",
    "led_9",
    "led_10",
    "led_11",
    "led_12",
    "led_13",
    "led_14",
    "led_15",
    "unknown_6",
    "pad_edit_knob_mode",
    "pad_edit_knob_1_group",
    "pad_edit_knob_2_group",
    "unknown_7",
    "pad_edit_knob_1_param",
    "pad_edit_knob_2_param",
    "unknown_8",
    "exp_pedal_assign",
    "exp_pedal_mode",
    "unknown_9",

];
 */

pub const SYS_KIT_SWITCH : [&str ; 2] = [
    "SYSTEM",
    "KIT",
];

const TEMPO_MIN : Decimal = Decimal::from_parts(20, 0, 0, false, 0);
const TEMPO_MAX : Decimal = Decimal::from_parts(260, 0, 0, false, 0);

#[repeat_fields]
#[derive(Clone, Copy, Debug, FromBytes, IntoBytes, KnownLayout, TD0ChunkItem)]
#[repr(C, packed)]
pub struct KITaItem {
    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    name: [u8; 16],

    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    memo: [u8; 64],

    #[td0_field(field_type = "Volume")]
    volume: I16<LittleEndian>,

    #[td0_field(field_type = "EnumStr", collection = "SYS_KIT_SWITCH")]
    click_setting: u8,

    #[td0_field(field_type = "U8")]
    unknown_1: u8,

    tempo: U16<LittleEndian>,

    unknown_2: [u8; 3],
    #[repeat(count = 4, format = "mfx_{}_routing")]
    mfx_1_routing: u8,
    #[repeat(count = 4, format = "mfx_{}_direct_out_routing")]
    mfx_1_direct_out_routing: u8,
    sidechain_routing: u8,
    sidechain_direct_out_routing: u8,
    unknown_3: [u8; 3],
    pad_sequence_pad: u8,
    // Pad sequence steps (1 - 16): 0 - 19 = pads 1 - 9, triggers 1 - 8, ftswitches 1 - 2, 20 = SKIP
    #[repeat(count = 16, format = "pad_sequence_{}")]
    pad_sequence_: u8,
    unknown_4: [u8; 21],
    // Pad 1 - 9 LED Mode
    #[repeat(count = 9, format = "pad_{}_led_mode")]
    pad_1_led_mode: u8,
    unknown_5: [u8; 9],
    // LED 1 - 15
    #[repeat(count = 15, format = "led_{}")]
    led_1: u8,
    unknown_6: [u8; 3],
    pad_edit_knob_mode: u8,
    pad_edit_knob_1_group: u8,
    pad_edit_knob_2_group: u8,
    unknown_7: [u8; 2],
    pad_edit_knob_1_param: u8,
    pad_edit_knob_2_param: u8,
    unknown_8: [u8; 6],
    exp_pedal_assign: u8,
    exp_pedal_mode: u8,
    unknown_9: [u8; 13],
}

/*
impl Default for KITaItem {
    fn default() -> Self {
        Self{
            name: [0; 16],
            memo: [0; 64],
            /***************************
             *   Kit Settings section
             ***************************
             */
            volume: U16::from(0),
            click_setting: 0,
            unknown_1: 0,
            tempo: U16::from(0),
            unknown_2: [0; 3],
            mfx_1_routing: 0,
            mfx_2_routing: 0,
            mfx_3_routing: 0,
            mfx_4_routing: 0,
            mfx_1_direct_out_routing: 0,
            mfx_2_direct_out_routing: 0,
            mfx_3_direct_out_routing: 0,
            mfx_4_direct_out_routing: 0,
            sidechain_routing: 0,
            sidechain_direct_out_routing: 0,
            unknown_3: [0; 3],
            pad_sequence_pad: 0,
            // Pad sequence steps (1 - 16): 0 - 19 = pads 1 - 9, triggers 1 - 8, ftswitches 1 - 2, 20 = SKIP
            //
            unknown_4: [0; 21],
            // Pad 1 - 9 LED Mode
            pad_1_led_mode: 0,
            pad_2_led_mode: 0,
            pad_3_led_mode: 0,
            pad_4_led_mode: 0,
            pad_5_led_mode: 0,
            pad_6_led_mode: 0,
            pad_7_led_mode: 0,
            pad_8_led_mode: 0,
            pad_9_led_mode: 0,

            unknown_5: [0; 9],
            // LED 1 - 15
            led_1: 0,
            led_2: 0,
            led_3: 0,
            led_4: 0,
            led_5: 0,
            led_6: 0,
            led_7: 0,
            led_8: 0,
            led_9: 0,
            led_10: 0,
            led_11: 0,
            led_12: 0,
            led_13: 0,
            led_14: 0,
            led_15: 0,

            unknown_6: [0; 3],
            pad_edit_knob_mode: 0,
            pad_edit_knob_1_group: 0,
            pad_edit_knob_2_group: 0,
            unknown_7: [0; 2],
            pad_edit_knob_1_param: 0,
            pad_edit_knob_2_param: 0,
            unknown_8: [0; 6],
            exp_pedal_assign: 0,
            exp_pedal_mode: 0,
            unknown_9: [0; 13],
        }
    }
} */

/*
impl ChunkItem for KITaItem {
    fn get_value(&self, field: &str) -> Option<ChunkItemValue> {
        match field {
            "name" => Some(ChunkItemValue::Text(
                String::from_utf8_lossy(&self.name).to_string(),
            )),
            "memo" => Some(ChunkItemValue::Text(
                String::from_utf8_lossy(&self.memo).to_string(),
            )),
            _ => None,
        }
    }

    fn set_value(&mut self, field: &str, value: &ChunkItemValue) -> TD0Result<()> {
        match (field, value) {
            ("name", ChunkItemValue::Text(val)) => {
                copy_ascii_str_to_native(val, &mut self.name, field, 0x20)?;
            }
            ("memo", ChunkItemValue::Text(val)) => {
                copy_ascii_str_to_native(val, &mut self.memo, field, 0x20)?;
            }
            _ => {
                return Err(TD0Error::ConvertToNativeTypeError {
                    field: field.to_string(),
                    reason: "Unknown field or improper value type.".to_string(),
                });
            }
        }
        Ok(())
    }

    fn get_value_raw(&self, field: &str) -> Option<ChunkItemValueRaw> {
        match field {
            "name" => Some(ChunkItemValueRaw::Slice(Box::new(self.name.clone()))),
            "memo" => Some(ChunkItemValueRaw::Slice(Box::new(self.memo.clone()))),
            _ => None,
        }
    }

    fn set_value_raw(&mut self, field: &str, value: &ChunkItemValueRaw) -> TD0Result<()> {
        match (field, value) {
            ("name", ChunkItemValueRaw::Slice(val)) => {
                copy_slice_to_native(val, &mut self.name, field)?;
            }
            ("memo", ChunkItemValueRaw::Slice(val)) => {
                copy_slice_to_native(val, &mut self.memo, field)?;
            }
            _ => {
                return Err(TD0Error::ConvertToNativeTypeError {
                    field: field.to_string(),
                    reason: "Unknown field or improper value type.".to_string(),
                });
            }
        }

        Ok(())
    }

    // fn get_fields(&self) -> &'static [&'static str] { &FIELDS_KIT_A_ITEM }
} */
