use csv::WriterBuilder;
use std::error::Error;

#[derive(serde::Serialize)]
struct Record {
    iteration: usize,
    x: f64,
    error: f64,
}

pub fn secant<F: Fn(f64) -> f64>(
    x0: f64,
    x1: f64,
    func: F,
    iMax: usize,
    tol: f64
) -> Result<f64, Box<dyn Error>> {
    //! Find the solution through the secant method with initail value of x0 and x_1.
    let output_path = std::env::current_dir()?.join("examples/output/secant.csv");
    
    let mut iter = 0;
    let mut x = x0;
    let mut y = func(x);

    // writer to export result to csv file
    let mut writer = WriterBuilder::new().from_path(output_path)?;

    while y.abs() > tol && iter <= iMax {
        // write a record to csv file
        writer.serialize(Record {
            iteration: iter,
            x,
            error: y.abs(),
        })?;

        // // todo: when dev(x) == 0
        // x = x - y / d_func(x);
        // y = func(x);

        iter += 1;
    }

    if iter == iMax {
        Err("Maximum iteration exceeded.")?
    }

    Ok(0.0)
}