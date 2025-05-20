#![allow(non_snake_case)]
use blog_codes::numerics;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let x0 = 3.5;
    let func = |x: f64| x.powi(2) - 2.0;
    let dev = |x: f64| 2.0 * x;
    let iMax = 100;
    let tol = 1E-5;

    let x = numerics::newton(x0, func, dev, iMax, tol)?;
    println!("{x}");

    Ok(())
}
