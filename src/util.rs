#[repr(C)]
union F32 {
    f: f32,
    i: u32,
}

/// Clip x to [0.f, 1.f] (inclusive)
#[inline(always)]
pub fn clip01f(x: f32) -> f32 {
    clampfsel(0.0, x, 1.0)
}

/// Clamp x to [min, max] (inclusive)
#[inline(always)]
fn clampfsel(min: f32, x: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

/// Round to nearest integer.
#[inline(always)]
pub fn si_roundf(x: f32) -> f32 {
    (x + si_copysignf(0.5, x)) as i32 as f32
}

/// Return x with sign of y applied
#[inline(always)]
fn si_copysignf(x: f32, y: f32) -> f32 {
    let mut xs = F32 { f: x };
    let ys = F32 { f: y };

    unsafe {
        xs.i &= 0x7fffffff;
        xs.i |= ys.i & 0x80000000;

        xs.f
    }
}
