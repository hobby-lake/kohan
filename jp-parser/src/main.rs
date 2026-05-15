// src/main.rs
mod syntax;// 品詞分解
mod encode;// 符号化
mod meta;// 各セクション識別データ

use std::fs;
use crate::encode::encode;

fn main() {
    println!("[LOG] Queue started");
    let source_code = fs::read_to_string(r"D:/.prj/Language/kohan/test_file/test.k");

    let results = syntax::koubun_kaiseki(source_code.unwrap());
    let unwrapped = results.as_ref().unwrap();

    for result in unwrapped {
        println!("{}", result);
    }

    let analyzed: Vec<String> = unwrapped.clone();
    let raw_encoded = encode(analyzed);
    let mut result_str: String = String::new();
    let mut encoded = raw_encoded.unwrap();
    let id = encoded.remove(0);
    println!("{}", id);
    for result in encoded {
        //println!("{}", result);
        result_str = format!("{}{}", result_str, result.as_str()); 
    }
    println!("{}", result_str);
    println!("[LOG] Queue ended");
}
