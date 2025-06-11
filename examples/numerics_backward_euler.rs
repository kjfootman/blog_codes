#![allow(non_snake_case)]
use blog_codes::numerics::{self, Input};
use std::{error::Error, fs::File};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("examples/input/backward_euler.yaml")?;
    let input: Input = serde_yaml::from_reader(file)?;

    numerics::euler_backward(input)?;

    Ok(())
}
