#!/bin/bash
BASEDIR=$(dirname $0)

cd ${BASEDIR}
cargo build -r --examples

# run the example of numerics_bisection.rs
target/release/examples/numerics_bisection

# run the example of numerics_newton.rs
target/release/examples/numerics_newton

# run the example of numberics_secant.rs
target/release/examples/numerics_secant

# run the example of backward_euler.rs
target/release/examples/numerics_backward_euler