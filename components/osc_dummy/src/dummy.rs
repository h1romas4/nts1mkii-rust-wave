use logue_bind::*;

pub struct Dummy {
    runtime_desc: *const unit_runtime_desc_t,
    phase: f32,
}

impl Dummy {
    pub fn new() -> Self {
        Self {
            runtime_desc: core::ptr::null(),
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
            let sig = if self.phase - 0.5_f32 <= 0.0_f32 {
                1.0_f32
            } else {
                -1.0_f32
            };
            unsafe {
                *y = sig;
                y = y.offset(1);
            }
            self.phase += w0;
            self.phase -= self.phase as i32 as f32;
        }
    }

    pub fn set_parameter(&mut self, _index: u8, _value: i32) {}

    pub fn get_parameter_value(&self, _index: u8) -> i32 {
        0
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
