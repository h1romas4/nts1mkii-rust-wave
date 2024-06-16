#![no_std]
#![no_main]
mod header;
mod waves;
use core::{mem::MaybeUninit, panic::PanicInfo};
use logue_sdk_v2rs::*;
use waves::Waves;

// Oscillator is placed in memory.
#[used]
static mut UNIT: MaybeUninit<Waves> = MaybeUninit::uninit();

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
        UNIT.as_mut_ptr().write(Waves::new());
        UNIT.assume_init_mut().init(desc)
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
        UNIT.assume_init_mut().teardown();
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
        UNIT.assume_init_mut().reset();
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
        UNIT.assume_init_mut().resume();
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
        UNIT.assume_init_mut().suspend();
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
        UNIT.assume_init_mut().process(input, output, frames);
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
        UNIT.assume_init_mut().get_parameter_value(index)
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
        UNIT.assume_init_mut().get_parameter_str_value(index, value)
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
        UNIT.assume_init_mut().set_parameter(index, value);
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
        UNIT.assume_init_mut().note_on(note, velo);
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
        UNIT.assume_init_mut().note_off(note);
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
        UNIT.assume_init_mut().all_note_off();
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
        UNIT.assume_init_mut().pitch_bend(bend);
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
        UNIT.assume_init_mut().channel_pressure(press);
    }
}

///
/// Called upon MIDI aftertouch events. afterotuch is a 7-bit value.
///
#[no_mangle]
pub extern "C" fn unit_aftertouch(note: u8, press: u8) {
    unsafe {
        UNIT.assume_init_mut().after_touch(note, press);
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
