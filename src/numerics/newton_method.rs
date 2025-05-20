use std::error::Error;

pub fn newton<F: Fn(f64) -> f64, G: Fn(f64) -> f64>(
    x0: f64,
    func: F,
    dev: G,
    iMax: usize,
    tol: f64,
) -> Result<f64, Box<dyn Error>> {
    let mut iter = 0;
    let mut x = x0;
    let mut y = func(x);

    while y.abs() > tol && iter <= iMax {
        // todo: when dev(x) == 0
        x = x - y / dev(x);
        y = func(x);

        iter += 1;
    }

    if iter == iMax {
        Err("Maximum iteration exceeded.")?
    }

    Ok(x)
}
