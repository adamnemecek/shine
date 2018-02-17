#![feature(slice_patterns)]
extern crate image;

use std::env;
use std::fs;
use std::io::prelude::*;


fn main() {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        &[_, ref input, ref output] => {
            let img = image::open(input).unwrap();
            let img = img.to_rgb();
            let mut file = fs::File::create(output).unwrap();
            let slice = &img;
            file.write_all(slice).unwrap();
        }

        _ => {
            println!("Wrong arguments: {:?}", args);
            println!("usage: {} input_image output_r8g8b8", args[0]);
        }
    }
}
