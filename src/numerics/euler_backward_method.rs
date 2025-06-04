use std::collections::HashMap;

pub fn euler_backward(constant: HashMap<&str, f64>, d_x0: f64, x0: f64) {
    //! find solution of mx'' + cx' + kx = 0
    //!

    let d2_x = |d_x: f64, x: f64| -(constant["c"] * d_x + constant["k"]) / constant["m"];

    println!("{}", d2_x(d_x0, x0));
}
