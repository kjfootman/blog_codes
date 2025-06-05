use std::collections::HashMap;

pub fn euler_backward(constant: HashMap<&str, f64>, dx0: f64, x0: f64) {
    //! find solution of mx'' + cx' + kx = 0
    //!

    let m = constant["m"];
    let c = constant["c"];
    let k = constant["k"];
    let dt = 0.1;

    let mut M = [[0.0, 0.0], [0.0, 0.0]];
    let mut dx0 = dx0;
    let mut x0 = x0;
    let mut dx = dx0;
    let mut x = x0;

    M[0][0] = 1.0 + c * dt / m;
    M[0][1] = k * dt / m;
    M[1][0] = -dt;
    M[1][1] = 1.0;

    let det = M[0][0] * M[1][1] - M[0][1] * M[1][0];

    for i in 0..100 {
        dx = (M[1][1] * dx0 - M[1][0] * x0) / det;
        x = (-M[0][1] * dx0 + M[0][0] * x0) / det;

        dx0 = dx;
        x0 = x;

        println!("{i} dx: {dx:.4} x: {x:.4}");
    }

}
