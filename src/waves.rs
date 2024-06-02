use core::sync::atomic::AtomicU32;

use crate::header:: {
    K_WAVES_A_CNT,
    K_WAVES_B_CNT,
    SUB_WAVE_CNT,
};
use crate::{
    k_samplerate_recipf,
    k_unit_err_api_version,
    k_unit_err_geometry,
    k_unit_err_none,
    k_unit_err_samplerate,
    k_unit_err_target,
    k_unit_err_undef,
    param_10bit_to_f32,
    unit_api_is_compat,
    unit_header,
    unit_runtime_desc_t,
    osc_white,
    wavesA,
    wavesD,
};

pub struct Params {
    sub_mix: f32,
    ring_mix: f32,
    bit_crush: f32,
    shape: f32,
    drift: f32,
    wave_a: u8,
    wave_b: u8,
    sub_wave: u8,
    padding: u8,
}

enum ParamsIndex {
    K_SHAPE = 0,
    K_SUB_MIX,
    K_WAVE_A,
    K_WAVE_B,
    K_SUB_WAVE,
    K_RING_MIX,
    K_BIT_CRUSH,
    K_DRIFT,
}

impl Params {
    pub fn new() -> Self {
        Self {
            sub_mix: 0.05_f32,
            ring_mix: 0_f32,
            bit_crush: 0_f32,
            shape: 0_f32,
            drift: 1.25_f32,
            wave_a: 0,
            wave_b: 0,
            sub_wave: 0,
            padding: 0,
        }
    }

    pub fn reset(&mut self) {
        self.sub_mix = 0.05_f32;
        self.ring_mix = 0_f32;
        self.bit_crush = 0_f32;
        self.shape = 0_f32;
        self.drift = 1.25_f32;
        self.wave_a = 0;
        self.wave_b = 0;
        self.sub_wave = 0;
        self.padding = 0;
    }
}

enum StateFlags {
    K_FLAGS_NONE = 0,
    K_FLAG_WAVE_A = 1 << 1,
    K_FLAG_WAVE_B = 1 << 2,
    K_FLAG_SUB_WAVE = 1 << 3,
    K_FLAG_RING_MIX = 1 << 4,
    K_FLAG_BIT_CRUSH = 1 << 5,
    K_FLAG_RESET = 1 << 6,
}

struct State {
    // selected wave a data
    wave_a: *const f32,
    // selected wave b data
    wave_b: *const f32,
    // selected sub wave data
    sub_wave: *const f32,
    // wave a phase
    phi_a: f32,
    // wave b phase
    phi_b: f32,
    // sub wave phase
    phi_sub: f32,
    // wave a phase increment
    w0_a: f32,
    // wave b phase increment
    w0_b: f32,
    // sub wave phase increment
    w0_sub: f32,
    // target lfo value
    lfo: f32,
    // current interpolated lfo value
    lfoz: f32,
    // dithering amount before bit reduction
    dither: f32,
    // bit depth scaling factor
    bit_res: f32,
    // bit depth scaling reciprocal, returns signal to 0.-1.f after scaling/rounding
    bit_res_recip: f32,
    // tuning imperfection
    imperfection: f32,
    // flags passed to audio processing thread
    flags: AtomicU32,
}

impl State {
    pub fn new() -> Self {
        let mut state = Self {
            wave_a: unsafe { wavesA[0] },
            wave_b: unsafe { wavesD[0] },
            sub_wave: unsafe { wavesA[0] },
            phi_a: 0_f32,
            phi_b: 0_f32,
            phi_sub: 0_f32,
            w0_a: 440.0_f32 * k_samplerate_recipf as f32,
            w0_b: 440.0_f32 * k_samplerate_recipf as f32,
            w0_sub: 220.0_f32 * k_samplerate_recipf as f32,
            lfo: 0_f32,
            lfoz: 0_f32,
            dither: 0_f32,
            bit_res: 0_f32,
            bit_res_recip: 0_f32,
            imperfection: 0_f32,
            flags: AtomicU32::new(StateFlags::K_FLAGS_NONE as u32),
        };
        Self::reset(&mut state);
        // +/- 0.05Hz@48KHz
        state.imperfection = unsafe { osc_white() } * 1.0417e-006_f32;
        state
    }

    pub fn reset(&mut self) {
        self.phi_a = 0_f32;
        self.phi_b = 0_f32;
        self.phi_sub = 0_f32;
        self.lfo = self.lfoz
    }
}

pub struct Waves {
    state: State,
    params: Params,
    runtime_desc: *const unit_runtime_desc_t,
}

