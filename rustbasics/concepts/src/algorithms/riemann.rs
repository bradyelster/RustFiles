// An attribute to hide warnings for unused code.
#![allow(dead_code)]

pub fn riemann<F>(f: F, endpoints: (f64, f64), n_subdivisions: u64) -> f64
where 
    F: Fn(f64) -> f64, // The type of function `f` must accept a f64 and return a f64
{
    let (a, b) = endpoints;
    let delta_x: f64 = (b - a).abs() / (n_subdivisions as f64);
    let mut sum: f64 = 0.0;
    
    for i in 0..n_subdivisions {
        let x = a + i as f64 * delta_x;
        sum += delta_x * f(x);
    }
    sum
}

pub fn round_to_n_decimal(x: f64, places: u32) -> f64 {
    let scale: f64 = 10f64.powi(places as i32);
    (x * scale).round() / scale
}

// Define the integrand f(x)
pub fn integrand(x: f64) -> f64 {
    x.powi(5) + 1.0
}

fn main() {
    let bounds: (f64, f64) = (0.0, 1.0);
    let n: u64 = 5000;
    let decimals: u32 = 5;
    let integral: f64 = riemann(integrand, bounds, n);
    let exact: f64 = 7.0 / 6.0;
    let roundoff: f64 = round_to_n_decimal((((exact - integral).abs()) / exact) * 100.0, decimals);

    println!("The integral is approximately: {}", round_to_n_decimal(integral, decimals));
    println!("The error is: {} percent", roundoff);
}