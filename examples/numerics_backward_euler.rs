#![allow(non_snake_case)]
use blog_codes::numerics;
use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut constant = HashMap::new();
    constant.insert("m", 1.0);
    constant.insert("c", 1.0);
    constant.insert("k", 1.0);

    numerics::euler_backward(constant, 0.0, 1.0);

    Ok(())
}
