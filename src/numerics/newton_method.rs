use csv::WriterBuilder;
use std::error::Error;

#[derive(serde::Serialize)]
struct Record {
    iteration: usize,
    x: f64,
    error: f64,
}

pub fn newton<F: Fn(f64) -> f64, G: Fn(f64) -> f64>(
    x0: f64,
    func: F,
    dev: G,
    iMax: usize,
    tol: f64,
) -> Result<f64, Box<dyn Error>> {
    //! Find the solution through the Newton's method with initail guess of x0.
    let output_path = std::env::current_dir()?.join("examples/output/newton.csv");

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

        // todo: when dev(x) == 0
        x = x - y / dev(x);
        y = func(x);

        iter += 1;
    }

    if iter == iMax {
        Err("Maximum iteration exceeded.")?
    }

    writer.flush()?;

    Ok(x)
}