impl Waves {
    pub fn new() -> Self {
        Self {
            state: State::new(),
            params: Params::new(),
            runtime_desc: core::ptr::null(),
        }
    }

    pub fn init(&mut self, desc: *const unit_runtime_desc_t) -> i8 {
        if desc.is_null() {
            return k_unit_err_undef as i8;
        }

        // get desc reference
        let desc = unsafe { desc.as_ref().unwrap() };

        // Note: make sure the unit is being loaded to the correct platform/module target
        if desc.target != unsafe { unit_header.target } {
            return k_unit_err_target as i8;
        }

        // Check compatibility of samplerate with unit, for NTS-1 MKII should be 48000
        if !unit_api_is_compat!(desc.api) {
            return k_unit_err_api_version as i8;
        }

        // Check compatibility of samplerate with unit, for NTS-1 MKII should be 48000
        if desc.samplerate != 48000 {
            return k_unit_err_samplerate as i8;
        }

        // Check compatibility of frame geometry
        if desc.input_channels != 2 || desc.output_channels != 1 {
            // should be stereo input / mono output
            return k_unit_err_geometry as i8;
        }

        // Initialize pre/post filter coefficients
        // TODO:
        // prelpf_.mCoeffs.setPoleLP(0.9f);
        // postlpf_.mCoeffs.setFOLP(osc_tanpif(0.45f));

        // Cache runtime descriptor to keep access to API hooks
        self.runtime_desc = desc;

        // Make sure parameters are reset to default values
        self.params.reset();

        k_unit_err_none as i8
    }

    pub fn teardown(&mut self) {
        // Note: cleanup and release resources if any
    }

    pub fn reset(&mut self) {
        // Note: Reset effect state, excluding exposed parameter values.
        self.state.reset();
    }

    pub fn resume(&mut self) {
        // Unit will resume and exit suspend state.
        // Unit was selected and the render callback will start being called again
        self.state.reset();
    }

    pub fn suspend(&mut self) {
        // Unit will enter suspend state.
        // Unit was deselected and the render callback will stop being called
    }

    pub fn process(&mut self) {}

    pub fn set_parameter(&mut self, index: u8, value: i32) {
        match index {
            i if i == ParamsIndex::K_SHAPE as u8 => {
                //  min, max,  center, default, type,                   frac, frac. mode, <reserved>, name
                // {0,   1023, 0,      0,       k_unit_param_type_none, 0,    0,          0,          {"SHAPE"}},
                self.params.shape = 0.005_f32 + param_10bit_to_f32!(value) as f32 * 0.99_f32;
            }
            i if i == ParamsIndex::K_SUB_MIX as u8 => {
                //  min, max,  center, default, type,                   frac, frac. mode, <reserved>, name
                // {0,   1023, 0,      0,       k_unit_param_type_none, 0,    0,          0,          {"SUB"}},
                self.params.sub_mix = 0.05_f32 + param_10bit_to_f32!(value) as f32 * 0.90_f32;
            }
            i if i == ParamsIndex::K_WAVE_A as u8 => {
                //  min, max,          center, default, type,                   frac, frac. mode, <reserved>, name
                // {0,   WAVE_A_CNT-1, 0,      0,       k_unit_param_type_enum, 0,    0,          0,          {"WAVE A"}},
                self.params.wave_a = (value as i16 % K_WAVES_A_CNT) as u8;
                self.state.flags.fetch_or(
                    StateFlags::K_FLAG_WAVE_A as u32,
                    core::sync::atomic::Ordering::Relaxed,
                );
            }
            i if i == ParamsIndex::K_WAVE_B as u8 => {
                //  min, max,          center, default, type,                   frac, frac. mode, <reserved>, name
                // {0,   WAVE_B_CNT-1, 0,      0,       k_unit_param_type_enum, 0,    0,          0,          {"WAVE B"}},
                self.params.wave_a = (value as i16 % K_WAVES_B_CNT) as u8;
                self.state.flags.fetch_or(
                    StateFlags::K_FLAG_WAVE_B as u32,
                    core::sync::atomic::Ordering::Relaxed,
                );
            }
            i if i == ParamsIndex::K_SUB_WAVE as u8 => {
                //  min, max,            center, default, type,                   frac, frac. mode, <reserved>, name
                // {0,   SUB_WAVE_CNT-1, 0,      0,       k_unit_param_type_enum, 0,    0,          0,          {"SUB WAVE"}},
                self.params.wave_a = (value as i16 % SUB_WAVE_CNT) as u8;
                self.state.flags.fetch_or(
                    StateFlags::K_FLAG_SUB_WAVE as u32,
                    core::sync::atomic::Ordering::Relaxed,
                );
            }
            i if i == ParamsIndex::K_RING_MIX as u8 => {
                //  min, max,  center, default, type,                      frac, frac. mode, <reserved>, name
                // {0,   1000,  0,      0,       k_unit_param_type_percent, 1,    1,          0,          {"RING MIX"}},
                // self.params.ring_mix = unsafe { clip01f(value as f32 * 0.001_f32) };
            }
            i if i == ParamsIndex::K_BIT_CRUSH as u8 => {
                //  min, max,  center, default, type,                      frac, frac. mode, <reserved>, name
                // {0,   1000,  0,      0,       k_unit_param_type_percent, 1,    1,          0,          {"BIT CRUSH"}},
                // self.params.bit_crush = unsafe { clip01f(value as f32 * 0.001_f32) };
                self.state.flags.fetch_or(
                    StateFlags::K_FLAG_BIT_CRUSH as u32,
                    core::sync::atomic::Ordering::Relaxed,
                );
            }
            i if i == ParamsIndex::K_DRIFT as u8 => {
                //  min, max,  center, default, type,                      frac, frac. mode, <reserved>, name
                // {0,   1000,  0,      250,     k_unit_param_type_percent, 1,    1,          0,          {"DRIFT"}},
                self.params.drift = 1.0_f32 + value as f32 * 0.001_f32;
            }
            _ => {}
        }
    }

