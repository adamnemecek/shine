use syn;
use quote;
use std::path::*;


pub fn convert_snake_to_camel_case(intput: &str) -> String {
    let mut is_snake = true;
    let mut result = String::new();
    result.reserve(intput.len());

    for c in intput.chars() {
        match c {
            '_' if !is_snake => {
                is_snake = true;
            }

            c => {
                if is_snake {
                    for u in c.to_uppercase() {
                        result.push(u);
                    }
                } else {
                    result.push(c);
                }
                is_snake = false;
            }
        }
    }
    result
}


pub fn convert_snake_to_capital_case(intput: &str) -> String {
    let mut result = String::new();
    result.reserve(intput.len());

    for c in intput.chars() {
        match c {
            '_' => { /*consume*/ }

            c => {
                for u in c.to_uppercase() {
                    result.push(u);
                }
            }
        }
    }
    result
}


pub fn convert_camel_to_snake_case(intput: &str) -> String {
    let mut result = String::new();
    result.reserve(intput.len());

    for c in intput.chars() {
        if c.is_uppercase() {
            if !result.is_empty() {
                result.push('_');
            }
            for u in c.to_lowercase() {
                result.push(u);
            }
        } else {
            result.push(c);
        }
    }
    result
}
