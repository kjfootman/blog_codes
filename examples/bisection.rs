#![allow(non_snake_case)]
use std::error::Error;
use blog_codes::numerics;

fn main() -> Result<(), Box<dyn Error>> {
    let low = 0.0;
    let high = 7.0;
    let func = |x: f64| x.powi(2) - 1.0;
    let iMax = 100;
    let tol = 1E-5;

    let x = numerics::bisection(low, high, func, iMax, tol)?;

    println!("\nThe solution is {x:.4}");

    Ok(())
}