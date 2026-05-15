// src/main.rs
pub mod syntax;// 品詞分解
pub mod encode;// 符号化
pub mod meta;// 各セクション識別データ

use std::fs;
use encode::encode;

pub fn parse(file_path:&'static str) -> String {
    println!("[LOG] Parsing queue started");
    let source_code = fs::read_to_string(file_path);

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
    println!("[LOG] Parsing queue ended");

    result_str
}
