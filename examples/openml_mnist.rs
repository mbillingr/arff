extern crate arff;
use std::fs;

fn main() {
    let contents = fs::read_to_string("mnist_784.arff").expect(
        "Data file not found. Please download mnist_784.arff from https://www.openml.org/d/554",
    );

    let data: Vec<Vec<f32>> = arff::from_str(&contents).unwrap();
    println!("{:?}", data);
}
