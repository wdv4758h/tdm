//! Toy Math Library

use std::ops::{Add, Mul};

/// Calculate sine function with Taylor series as approximation approach
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
/// Error(x) = | sin(x) - taylor_sin(x) |
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
pub fn taylor_sin<T>(x: T) -> T
    where T: Mul<T, Output=T> + Mul<f64, Output=T> +
             Add<T, Output=T> + Add<f64, Output=T> +
             Copy
{

    const A0: f64 =  1.0;                                   //  1/1!
    const A1: f64 = -1.666666666666666666666666666666E-1;   // -1/3!
    const A2: f64 =  8.333333333333333333333333333333E-3;   //  1/5!
    const A3: f64 = -1.984126984126984126984126984126E-4;   // -1/7!
    const A4: f64 =  2.755731922398589065255731922398E-6;   //  1/9!
    const A5: f64 = -2.505210838544171877505210838544E-8;   // -1/11!
    const A6: f64 =  1.605904383682161459939237717015E-10;  //  1/13!

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

/// Calculate cosine function with Taylor series as approximation approach
///
///
/// ```raw
///              x^2   x^4   x^6   x^8   x^10   x^12
/// sin(x) = 1 - --- + --- - --- + --- - ---- + ---- - ...
///               2!    4!    6!    8!    10!    12!
/// ```
///
/// Error Function :
///
/// ```raw
/// Error(x) = | cos(x) - taylor_cos(x) |
/// ```
///
/// * the error getting smaller when x getting close to 0
/// * the error getting greater when x getting far from 0
///
/// limit the x will make precision greater
///
/// trigonometric identities we can use :
///
/// * cos(2π+β) = cos(β)
/// * cos(π+β) = -cos(β)
/// * cos(-π/2+β) = sin(β)
///
/// `sin(x) = sin(απ+β) = (-1)^α sin(β), ∀ α ∈ ℤ, β ∈ ℝ, 0 ≦ β < π`
///
pub fn taylor_cos<T>(x: T) -> T
    where T: Mul<T, Output=T> + Mul<f64, Output=T> +
             Add<T, Output=T> + Add<f64, Output=T> +
             Copy
{

    const A0: f64 =  1.0;                                   //  1/0!
    const A1: f64 = -0.5;                                   // -1/2!
    const A2: f64 =  4.166666666666666666666666666666E-2;   //  1/4!
    const A3: f64 = -1.388888888888888888888888888888E-3;   // -1/6!
    const A4: f64 =  2.480158730158730158730158730158E-5;   //  1/8!
    const A5: f64 = -2.755731922398589065255731922398E-7;   // -1/10!
    const A6: f64 =  2.087675698786809897921009032120E-9;   //  1/12!


    let x2 = x * x;

    ((((((
        x2 * A6
           + A5) * x2
           + A4) * x2
           + A3) * x2
           + A2) * x2
           + A1) * x2
           + A0)

}

/// sine function for f64
///
/// Currently, we call [taylor_sin] and [taylor_cos] as implementation.
///
/// [taylor_sin]: fn.taylor_sin.html
/// [taylor_cos]: fn.taylor_cos.html
///
pub fn sin(data: f64) -> f64 {
    use std::f64::consts::{PI, FRAC_PI_2, FRAC_PI_4};

    // [-∞, ∞] => (-π, π)
    // sin(θ) = -sin(θ-π)
    let (data, neg_flag) = (data % PI, ((data / PI) as u8) % 2 != 0);
    // (-π, π) => (0, π)
    // sin(θ) = -sin(-θ)
    let (data, neg_flag) = (data.abs(), neg_flag ^ (data < 0.));

    let ret = match data {
        data if ((0. <= data) && (data < FRAC_PI_4)) => {
            // sin(θ)
            taylor_sin(data)
        },
        data if ((FRAC_PI_4 <= data) && (data < (PI - FRAC_PI_4))) => {
            // sin(θ) = cos(θ-π/2)
            taylor_cos(data - FRAC_PI_2)
        },
        data if (((PI - FRAC_PI_4) <= data) && (data < PI)) => {
            // sin(θ) = sin(π-θ)
            taylor_sin(PI - data)
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


/// cosine function for f64
///
/// Currently, we call [sin] as implementation.
///
/// [sin]: fn.sin.html
pub fn cos(data: f64) -> f64 {
    use std::f64::consts::FRAC_PI_2;

    sin(FRAC_PI_2 - data)
}

/// cosine function for f32
///
/// Currently, we call [cos] as implementation.
///
/// [cos]: fn.cos.html
pub fn cosf(data: f32) -> f32 {
    cos(data as f64) as f32
}
