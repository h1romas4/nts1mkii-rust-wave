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
        (($val as f32) * 0.0009775_f32) as f32 // 0-1023 -> 0.f-1.f (9.77517106549365e-004_f32)
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
        (unsafe { si_roundf($f32 * 1023.0) } as i32) // 0.f-1.f -> 0-1023
    };
}

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
        ($q as f32) * Q15_TO_F32_C
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
        ($q as f32) * Q31_TO_F32_C
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
        (($f * ((1 << 15)- 1) as f32) as i32).min(32767).max(-32768) as i16
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
        ($f * 0x7fffffff as f32) as i32
    };
}
