use csv::WriterBuilder;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct Record {
    t: f64,
    x: f64,
    x_exact: f64,
    error: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    pub m: f64,
    pub c: f64,
    pub k: f64,
    pub tMax: f64,
    pub np: usize,
    pub x0: f64,
    pub dx0: f64,
    pub outpath: String,
}

pub fn euler_backward(input: Input) -> Result<(), Box<dyn Error>> {
    //! find solution of mx'' + cx' + kx = 0
    let output_path = std::env::current_dir()?.join(&input.outpath);

    let m = input.m;
    let c = input.c;
    let k = input.k;

    let tmax = input.tMax;
    let np = input.np;
    let dt = tmax / (np - 1) as f64;

    // inverse matrix of coefficient matrix
    let mut M = [[0.0, 0.0], [0.0, 0.0]];
    let mut dx0 = input.dx0;
    let mut x0 = input.x0;
    let mut dx;
    let mut x = x0;

    // writer to export result to csv file
    let mut writer = WriterBuilder::new().from_path(output_path)?;

    // inverse of determinant
    let det = m / (m + c * dt + k * dt.powi(2));

    M[0][0] = det;
    M[0][1] = -det * k * dt / m;
    M[1][0] = det * dt;
    M[1][1] = det * (1.0 + c * dt / m);

    let t = (0..np).map(|i| i as f64 * dt).collect::<Vec<_>>();
    let exact = exact_solution(&input, &t);

    for (t, x_exact) in t.iter().zip(exact) {
        // write a record to csv file
        writer.serialize(Record {
            t: *t,
            x,
            x_exact,
            error: x - x_exact,
        })?;

        dx = M[0][0] * dx0 + M[0][1] * x0;
        x = M[1][0] * dx0 + M[1][1] * x0;

        dx0 = dx;
        x0 = x;
    }

    Ok(())
}

fn exact_solution(input: &Input, t: &[f64]) -> Vec<f64> {
    //! Returns the exact solution corresponding to t.

    let m = input.m;
    let c = input.c;
    let k = input.k;
    let det = c.powi(2) - 4.0 * m * k;

    // over damping
    if det > 0.0 {
        let lamb1 = 0.5 * (-c + (c.powi(2) - 4.0 * m * k).sqrt()) / m;
        let lamb2 = 0.5 * (-c - (c.powi(2) - 4.0 * m * k).sqrt()) / m;
        let c1 = (input.dx0 - lamb2 * input.x0) / (lamb1 - lamb2);
        let c2 = (lamb1 * input.x0 - input.dx0) / (lamb1 - lamb2);

        return t
            .iter()
            .map(|t| c1 * (lamb1 * t).exp() + c2 * (lamb2 * t).exp())
            .collect::<Vec<_>>();
    }

    // critical damping
    if det == 0.0 {
        let lamb = -c / (2.0 * m);
        let c1 = input.x0;
        let c2 = -lamb * input.x0 + input.dx0;

        return t
            .iter()
            .map(|t| c1 * (lamb * t).exp() + c2 * t * (lamb * t).exp())
            .collect::<Vec<_>>();
    }

    // under damping
    if det < 0.0 {
        let alpha = -c / (2.0 * m);
        let beta = (4.0 * m * k - c.powi(2)).sqrt() / (2.0 * m);
        let c1 = input.x0;
        let c2 = (input.dx0 - alpha * input.x0) / beta;

        return t
            .iter()
            .map(|t| (alpha * t).exp() * (c1 * (beta * t).cos() + c2 * (beta * t).sin()))
            .collect::<Vec<_>>();
    }

    Vec::new()
}
