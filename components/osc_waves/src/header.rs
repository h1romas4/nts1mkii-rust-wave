use logue_bind::*;
use crate::waves::{K_WAVES_A_CNT, K_WAVES_B_CNT, SUB_WAVE_CNT};

///
/// The sound unit header.
///
#[used]
#[link_section = ".unit_header"]
#[export_name = "unit_header"]
pub static UNIT_HEADER: unit_header_t = unit_header_t {
    // This unit's version: major.minor.patch (major<<16 minor<<8 patch).
    version: 0x00020000,
    // Size of this header. Leave as is.
    header_size: core::mem::size_of::<unit_header_t>() as u32,
    // Target platform and module pair for this unit
    target: k_unit_target_nts1_mkii | k_unit_module_osc,
    // API version for which unit was built. See runtime.h
    api: k_unit_api_2_0_0,
    // Developer ID. See https://github.com/korginc/logue-sdk/blob/master/developer_ids.md
    dev_id: sound_unit_dev_id_string!(b"H1RO"),
    // ID for this unit. Scoped within the context of a given dev_id.
    unit_id: 0x050400,
    // Name for this unit, will be displayed on device
    name: sound_unit_string!(b"rWAV", 20),
    // Reserved
    reserved0: 0,
    reserved1: 0,
    // Number of valid parameter descriptors. (max. 10)
    num_params: 8,
    // Format:
    // min, max, center, default, type, frac. bits, frac. mode, <reserved>, name
    // See common/runtime.h for type enum and unit_param_t structure
    params: [
        // Fixed/direct UI parameters
        // A knob
        unit_param_t {
            min: 0,
            max: 1023,
            center: 0,
            init: 0,
            type_: k_unit_param_type_none as u8,
            _bitfield_align_1: [0u8; 0],
            // frac : 4, frac_mode : 1, reserved : 3
            _bitfield_1: __BindgenBitfieldUnit::new([0b00000000; 1]),
            name: sound_unit_string!(b"SHPE", 22),
        },
        // B knob
        unit_param_t {
            min: 0,
            max: 1023,
            center: 0,
            init: 0,
            type_: k_unit_param_type_none as u8,
            _bitfield_align_1: [0u8; 0],
            // frac : 4, frac_mode : 1, reserved : 3
            _bitfield_1: __BindgenBitfieldUnit::new([0b00000000; 1]),
            name: sound_unit_string!(b"SUB", 22),
        },
        // 8 Edit menu parameters
        unit_param_t {
            min: 0,
            max: K_WAVES_A_CNT - 1,
            center: 0,
            init: 0,
            type_: k_unit_param_type_enum as u8,
            _bitfield_align_1: [0u8; 0],
            // frac : 4, frac_mode : 1, reserved : 3
            _bitfield_1: __BindgenBitfieldUnit::new([0b00000000; 1]),
            name: sound_unit_string!(b"WAVE A", 22),
        },
        unit_param_t {
            min: 0,
            max: K_WAVES_B_CNT - 1,
            center: 0,
            init: 0,
            type_: k_unit_param_type_none as u8,
            _bitfield_align_1: [0u8; 0],
            // frac : 4, frac_mode : 1, reserved : 3
            _bitfield_1: __BindgenBitfieldUnit::new([0b00000000; 1]),
            name: sound_unit_string!(b"WAVE B", 22),
        },
        unit_param_t {
            min: 0,
            max: SUB_WAVE_CNT - 1,
            center: 0,
            init: 0,
            type_: k_unit_param_type_none as u8,
            _bitfield_align_1: [0u8; 0],
            // frac : 4, frac_mode : 1, reserved : 3
            _bitfield_1: __BindgenBitfieldUnit::new([0b00000000; 1]),
            name: sound_unit_string!(b"SUB WAVE", 22),
        },
        unit_param_t {
            min: 0,
            max: 1000,
            center: 0,
            init: 0,
            type_: k_unit_param_type_percent as u8,
            _bitfield_align_1: [0u8; 0],
            // frac : 4, frac_mode : 1, reserved : 3
            _bitfield_1: __BindgenBitfieldUnit::new([0b00010001; 1]),
            name: sound_unit_string!(b"RING MIX", 22),
        },
        unit_param_t {
            min: 0,
            max: 1000,
            center: 0,
            init: 0,
            type_: k_unit_param_type_none as u8,
            _bitfield_align_1: [0u8; 0],
            // frac : 4, frac_mode : 1, reserved : 3
            _bitfield_1: __BindgenBitfieldUnit::new([0b00010001; 1]),
            name: sound_unit_string!(b"BIT CRUSH", 22),
        },
        unit_param_t {
            min: 0,
            max: 1000,
            center: 0,
            init: 250,
            type_: k_unit_param_type_percent as u8,
            _bitfield_align_1: [0u8; 0],
            // frac : 4, frac_mode : 1, reserved : 3
            _bitfield_1: __BindgenBitfieldUnit::new([0b00010001; 1]),
            name: sound_unit_string!(b"DRIFT", 22),
        },
        unit_param_t {
            min: 0,
            max: 0,
            center: 0,
            init: 0,
            type_: k_unit_param_type_none as u8,
            _bitfield_align_1: [0u8; 0],
            // frac : 4, frac_mode : 1, reserved : 3
            _bitfield_1: __BindgenBitfieldUnit::new([0u8; 1]),
            name: [0x0; 22],
        },
        unit_param_t {
            min: 0,
            max: 0,
            center: 0,
            init: 0,
            type_: k_unit_param_type_none as u8,
            _bitfield_align_1: [0u8; 0],
            // frac : 4, frac_mode : 1, reserved : 3
            _bitfield_1: __BindgenBitfieldUnit::new([0u8; 1]),
            name: [0x0; 22],
        },
        unit_param_t {
            min: 0,
            max: 0,
            center: 0,
            init: 0,
            type_: k_unit_param_type_none as u8,
            _bitfield_align_1: [0u8; 0],
            // frac : 4, frac_mode : 1, reserved : 3
            _bitfield_1: __BindgenBitfieldUnit::new([0u8; 1]),
            name: [0x0; 22],
        },
    ],
};
