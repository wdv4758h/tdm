use sin::sin;

use std::ops::{Add, Mul};

/// Calculate cosine function with modified Taylor series as approximation approach
///
///
/// ```raw
///              x^2   x^4   x^6   x^8   x^10   x^12
/// cos(x) = 1 - --- + --- - --- + --- - ---- + ---- - ...
///               2!    4!    6!    8!    10!    12!
/// ```
///
/// Error Function :
///
/// ```raw
/// Error(x) = | cos(x) - poly_cos(x) |
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
pub fn poly_cos<T>(x: T) -> T
    where T: Mul<T, Output=T> + Mul<f64, Output=T> +
             Add<T, Output=T> + Add<f64, Output=T> +
             Copy
{

    // taylor
    // const A0: f64 =  1.0;                                   //  1/0!
    // const A1: f64 = -0.5;                                   // -1/2!
    // const A2: f64 =  4.166666666666666666666666666666E-2;   //  1/4!
    // const A3: f64 = -1.388888888888888888888888888888E-3;   // -1/6!
    // const A4: f64 =  2.480158730158730158730158730158E-5;   //  1/8!
    // const A5: f64 = -2.755731922398589065255731922398E-7;   // -1/10!
    // const A6: f64 =  2.087675698786809897921009032120E-9;   //  1/12!

    // modified
    const A0: f64 =  1.0;
    const A1: f64 = -0.5;
    const A2: f64 =  4.16666666666666019037E-2;
    const A3: f64 = -1.38888888888741095749E-3;
    const A4: f64 =  2.48015872894767294178E-05;
    const A5: f64 = -2.75573143513906633035E-7;
    const A6: f64 =  2.08757232129817482790E-9;
    //const A7: f64 = -1.13596475577881948265E-11;

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

/// cosine function for f64
///
/// Currently, we call [sin] as implementation.
///
/// cos(θ) = sin(π/2 - θ)
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
