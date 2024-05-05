#![no_std]
#![no_main]
#![allow(nonstandard_style)]
#![allow(clippy::all)]
include!("bindings_libnts1mkii.rs");

use core::panic::PanicInfo;

const K_WAVES_A_CNT: i16 = (k_waves_a_cnt + k_waves_b_cnt + k_waves_c_cnt) as i16;
const K_WAVES_B_CNT: i16 = (k_waves_d_cnt + k_waves_e_cnt + k_waves_f_cnt) as i16;
const SUB_WAVE_CNT: i16 = k_waves_a_cnt as i16;

///
/// The unit header
///
#[link_section = ".unit_header"]
#[used]
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
    dev_id: 0x4f523148, // H1RO
    // ID for this unit. Scoped within the context of a given dev_id.
    unit_id: 0x050400,
    // Name for this unit, will be displayed on device
    name: unsafe { core::mem::transmute(*b"rWAV\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0") },
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
            name: unsafe { core::mem::transmute(*b"SHPE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0") },
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
            name: unsafe { core::mem::transmute(*b"SUB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0") },
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
            name: unsafe { core::mem::transmute(*b"WAVE A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0") },
        },
        unit_param_t {
            min: 0,
            max: K_WAVES_B_CNT - 1,
            center: 0,
            init: 0,
            type_: k_unit_param_type_enum as u8,
            _bitfield_align_1: [0u8; 0],
            // frac : 4, frac_mode : 1, reserved : 3
            _bitfield_1: __BindgenBitfieldUnit::new([0b00000000; 1]),
            name: unsafe { core::mem::transmute(*b"WAVE B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0") },
        },
        unit_param_t {
            min: 0,
            max: SUB_WAVE_CNT - 1,
            center: 0,
            init: 0,
            type_: k_unit_param_type_enum as u8,
            _bitfield_align_1: [0u8; 0],
            // frac : 4, frac_mode : 1, reserved : 3
            _bitfield_1: __BindgenBitfieldUnit::new([0b00000000; 1]),
            name: unsafe { core::mem::transmute(*b"SUB WAVE\0\0\0\0\0\0\0\0\0\0\0\0\0\0") },
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
            name: unsafe { core::mem::transmute(*b"RING MIX\0\0\0\0\0\0\0\0\0\0\0\0\0\0") },
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
            name: unsafe { core::mem::transmute(*b"BIT CRUSH\0\0\0\0\0\0\0\0\0\0\0\0\0") },
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
            name: unsafe { core::mem::transmute(*b"DRIFT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0") },
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

#[no_mangle]
pub extern "C" fn unit_init(_arg1: *const unit_runtime_desc_t) -> i8 {
    k_unit_err_none as i8
}

#[no_mangle]
pub extern "C" fn unit_teardown() {

}

#[no_mangle]
pub extern "C" fn unit_reset() {

}

#[no_mangle]
pub extern "C" fn unit_resume() {

}

#[no_mangle]
pub extern "C" fn unit_suspend() {

}

#[no_mangle]
pub extern "C" fn unit_render(_arg1: *const f32, _arg2: *mut f32, _arg3: u32) {

}

#[no_mangle]
pub extern "C" fn unit_get_param_value(_arg1: u8) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn unit_get_param_str_value(_arg1: u8, _arg2: i32) -> *const core::ffi::c_char {
    core::ptr::null()
}

#[no_mangle]
pub extern "C" fn unit_set_param_value(_arg1: u8, _arg2: i32) {

}

#[no_mangle]
pub extern "C" fn unit_set_tempo(_arg1: u32) {

}

#[no_mangle]
pub extern "C" fn unit_tempo_4ppqn_tick(_arg1: u32) {

}

#[no_mangle]
pub extern "C" fn unit_note_on(_arg1: u8, _arg2: u8) {

}

#[no_mangle]
pub extern "C" fn unit_note_off(_arg1: u8) {

}

#[no_mangle]
pub extern "C" fn unit_all_note_off() {

}

#[no_mangle]
pub extern "C" fn unit_pitch_bend(_arg1: u16) {

}

#[no_mangle]
pub extern "C" fn unit_channel_pressure(_arg1: u8) {

}

#[no_mangle]
pub extern "C" fn unit_aftertouch(_arg1: u8, _arg2: u8) {

}

/// Custom panic handler that loops indefinitely.
///
/// # Arguments
///
/// * `_info` - Information about the panic
///
/// # Panics
///
/// This function never returns and loops indefinitely.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
