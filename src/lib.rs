#![no_std]
#![no_main]
#![allow(nonstandard_style)]
#![allow(clippy::all)]
mod header;
mod waves;
mod libm_sqrtf;

use core::{mem::MaybeUninit, panic::PanicInfo};
use waves::Waves;

// Include louge-sdk bindings (bindgen generated)
include!("bindings_libnts1mkii.rs");
// Definition of const not generated by bindgen
#[allow(unused)]
const k_samplerate_recipf: f64 = 2.08333333333333e-005_f64;

// Oscillator is placed in memory.
#[used]
static mut WAVES: MaybeUninit<Waves> = MaybeUninit::uninit();

///
/// unit_init
///
/// Called after unit is loaded. Should be used to perform basic checks on runtime environment,
/// initialize the unit, and allocate any external memory if needed.
/// `desc` provides a description of the current runtime environment.
///
/// # Arguments
///
/// * `desc` - Pointer to the unit runtime description
///
/// # Returns
///
/// Returns an `i8` indicating the result of the initialization process.
#[no_mangle]
pub extern "C" fn unit_init(desc: *const unit_runtime_desc_t) -> i8 {
    unsafe {
        WAVES.as_mut_ptr().write(Waves::new());
        WAVES.assume_init_mut().init(desc)
    }
}

///
/// unit_teardown
///
/// Called before unit is unloaded. Should be used to free any external memory if needed.
///
#[no_mangle]
pub extern "C" fn unit_teardown() {
    unsafe {
        WAVES.assume_init_mut().teardown();
    }
}

///
/// unit_reset
///
/// Called when unit must be reset to a neutral state. Active notes must be deactivated,
/// enveloppe generators reset to their neutral state, oscillator phases reset,
/// delay lines set to be cleared (clearing at once may be to heavy). However,
/// parameter values should not be reset to their default values.
///
#[no_mangle]
pub extern "C" fn unit_reset() {
    unsafe {
        WAVES.assume_init_mut().reset();
    }
}

///
/// unit_resume
///
/// Called just before a unit will start processing again after being suspended.
///
#[no_mangle]
pub extern "C" fn unit_resume() {
    unsafe {
        WAVES.assume_init_mut().resume();
    }
}

///
/// unit_suspend
///
/// Called when a unit is being suspended. For instance,
/// when the currently active unit is being swapped for a different unit.
/// Usually followed by a call to unit_reset().
///
#[no_mangle]
pub extern "C" fn unit_suspend() {
    unsafe {
        WAVES.assume_init_mut().suspend();
    }
}

///
/// unit_render
///
/// Audio rendering callback. Input/output buffer geometry information is provided
/// via the unit_runtime_desc_t argument of unit_init(..).
///
#[no_mangle]
pub extern "C" fn unit_render(input: *const f32, output: *mut f32, frames: u32) {
    unsafe {
        WAVES.assume_init_mut().process(input, output, frames);
    }
}

///
/// unit_get_param_value
///
/// Called to obtain the current value of the parameter designated by the specified index.
///
#[no_mangle]
pub extern "C" fn unit_get_param_value(index: u8) -> i32 {
    unsafe {
        WAVES.assume_init_mut().get_parameter_value(index)
    }
}

///
/// unit_get_param_str_value
///
/// Called to obtain the string representation of the specified value,
/// for a k_unit_param_type_strings typed parameter.
/// The returned value should point to a nul-terminated 7-bit ASCII C string of maximum X characters.
/// It can be safely assumed that the C string pointer will not be cached
/// or reused until unit_get_param_str_value(..) is called again,
/// and thus the same memory area can be reused across calls (if convenient).
///
#[no_mangle]
pub extern "C" fn unit_get_param_str_value(index: u8, value: i32) -> *const core::ffi::c_char {
    unsafe {
        WAVES.assume_init_mut().get_parameter_str_value(index, value)
    }
}

///
/// unit_set_param_value
///
/// Called to set the current value of the parameter designated by the specified index.
/// Note that for the NTS-1 digital kit mkII values are stored as 16-bit integers,
/// but to avoid future API changes, they are passed as 32bit integers.
/// For additional safety, make sure to bound check values as per the min/max values declared in the header.
///
#[no_mangle]
pub extern "C" fn unit_set_param_value(index: u8, value: i32) {
    unsafe {
        WAVES.assume_init_mut().set_parameter(index, value);
    }
}

///
/// unit_set_tempo
///
/// Called when a tempo change occurs. The tempo is formatted in fixed point format,
/// with the BPM integer part in the upper 16 bits, and fractional part in the lower 16 bits (low endian).
/// Care should be taken to keep CPU load as low as possible
/// when handling tempo changes as this handler may be called frequently especially if externally synced.
///
#[no_mangle]
pub extern "C" fn unit_set_tempo(_tempo: u32) {
}

///
/// unit_tempo_4ppqn_tick
///
/// After initialization, the callback may be called at any time to notify
/// the unit of a clock event (4PPQN interval, ie: 16th notes with regards to tempo).
#[no_mangle]
pub extern "C" fn unit_tempo_4ppqn_tick(_tempo: u32) {
}

