use csv::WriterBuilder;
use std::error::Error;

#[derive(serde::Serialize)]
struct Record {
    iteration: usize,
    low: f64,
    mid: f64,
    high: f64,
    error: f64,
}

pub fn bisection<F: Fn(f64) -> f64>(
    low: f64,
    high: f64,
    func: F,
    iMax: usize,
    tol: f64,
) -> Result<f64, Box<dyn Error>> {
    //! Find the solution in range of [low, hight] through Bisection method.
    //! The results of iteration, low, mid, high and error will be exported to csv
    //! in the directory of examples/output/bisctions.csv
    let output_path = std::env::current_dir()?.join("examples/output/bisection.csv");

    let mut low = low;
    let mut high = high;
    let mut mid = 0.0;

    let mut iter = 0;
    let mut y = f64::MAX;

    // writer to export result to csv file
    let mut writer = WriterBuilder::new().from_path(output_path)?;

    while y.abs() > tol && iter < iMax {
        mid = 0.5 * (low + high);
        y = func(mid);

        if func(low) * y < 0.0 {
            high = mid;
        }

        if func(high) * y < 0.0 {
            low = mid;
        }

        // write a record to csv file
        writer.serialize(Record {
            iteration: iter,
            low,
            high,
            mid,
            error: y.abs(),
        })?;

        iter += 1;
    }

    if iter == iMax {
        Err("Maximum iteration exceeded.")?
    }

    writer.flush()?;

    Ok(mid)
}
