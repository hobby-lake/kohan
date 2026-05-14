// src/main.rs

use std::fs;

mod syntax;

fn main() {
    println!("[LOG] Queue started");
    let source_code = fs::read_to_string(r"D:/.prj/Language/kohan/compiler_ts/test_file/test.k");
    let results = syntax::koubun_kaiseki(source_code.unwrap());
    for result in results.unwrap() {
        println!("{}", result);
    }
    println!("[LOG] Queue ended");
}