    pub fn get_parameter_value(&self, index: u8) -> i32 {
        match index {
            i if i == ParamsIndex::K_SHAPE as u8 => {
                //  min, max,  center, default, type,                   frac, frac. mode, <reserved>, name
                // {0,   1023, 0,      0,       k_unit_param_type_none, 0,    0,          0,          {"SHAPE"}},
                return param_10bit_to_f32!((self.params.shape - 0.05_f32) / 0.99_f32);
            }
            i if i == ParamsIndex::K_SUB_MIX as u8 => {
                //  min, max,  center, default, type,                   frac, frac. mode, <reserved>, name
                // {0,   1023, 0,      0,       k_unit_param_type_none, 0,    0,          0,          {"SUB"}},
                return param_10bit_to_f32!((self.params.sub_mix - 0.05_f32) / 0.90_f32);
            }
            i if i == ParamsIndex::K_WAVE_A as u8 => {
                //  min, max,          center, default, type,                   frac, frac. mode, <reserved>, name
                // {0,   WAVE_A_CNT-1, 0,      0,       k_unit_param_type_enum, 0,    0,          0,          {"WAVE A"}},
                return self.params.wave_a as i32;
            }
            i if i == ParamsIndex::K_WAVE_B as u8 => {
                //  min, max,          center, default, type,                   frac, frac. mode, <reserved>, name
                // {0,   WAVE_B_CNT-1, 0,      0,       k_unit_param_type_enum, 0,    0,          0,          {"WAVE B"}},
                return self.params.wave_b as i32;
            }
            i if i == ParamsIndex::K_SUB_WAVE as u8 => {
                //  min, max,            center, default, type,                   frac, frac. mode, <reserved>, name
                // {0,   SUB_WAVE_CNT-1, 0,      0,       k_unit_param_type_enum, 0,    0,          0,          {"SUB WAVE"}},
                return self.params.sub_wave as i32;
            }
            i if i == ParamsIndex::K_RING_MIX as u8 => {
                //  min, max,  center, default, type,                      frac, frac. mode, <reserved>, name
                // {0,   1000,  0,      0,       k_unit_param_type_percent, 1,    1,          0,          {"RING MIX"}},
                // return unsafe { si_roundf(self.params.ring_mix * 1000_f32) as i32 };
                0
            }
            i if i == ParamsIndex::K_BIT_CRUSH as u8 => {
                //  min, max,  center, default, type,                      frac, frac. mode, <reserved>, name
                // {0,   1000,  0,      0,       k_unit_param_type_percent, 1,    1,          0,          {"BIT CRUSH"}},
                // return unsafe { si_roundf(self.params.bit_crush * 1000_f32) as i32 };
                0
            }
            i if i == ParamsIndex::K_DRIFT as u8 => {
                //  min, max,  center, default, type,                      frac, frac. mode, <reserved>, name
                // {0,   1000,  0,      250,     k_unit_param_type_percent, 1,    1,          0,          {"DRIFT"}},
                // return unsafe { si_roundf((self.params.drift - 1.0_f32) * 1000_f32) as i32 };
                0
            }
            _ => 0,
        }
    }

    pub fn get_parameter_str_value(&self, index: u8, _value: i32) -> *const core::ffi::c_char {
        match index {
            _ => core::ptr::null(),
        }
    }
}
