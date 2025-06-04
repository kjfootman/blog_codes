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
    x_1: f64,
    func: F,
    iMax: usize,
    tol: f64
) -> Result<f64, Box<dyn Error>> {
    //! Find the solution through the secant method with initail value of x0 and x_1.
    let output_path = std::env::current_dir()?.join("examples/output/secant.csv");

    let mut iter = 0;
    let mut x = x0;
    let mut x0 = x0;
    let mut x_1 = x_1;
    let mut y0 = func(x0);
    let mut y_1 = func(x_1);

    // writer to export result to csv file
    let mut writer = WriterBuilder::new().from_path(output_path)?;

    // write a initial value to csv file
    writer.serialize(Record {
        iteration: iter,
        x,
        error: y0.abs(),
    })?;

    while y0.abs() > tol && iter <= iMax {
        x = x0 - y0 * (x0 - x_1) / (y0 - y_1);

        x_1 = x0;
        y_1 = y0;
        x0 = x;
        y0 = func(x);

        iter += 1;

        // write a record to csv file
        writer.serialize(Record {
            iteration: iter,
            x,
            error: y0.abs(),
        })?;
    }

    if iter == iMax {
        Err("Maximum iteration exceeded.")?
    }

    Ok(x)
}