#![no_std]
#![no_main]
#![allow(nonstandard_style)]
#![allow(clippy::all)]

mod macros;

// Include louge-sdk bindings (bindgen generated)
include!("bindings_libnts1mkii.rs");

// Definition of const not generated by bindgen
pub const k_samplerate_recipf: f64 = 2.08333333333333e-005_f64;
pub const Q15_TO_F32_C: f32 = 3.05175781250000e-005;
pub const Q31_TO_F32_C: f32 = 4.65661287307739e-010;

// For libm error handling
#[no_mangle]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn __errno() {
}
