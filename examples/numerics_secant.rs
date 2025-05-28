#![allow(non_snake_case)]
use blog_codes::numerics;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let x0 = 3.5;
    let x_1 = 4.0;
    let func = |x: f64| x.powi(2) - 2.0;
    let iMax = 100;
    let tol = 1E-5;

    let x = numerics::secant(x0, x_1, func, iMax, tol)?;

    println!("\nThe solution is {x:.4}");

    Ok(())
}