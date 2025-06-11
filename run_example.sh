#!/bin/bash
BASEDIR=$(dirname $0)

cd ${BASEDIR}
cargo build -r --examples

# # run the example of numerics_bisection.rs
# cargo run -r --example numerics_bisection
target/release/examples/numerics_bisection

# # run the example of numerics_newton.rs
# cargo run -r --example numerics_newton
target/release/examples/numerics_newton

# # run the example of numberics_secant.rs
# cargo run -r --example numerics_secant
target/release/examples/numerics_secant

# # run the example of backward_euler.rs
# cargo run -r --example numerics_backward_euler
target/release/examples/numerics_backward_euler