use sin::{sin, sinf};
use cos::{cos, cosf};

/// tangent function for f64
///
/// Currently, we call [sin] and [cos] as implementation.
///
/// [sin]: fn.sin.html
/// [cos]: fn.cos.html
pub fn tan(data: f64) -> f64 {
    sin(data) / cos(data)
}

/// tangent function for f32
///
/// Currently, we call [sinf] and [cosf] as implementation.
///
/// [sinf]: fn.sinf.html
/// [cosf]: fn.cosf.html
pub fn tanf(data: f32) -> f32 {
    sinf(data) / cosf(data)
}

