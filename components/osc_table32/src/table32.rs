use logue_sdk_v2rs::*;

pub const WAVE_TABLE_COUNT: usize = 32;
pub const WAVE_TABLE_WIDTH: usize = 32;

// wave table
include!("bss.rs");

pub struct Table32 {
    runtime_desc: *const unit_runtime_desc_t,
    wave_table: *const u8,
    phase: f32,
}

#[allow(dead_code)]
enum ParamsIndex {
    Table = 0,
    None,
    TableSel,
}

impl Table32 {
    pub fn new() -> Self {
        Self {
            runtime_desc: core::ptr::null(),
            wave_table: WAVE_TABLE_32[0].as_ptr(),
            phase: 0_f32,
        }
    }

    pub fn init(&mut self, desc: *const unit_runtime_desc_t) -> i8 {
        if desc.is_null() {
            return k_unit_err_undef as i8;
        }

        let desc = unsafe { desc.as_ref().unwrap() };

        if desc.target != unsafe { unit_header.target } {
            return k_unit_err_target as i8;
        }

        if !unit_api_is_compat!(desc.api) {
            return k_unit_err_api_version as i8;
        }

        if desc.samplerate != 48000 {
            return k_unit_err_samplerate as i8;
        }

        if desc.input_channels != 2 || desc.output_channels != 1 {
            return k_unit_err_geometry as i8;
        }

        self.runtime_desc = desc;

        k_unit_err_none as i8
    }

    pub fn teardown(&mut self) {}

    pub fn reset(&mut self) {}

    pub fn resume(&mut self) {}

    pub fn suspend(&mut self) {}

    pub fn process(&mut self, _input: *const f32, output: *mut f32, frames: u32) {
        let ctxt = unsafe {
            (*self.runtime_desc).hooks.runtime_context as *const unit_runtime_osc_context_t
        };

        let w0 = unsafe {
            osc_w0f_for_note((((*ctxt).pitch) >> 8) as u8, (((*ctxt).pitch) & 0xff) as u8)
        };

        let mut y = output;
        let y_e = unsafe { y.offset(frames as isize) };

        while y != y_e {
            let index = unsafe { si_roundf((self.phase + 1.0) / 2.0 * (WAVE_TABLE_WIDTH - 1) as f32) } as isize;
            let sig = unsafe { *self.wave_table.offset(index) };
            let sig = (sig as i8) as f32 / 128.0;

            unsafe {
                *y = sig;
                y = y.offset(1);
            }
            self.phase += w0;
            self.phase -= self.phase as i32 as f32;
        }
    }

    pub fn set_parameter(&mut self, index: u8, value: i32) {
        match index {
            i if i == ParamsIndex::Table as u8 || i == ParamsIndex::TableSel as u8 => {
                self.wave_table =
                    unsafe { WAVE_TABLE_32.as_ptr().offset(value as isize) } as *const u8;
                self.phase = 0_f32;
            }
            _ => {}
        }
    }

    pub fn get_parameter_value(&self, index: u8) -> i32 {
        match index {
            i if i == ParamsIndex::Table as u8 || i == ParamsIndex::TableSel as u8 => {
                self.wave_table as i32
            }
            _ => {
                0
            }
        }
    }

    pub fn get_parameter_str_value(&self, _index: u8, _value: i32) -> *const core::ffi::c_char {
        core::ptr::null()
    }

    pub fn note_on(&self, _note: u8, _velo: u8) {}

    pub fn note_off(&self, _note: u8) {}

    pub fn all_note_off(&self) {}

    pub fn pitch_bend(&self, _bend: u8) {}

    pub fn channel_pressure(&self, _press: u8) {}

    pub fn after_touch(&self, _note: u8, _press: u8) {}
}
