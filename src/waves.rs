use core::sync::atomic::AtomicU32;

use crate::header::{K_WAVES_A_CNT, K_WAVES_B_CNT, SUB_WAVE_CNT};
use crate::{
    clip01f, clip1m1f, fastertanh2f, k_samplerate_recipf, k_unit_err_api_version,
    k_unit_err_geometry, k_unit_err_none, k_unit_err_samplerate, k_unit_err_target,
    k_unit_err_undef, k_waves_a_cnt, k_waves_b_cnt, k_waves_c_cnt, k_waves_d_cnt, k_waves_e_cnt,
    k_waves_f_cnt, osc_bitresf, osc_w0f_for_note, osc_wave_scanf, osc_white, param_10bit_to_f32,
    q31_to_f32, si_roundf, unit_api_is_compat, unit_header, unit_runtime_desc_t,
    unit_runtime_osc_context_t, wavesA, wavesB, wavesC, wavesD, wavesE, wavesF,
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

#[allow(unused)]
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

    pub fn process(&mut self, _input: *const f32, output: *mut f32, frames: u32) {
        let _state = &self.state;
        let _params: &Params = &self.params;
        let ctxt = unsafe {
            (*self.runtime_desc).hooks.runtime_context as *const unit_runtime_osc_context_t
        };

        // Handle events.
        {
            self.update_pitch(unsafe {
                osc_w0f_for_note((((*ctxt).pitch) >> 8) as u8, (((*ctxt).pitch) & 0xff) as u8)
                    as f32
            });
            let flags = self.state.flags.swap(
                StateFlags::K_FLAGS_NONE as u32,
                core::sync::atomic::Ordering::Relaxed,
            );
            self.update_waves(flags);

            if flags & StateFlags::K_FLAG_RESET as u32 != 0 {
                self.reset();
            }

            self.state.lfo = unsafe { q31_to_f32!((*ctxt).shape_lfo) };

            if flags & StateFlags::K_FLAG_BIT_CRUSH as u32 != 0 {
                self.state.dither = self.params.bit_crush * 2e-008_f32;
                self.state.bit_res = unsafe { osc_bitresf(self.params.bit_crush) };
                self.state.bit_res_recip = 1.0_f32 / self.state.bit_res;
            }
        }

        // Temporaries.
        let mut phi_a: f32 = self.state.phi_a;
        let mut phi_b: f32 = self.state.phi_b;
        let mut phi_sub: f32 = self.state.phi_sub;

        let mut lfoz: f32 = self.state.lfoz;
        let lfo_inc: f32 = (self.state.lfo - lfoz) / frames as f32;

        let _ditheramt = self.params.bit_crush * 2e-008_f32;

        let sub_mix = self.params.sub_mix * 0.5011872336272722_f32;
        let ring_mix = self.params.ring_mix;

        let mut y = output as *mut f32;
        let y_e = unsafe { y.offset(frames as isize) };

        while y != y_e {
            let wave_mix = unsafe { clip01f(self.params.shape + lfoz) };

            let mut sig =
                (1.0_f32 - wave_mix) * unsafe { osc_wave_scanf(self.state.wave_a, phi_a) };
            sig += wave_mix * unsafe { osc_wave_scanf(self.state.wave_b, phi_b) };

            let sub_sig = unsafe { osc_wave_scanf(self.state.sub_wave, phi_sub) };
            sig = (1.0_f32 - ring_mix) * sig + ring_mix * 1.4125375446227544_f32 * (sub_sig * sig);
            sig += sub_mix * sub_sig;
            sig *= 1.4125375446227544_f32;
            sig = unsafe { clip1m1f(fastertanh2f(sig)) };

            // TODO: dsp::BiQuad

            // *(y++) = sig;
            unsafe {
                *y = sig;
                y = y.offset(1);
            }

            phi_a += self.state.w0_a;
            phi_a -= phi_a as i32 as f32;
            phi_b += self.state.w0_b;
            phi_b -= phi_b as i32 as f32;
            phi_sub += self.state.w0_sub;
            phi_sub -= phi_sub as i32 as f32;
            lfoz += lfo_inc;
        }

        // Update state
        self.state.phi_a = phi_a;
        self.state.phi_b = phi_b;
        self.state.phi_sub = phi_sub;
        self.state.lfoz = lfoz;
    }

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
                self.params.ring_mix = unsafe { clip01f(value as f32 * 0.001_f32) };
            }
            i if i == ParamsIndex::K_BIT_CRUSH as u8 => {
                //  min, max,  center, default, type,                      frac, frac. mode, <reserved>, name
                // {0,   1000,  0,      0,       k_unit_param_type_percent, 1,    1,          0,          {"BIT CRUSH"}},
                self.params.bit_crush = unsafe { clip01f(value as f32 * 0.001_f32) };
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
                return unsafe { si_roundf(self.params.ring_mix * 1000_f32) as i32 };
            }
            i if i == ParamsIndex::K_BIT_CRUSH as u8 => {
                //  min, max,  center, default, type,                      frac, frac. mode, <reserved>, name
                // {0,   1000,  0,      0,       k_unit_param_type_percent, 1,    1,          0,          {"BIT CRUSH"}},
                return unsafe { si_roundf(self.params.bit_crush * 1000_f32) as i32 };
            }
            i if i == ParamsIndex::K_DRIFT as u8 => {
                //  min, max,  center, default, type,                      frac, frac. mode, <reserved>, name
                // {0,   1000,  0,      250,     k_unit_param_type_percent, 1,    1,          0,          {"DRIFT"}},
                return unsafe { si_roundf((self.params.drift - 1.0_f32) * 1000_f32) as i32 };
            }
            _ => 0,
        }
    }

    pub fn get_parameter_str_value(&self, index: u8, _value: i32) -> *const core::ffi::c_char {
        match index {
            // Note: String memory must be accessible even after function returned.
            //       It can be assumed that caller will have copied or used the string
            //       before the next call to getParameterStrValue

            // Currently no Parameters of type k_unit_param_type_strings.
            _ => core::ptr::null(),
        }
    }

    pub fn note_on(&self, _note: u8, _velo: u8) {
        // Schedule phase reset
        self.state.flags.fetch_or(
            StateFlags::K_FLAG_RESET as u32,
            core::sync::atomic::Ordering::Relaxed,
        );

        // TODO: should we still fully rely on osc context pitch?
    }

    pub fn note_off(&self, _note: u8) {}

    pub fn all_note_off(&self) {}

    pub fn pitch_bend(&self, _bend: u8) {}

    pub fn channel_pressure(&self, _press: u8) {}

    pub fn after_touch(&self, _note: u8, _press: u8) {}

    fn update_pitch(&mut self, w0: f32) {
        let w0: f32 = w0 + self.state.imperfection;
        let drift = self.params.drift;
        self.state.w0_a = w0;
        // Alt. osc with slight phase drift (0.25Hz@48KHz)
        self.state.w0_b = w0 + drift * 5.20833333333333e-006_f32;
        // Sub one octave down, with a phase drift (0.15Hz@48KHz)
        self.state.w0_sub = 0.5_f32 * w0 + drift * 3.125e-006_f32;
    }

    fn update_waves(&mut self, flags: u32) {
        if flags & StateFlags::K_FLAG_WAVE_A as u32 != 0 {
            let k_a_thr: u8 = k_waves_a_cnt as u8;
            let k_b_thr: u8 = k_a_thr + k_waves_b_cnt as u8;
            let _k_c_thr: u8 = k_b_thr + k_waves_c_cnt as u8;

            let mut idx: u8 = self.params.wave_a;

            if idx < k_a_thr {
                self.state.wave_a = unsafe { get_waves_ptr(wavesA.as_ptr(), idx) };
            } else if idx < k_b_thr {
                idx -= k_a_thr;
                self.state.wave_a = unsafe { get_waves_ptr(wavesB.as_ptr(), idx) };
            } else {
                idx -= k_b_thr;
                self.state.wave_a = unsafe { get_waves_ptr(wavesC.as_ptr(), idx) };
            }
        }
        if flags & StateFlags::K_FLAG_WAVE_B as u32 != 0 {
            let k_d_thr: u8 = k_waves_d_cnt as u8;
            let k_e_thr: u8 = k_d_thr + k_waves_e_cnt as u8;
            let _k_f_thr: u8 = k_e_thr + k_waves_f_cnt as u8;

            let mut idx: u8 = self.params.wave_b;

            if idx < k_d_thr {
                self.state.wave_b = unsafe { get_waves_ptr(wavesD.as_ptr(), idx) };
            } else if idx < k_e_thr {
                idx -= k_d_thr;
                self.state.wave_b = unsafe { get_waves_ptr(wavesE.as_ptr(), idx) };
            } else {
                // if (idx < k_f_thr) {
                idx -= k_e_thr;
                self.state.wave_b = unsafe { get_waves_ptr(wavesF.as_ptr(), idx) };
            }
        }
        if flags & StateFlags::K_FLAG_SUB_WAVE as u32 != 0 {
            self.state.sub_wave = unsafe { get_waves_ptr(wavesA.as_ptr(), self.params.sub_wave) };
        }
    }
}

#[inline(always)]
pub fn get_waves_ptr(table: *const *const f32, idx: u8) -> *const f32 {
    unsafe { *table.offset(idx as isize) }
}
