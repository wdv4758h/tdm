use cos::poly_cos;

use std::ops::{Add, Mul};

/// Calculate sine function with modified Taylor series as approximation approach
///
///
/// ```raw
///              x^3   x^5   x^7   x^9   x^11   x^13
/// sin(x) = x - --- + --- - --- + --- - ---- + ---- - ...
///               3!    5!    7!    9!    11!    13!
/// ```
///
/// Error Function :
///
/// ```raw
/// Error(x) = | sin(x) - poly_sin(x) |
/// ```
///
/// * the error getting smaller when x getting close to 0
/// * the error getting greater when x getting far from 0
///
/// limit the x will make precision greater
///
/// trigonometric identities we can use :
///
/// * sin(2π+β) = sin(β)
/// * sin(π+β) = -sin(β)
/// * sin(-π/2+β) = -cos(β)
///
/// `sin(x) = sin(απ+β) = (-1)^α sin(β), ∀ α ∈ ℤ, β ∈ ℝ, 0 ≦ β < π`
///
pub fn poly_sin<T>(x: T) -> T
    where T: Mul<T, Output=T> + Mul<f64, Output=T> +
             Add<T, Output=T> + Add<f64, Output=T> +
             Copy
{

    // taylor
    // const A0: f64 =  1.0;                                   //  1/1!
    // const A1: f64 = -1.666666666666666666666666666666E-1;   // -1/3!
    // const A2: f64 =  8.333333333333333333333333333333E-3;   //  1/5!
    // const A3: f64 = -1.984126984126984126984126984126E-4;   // -1/7!
    // const A4: f64 =  2.755731922398589065255731922398E-6;   //  1/9!
    // const A5: f64 = -2.505210838544171877505210838544E-8;   // -1/11!
    // const A6: f64 =  1.605904383682161459939237717015E-10;  //  1/13!

    // modified
    const A0: f64 =  1.0;
    const A1: f64 = -1.66666666666666324348E-1;
    const A2: f64 =  8.33333333332248946124E-3;
    const A3: f64 = -1.98412698298579493134E-4;
    const A4: f64 =  2.75573137070700676789E-6;
    const A5: f64 = -2.50507602534068634195E-8;
    const A6: f64 =  1.58969099521155010221E-10;

    let x2 = x * x;

    ((((((
        x2 * A6
           + A5) * x2
           + A4) * x2
           + A3) * x2
           + A2) * x2
           + A1) * x2
           + A0) * x

}

/// sine function for f64
///
/// Currently, we call [poly_sin] and [poly_cos] as implementation.
///
/// [poly_sin]: fn.poly_sin.html
/// [poly_cos]: fn.poly_cos.html
///
pub fn sin(data: f64) -> f64 {
    use std::f64;
    use std::f64::consts::{PI, FRAC_PI_2, FRAC_PI_4};

    if data.is_nan() || data.is_infinite() {
        return f64::NAN;
    }

    // [-∞, ∞] => (-π, π)
    // sin(θ) = -sin(θ-π)
    let (data, neg_flag) = (data % PI, ((data / PI) as u8) % 2 != 0);
    // (-π, π) => (0, π)
    // sin(θ) = -sin(-θ)
    let (data, neg_flag) = (data.abs(), neg_flag ^ (data < 0.));

    let ret = match data {
        data if ((0. <= data) && (data < FRAC_PI_4)) => {
            // sin(θ)
            poly_sin(data)
        },
        data if ((FRAC_PI_4 <= data) && (data < (PI - FRAC_PI_4))) => {
            // sin(θ) = cos(θ-π/2)
            poly_cos(data - FRAC_PI_2)
        },
        data if (((PI - FRAC_PI_4) <= data) && (data < PI)) => {
            // sin(θ) = sin(π-θ)
            poly_sin(PI - data)
        },
        _ => { panic!("why are you here ?") }
    };

    if neg_flag {
        return -ret;
    }

    ret
}

/// sine function for f32
///
/// Currently, we call [sin] as implementation.
///
/// [sin]: fn.sin.html
pub fn sinf(data: f32) -> f32 {
    sin(data as f64) as f32
}
