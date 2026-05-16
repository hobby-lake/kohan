// src/main.rs
pub mod syntax;// 品詞分解
pub mod encode;// 符号化
pub mod meta;// 各セクション識別データ

use std::fs;
use encode::encode;

pub fn to_halfsize(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            // 全角英字（Ａ〜Ｚ, ａ〜ｚ）
            'Ａ'..='Ｚ' | 'ａ'..='ｚ' => ((c as u32) - 0xFEE0) as u8 as char,

            // 全角数字（０〜９）
            '０'..='９' => ((c as u32) - 0xFEE0) as u8 as char,

            _ => c,
        })
        .collect::<String>()
        .to_ascii_lowercase()
}

pub fn parse(file_path:&'static str) -> String {
    println!("[LOG] Parsing queue started");
    let source_code = fs::read_to_string(file_path);
    let normalized_code = to_halfsize(&source_code.unwrap());

    let results = syntax::koubun_kaiseki(normalized_code);

    for result in &results {
        println!("{}", result);
    }

    let analyzed: Vec<String> = results.clone();
    let encoded = encode(analyzed);
    let mut result_str: String = String::new();
    for result in encoded {
        //println!("{}", result);
        result_str = format!("{}{}", result_str, result.as_str()); 
    }
    println!("{}", result_str);
    println!("[LOG] Parsing queue ended");

    result_str
}
