use csv::WriterBuilder;
use std::{collections::HashMap, error::Error};

#[derive(serde::Serialize)]
struct Record {
    t: f64,
    x: f64,
    x_exact: f64,
    error: f64,
}

pub fn euler_backward(
    constant: HashMap<&str, f64>,
    dx0: f64,
    x0: f64,
) -> Result<Vec<f64>, Box<dyn Error>> {
    //! find solution of mx'' + cx' + kx = 0
    let output_path = std::env::current_dir()?.join("examples/output/backward_euler.csv");

    let m = constant["m"];
    let c = constant["c"];
    let k = constant["k"];

    let tmax = 10.0;
    let np = 101;
    let dt = tmax / (np - 1) as f64;

    let mut M = [[0.0, 0.0], [0.0, 0.0]];
    let mut dx0 = dx0;
    let mut x0 = x0;
    let mut dx = dx0;
    let mut x = x0;
    // let mut t = 0.0;

    // writer to export result to csv file
    let mut writer = WriterBuilder::new().from_path(output_path)?;

    M[0][0] = 1.0 + c * dt / m;
    M[0][1] = k * dt / m;
    M[1][0] = -dt;
    M[1][1] = 1.0;

    let det = M[0][0] * M[1][1] - M[0][1] * M[1][0];
    let t = (0..np).map(|i| i as f64 * dt).collect::<Vec<_>>();
    let exact = exact_solution(constant, &t);

    for (t, x_exact) in t.iter().zip(exact) {
        // write a record to csv file
        writer.serialize(Record {
            t: *t,
            x,
            x_exact,
            error: x - x_exact,
        })?;

        dx = (M[1][1] * dx0 - M[1][0] * x0) / det;
        x = (-M[0][1] * dx0 + M[0][0] * x0) / det;

        dx0 = dx;
        x0 = x;
    }

    Ok(vec![0.0])
}

fn exact_solution(constant: HashMap<&str, f64>, t: &[f64]) -> Vec<f64> {
    //! Returns the exact solution corresponding to t.
    //! x(0) = 1.0
    //! x'(0) = 0.0

    let m = constant["m"];
    let c = constant["c"];
    let k = constant["k"];
    let det = c.powi(2) - 4.0 * m * k;

    if det > 0.0 {
        println!("over damping");

        let lamb1 = 0.5 * (-c + (c.powi(2) - 4.0 * m * k).sqrt()) / m;
        let lamb2 = 0.5 * (-c - (c.powi(2) - 4.0 * m * k).sqrt()) / m;
        let c1 = -lamb2 / (lamb1 - lamb2);
        let c2 = lamb1 / (lamb1 - lamb2);

        return t
            .iter()
            .map(|t| c1 * (lamb1 * t).exp() + c2 * (lamb2 * t).exp())
            .collect::<Vec<_>>();
    }

    if det == 0.0 {
        // let lamb = 0.5 * -c / m;
        // let c1 = -lamb2 / (lamb1 - lamb2);
        // let c2 = lamb1 / (lamb1 - lamb2);
        // println!("critical damping");
        unimplemented!();
        // return Vec::new();
    }

    if det < 0.0 {
        println!("under damping");

        let alpha = -c / (2.0 * m);
        let beta = (4.0 * m * k - c.powi(2)).sqrt() / (2.0 * m);
        let c1 = 1.0;
        let c2 = -alpha / beta;

        return t
            .iter()
            .map(|t| (alpha * t).exp() * (c1 * (beta * t).cos() + c2 * (beta * t).sin()))
            .collect::<Vec<_>>();
    }

    Vec::new()
}