///
/// Oscillator Unit Specific Functions
///

///
/// unit_note_on
///
/// Called upon MIDI note on events, and upon internal sequencer gate on events
/// if an explicit unit_gate_on(..) handler is not provided,
/// in which case note will be set to 0xFFU. velocity is a 7-bit value.
///
#[no_mangle]
pub extern "C" fn unit_note_on(note: u8, velo: u8) {
    unsafe {
        WAVES.assume_init_mut().note_on(note, velo);
    }
}

///
/// unit_note_off
///
/// Called upon MIDI note off events, and upon internal sequencer gate off events
/// if an explicit unit_gate_off(..) handler is not provided, in which case note will be set to 0xFFU.
///
#[no_mangle]
pub extern "C" fn unit_note_off(note: u8) {
    unsafe {
        WAVES.assume_init_mut().note_off(note);
    }
}

///
/// unit_all_note_off
///
/// When called all active notes should be deactivated and enveloppe generators reset.
///
#[no_mangle]
pub extern "C" fn unit_all_note_off() {
    unsafe {
        WAVES.assume_init_mut().all_note_off();
    }
}

///
/// unit_pitch_bend
///
/// Called upon MIDI pitch bend event. bend is a 14-bit value with neutral center at 0x2000U.
/// Sensitivity can be defined according to the unit's needs.
///
#[no_mangle]
pub extern "C" fn unit_pitch_bend(bend: u8) {
    unsafe {
        WAVES.assume_init_mut().pitch_bend(bend);
    }
}

///
/// unit_channel_pressure
///
/// Called upon MIDI channel pressure events. pressure is a 7-bit value.
///
#[no_mangle]
pub extern "C" fn unit_channel_pressure(press: u8) {
    unsafe {
        WAVES.assume_init_mut().channel_pressure(press);
    }
}

///
/// Called upon MIDI aftertouch events. afterotuch is a 7-bit value.
///
#[no_mangle]
pub extern "C" fn unit_aftertouch(note: u8, press: u8) {
    unsafe {
        WAVES.assume_init_mut().after_touch(note, press);
    }
}

///
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
    loop { }
}

///
/// The `sound_unit_string` macro converts a given `u8` array into an `i8` array of a specified length,
/// padding the right side with `\0` if necessary.
///
/// # Arguments
///
/// * `$s` - An array of `u8`. The elements of this array are cast to `i8`.
/// * `$len` - The length of the array to be generated. If the length of `$s` is shorter than `$len`,
///   the remaining elements of the generated array are filled with `\0`.
///
/// # Returns
///
/// An `i8` array of the specified length. The elements of the array are the elements of `$s` cast to `i8`,
/// and `\0` as needed to fill the array to the specified length.
///
/// # Example
///
/// ```
/// static NAME: [i8; 16] = sound_unit_string!(b"Waves", 16);
/// ```
///
/// This code converts the `u8` array `b"Waves"` into an `i8` array, and pads the right side with `\0`
/// to make the length of the array 16.
///
#[macro_export]
macro_rules! sound_unit_string {
    ($s:expr, $len:expr) => {{
        let mut arr = [0i8; $len];
        let mut i = 0;
        while i < $len && i < $s.len() {
            arr[i] = $s[i] as i8;
            i += 1;
        }
        arr
    }};
}

///
/// The `sound_unit_dev_id_string` macro converts a 4-character ASCII string into a `u32`.
///
/// # Arguments
///
/// * `$s` - A 4-character ASCII string. Each character of this string is cast to `u32`.
///
/// # Returns
///
/// A `u32` that is the result of converting the 4-character ASCII string. Each character is converted as follows:
/// * The first character is shifted left by 24 bits.
/// * The second character is shifted left by 16 bits.
/// * The third character is shifted left by 8 bits.
/// * The fourth character is used as is.
/// These results are then OR'd together at the bit level.
///
/// # Example
///
/// ```
/// let result = sound_unit_dev_id_string!(b"H1RO");
/// ```
///
/// This code converts the ASCII string `b"H1RO"` into a `u32`. The result is `0x4831524f`.
///
#[macro_export]
macro_rules! sound_unit_dev_id_string {
    ($s:expr) => {{
        assert!($s.len() == 4, "Input string must be 4 characters long");
        (($s[0] as u32) << 24) | (($s[1] as u32) << 16) | (($s[2] as u32) << 8) | ($s[3] as u32)
    }};
}

///
/// The `create_unit_param_bitfield` macro creates a new instance of `__BindgenBitfieldUnit` and sets its fields.
///
/// # Arguments
///
/// * `$frac` - The value to be set in the first 4 bits of the `__BindgenBitfieldUnit`.
/// * `$frac_mode` - The value to be set in the 5th bit of the `__BindgenBitfieldUnit`.
/// * `$reserved` - The value to be set in the last 3 bits (6th to 8th) of the `__BindgenBitfieldUnit`.
///
/// # Returns
///
/// A `__BindgenBitfieldUnit` with its fields set according to the provided arguments.
///
/// # Example
///
/// ```
/// let unit = create_unit_param_bitfield!(4, 1, 3);
/// ```
///
/// This code creates a `__BindgenBitfieldUnit` with the first 4 bits set to 4, the 5th bit set to 1, and the last 3 bits set to 3.
///
#[macro_export]
macro_rules! create_unit_param_bitfield {
    ($frac:expr, $frac_mode:expr, $reserved:expr) => {{
        let mut unit = __BindgenBitfieldUnit::new();
        unit.set(0..4, $frac);
        unit.set(4..5, $frac_mode);
        unit.set(5..8, $reserved);
        unit
    }};
}

