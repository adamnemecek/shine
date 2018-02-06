use syn;
use quote;
use std::path::*;

/// Return (relative) path to the source file of the calling site.
pub fn find_source_file() -> PathBuf {
    use proc_macro;

    let source_file = proc_macro::Span::call_site().source_file();
    if !source_file.is_real() {
        panic!("This derive macro can be used from real files only, no macro or other dark magic, sry.");
    }

    let source_file = PathBuf::from(format!("{}", source_file.path()));
    if !source_file.exists() {
        panic!("Could not find source path: {:?}", source_file);
    }
    source_file
}


/// Return (relative) path to the directory of source file of the calling site.
pub fn find_source_dir() -> PathBuf {
    PathBuf::from(find_source_file().as_path().parent().unwrap())
}


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
