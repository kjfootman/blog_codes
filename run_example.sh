#!/bin/bash
BASEDIR=$(dirname $0)

cd ${BASEDIR}

# run the example of numerics_bisection.rs
cargo run -r --example numerics_bisection

# run the example of numerics_newton.rs
cargo run -r --example numerics_newton

# run the example of numberics_secant.rs
cargo run -r --example numerics_secant

# run the example of backward_euler.rs
cargo run -r --example numerics_backward_euler
