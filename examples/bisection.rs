#![allow(non_snake_case)]
use std::error::Error;

pub fn bisection<F: Fn(f64) -> f64>(
    low: f64,
    high: f64,
    func: F,
    iMax: usize,
    tol: f64,
) -> Result<f64, Box<dyn Error>> {
    //! Find the solution in range of [low, hight] through Bisection method.
    let mut low = low;
    let mut high = high;
    let mut mid = 0.0;

    let mut iter = 0;
    let mut y = f64::MAX;

    while y.abs() > tol && iter < iMax {
        mid = 0.5 * (low + high);
        y = func(mid);

        if func(low) * y < 0.0 {
            high = mid;
        }

        if func(high) * y < 0.0 {
            low = mid;
        }

        println!("i = {iter:>2.0}  mid = {mid:<5.4}  f(mid) = {:>7.4}  residual = {:.2E}", y, y.abs());

        iter += 1;
    }

    if iter == iMax {
        Err("Maximum iteration exceeded.")?
    }

    Ok(mid)
}

fn main() -> Result<(), Box<dyn Error>> {
    let low = 0.0;
    let high = 7.0;
    let func = |x: f64| x.powi(2) - 1.0;
    let iMax = 100;
    let tol = 1E-5;

    let x = bisection(low, high, func, iMax, tol)?;

    println!("\nThe solution is {x:.4}");

    Ok(())
}