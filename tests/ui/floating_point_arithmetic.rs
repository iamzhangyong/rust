#![allow(dead_code)]
#![warn(clippy::floating_point_improvements)]

const TWO: f32 = 2.0;
const E: f32 = std::f32::consts::E;

fn check_log_base() {
    let x = 1f32;
    let _ = x.log(2f32);
    let _ = x.log(10f32);
    let _ = x.log(std::f32::consts::E);
    let _ = x.log(TWO);
    let _ = x.log(E);

    let x = 1f64;
    let _ = x.log(2f64);
    let _ = x.log(10f64);
    let _ = x.log(std::f64::consts::E);
}

fn check_ln1p() {
    let x = 1f32;
    let _ = (1.0 + x).ln();
    let _ = (1.0 + x * 2.0).ln();
    let _ = (1.0 + x.powi(2)).ln();
    let _ = (1.0 + x.powi(2) * 2.0).ln();
    let _ = (1.0 + (std::f32::consts::E - 1.0)).ln();
    // Cases where the lint shouldn't be applied
    let _ = (x + 1.0).ln();
    let _ = (1.0 + x + 2.0).ln();
    let _ = (1.0 + x - 2.0).ln();

    let x = 1f64;
    let _ = (1.0 + x).ln();
    let _ = (1.0 + x * 2.0).ln();
    let _ = (1.0 + x.powi(2)).ln();
    // Cases where the lint shouldn't be applied
    let _ = (x + 1.0).ln();
    let _ = (1.0 + x + 2.0).ln();
    let _ = (1.0 + x - 2.0).ln();
}

fn check_powf() {
    let x = 3f32;
    let _ = 2f32.powf(x);
    let _ = std::f32::consts::E.powf(x);
    let _ = x.powf(1.0 / 2.0);
    let _ = x.powf(1.0 / 3.0);
    let _ = x.powf(2.0);
    let _ = x.powf(-2.0);
    let _ = x.powf(2.1);
    let _ = x.powf(-2.1);
    let _ = x.powf(16_777_217.0);
    let _ = x.powf(-16_777_217.0);

    let x = 3f64;
    let _ = 2f64.powf(x);
    let _ = std::f64::consts::E.powf(x);
    let _ = x.powf(1.0 / 2.0);
    let _ = x.powf(1.0 / 3.0);
    let _ = x.powf(2.0);
    let _ = x.powf(-2.0);
    let _ = x.powf(2.1);
    let _ = x.powf(-2.1);
    let _ = x.powf(9_007_199_254_740_993.0);
    let _ = x.powf(-9_007_199_254_740_993.0);
}

fn check_expm1() {
    let x = 2f32;
    let _ = x.exp() - 1.0;
    let _ = x.exp() - 1.0 + 2.0;
    // Cases where the lint shouldn't be applied
    let _ = x.exp() - 2.0;
    let _ = x.exp() - 1.0 * 2.0;

    let x = 2f64;
    let _ = x.exp() - 1.0;
    let _ = x.exp() - 1.0 + 2.0;
    // Cases where the lint shouldn't be applied
    let _ = x.exp() - 2.0;
    let _ = x.exp() - 1.0 * 2.0;
}

fn main() {}
