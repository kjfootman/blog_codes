#!/bin/bash
BASEDIR=$(dirname $0)

cd ${BASEDIR}

# run the example of bisection.rs
#cargo run -r --example numerics_bisection

# run the example of bisection.rs
cargo run -r --example numerics_newton