/// unit_api_is_compat
///
/// Checks if the provided API version is compatible with the current unit API version.
///
/// # Arguments
///
/// * `$api` - The API version to check compatibility with.
///
/// # Returns
///
/// Returns a boolean value indicating whether the provided API version is compatible with the current unit API version.
///
/// # Example
///
/// ```
/// let is_compat = unit_api_is_compat!(crate::k_unit_api_2_0_0);
/// ```
///
/// This code checks if the API version `crate::k_unit_api_2_0_0` is compatible with the current unit API version.
/// The result is a boolean value indicating the compatibility.
#[macro_export]
macro_rules! unit_api_is_compat {
    ($api:expr) => {
        (($api & crate::UNIT_API_MAJOR_MASK) == (crate::k_unit_api_2_0_0 & crate::UNIT_API_MAJOR_MASK))
            && (($api & crate::UNIT_API_MINOR_MASK) <= (crate::k_unit_api_2_0_0 & crate::UNIT_API_MINOR_MASK))
    };
}

/// Converts a 10-bit parameter value to a floating-point value.
///
/// # Arguments
///
/// * `val` - The 10-bit parameter value to convert.
///
/// # Returns
///
/// The converted floating-point value.
#[macro_export]
macro_rules! param_10bit_to_f32 {
    ($val:expr) => {
        (($val as f32) * 9.77517106549365e-004_f32) as i32
    };
}

/// Converts a floating-point value to a 10-bit parameter value.
///
/// # Arguments
///
/// * `val` - The floating-point value to convert.
///
/// # Returns
///
/// The converted 10-bit parameter value.
#[macro_export]
macro_rules! param_f32_to_10bit {
    ($f32:expr) => {
        (unsafe { si_roundf($f32 * 1023.0) } as i32)
    };
}

#[allow(unused)]
const Q15_TO_F32_C: f32 = 3.05175781250000e-005;
const Q31_TO_F32_C: f32 = 4.65661287307739e-010;

/// The `q15_to_f32` macro converts a value in Q15 format to F32 format.
///
/// # Arguments
///
/// * `$q` - The value in Q15 format. This is represented as a value of type `i16`.
///
/// # Returns
///
/// * The value in F32 format. This is represented as a value of type `f32`.
///
/// # Example
///
/// ```
/// let q: i16 = 32767;
/// let f: f32 = q15_to_f32!(q);  // f will be approximately 1.0.
/// ```
#[macro_export]
macro_rules! q15_to_f32 {
    ($q:expr) => {
        ($q as f32) * crate::Q15_TO_F32_C
    };
}

/// The `q31_to_f32` macro converts a value in Q31 format to F32 format.
///
/// # Arguments
///
/// * `$q` - The value in Q31 format. This is represented as a value of type `i32`.
///
/// # Returns
///
/// * The value in F32 format. This is represented as a value of type `f32`.
///
/// # Example
///
/// ```
/// let q: i32 = 2147483647;
/// let f: f32 = q31_to_f32!(q);  // f will be approximately 1.0.
/// ```
#[macro_export]
macro_rules! q31_to_f32 {
    ($q:expr) => {
        ($q as f32) * crate::Q31_TO_F32_C
    };
}

/// The `f32_to_q15` macro converts a value in F32 format to Q15 format.
///
/// # Arguments
///
/// * `$f` - The value in F32 format. This is represented as a value of type `f32`.
///
/// # Returns
///
/// * The value in Q15 format. This is represented as a value of type `i16`.
///   The result is clamped to the range of `i16` to prevent overflow.
///
/// # Example
///
/// ```
/// let f: f32 = 1.0;
/// let q: i16 = f32_to_q15!(f);  // q will be 32767.
/// ```
#[macro_export]
macro_rules! f32_to_q15 {
    ($f:expr) => {
        (($f * ((1<<15)-1) as f32) as i32).min(32767).max(-32768) as i16
    };
}

/// The `f32_to_q31` macro converts a value in F32 format to Q31 format.
///
/// # Arguments
///
/// * `$f` - The value in F32 format. This is represented as a value of type `f32`.
///
/// # Returns
///
/// * The value in Q31 format. This is represented as a value of type `i32`.
///
/// # Example
///
/// ```
/// let f: f32 = 1.0;
/// let q: i32 = f32_to_q31!(f);  // q will be 2147483647.
/// ```
#[macro_export]
macro_rules! f32_to_q31 {
    ($f:expr) => {
        ($f * 0x7FFFFFFF as f32) as i32
    };
}
