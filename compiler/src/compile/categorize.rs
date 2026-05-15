use std::{collections::HashMap, hash::Hash};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum CodeType {
    None,
    Define,
    If,
    For
}

pub fn execute(code_array: Vec<String>) {
    let mut code_hash: HashMap<(usize, CodeType), String> = HashMap::new();

    for (index, code_line) in code_array.iter().enumerate() {
        let key = if code_line.contains("!") {
            (index, CodeType::Define)
        } else if code_line.contains("if") {
            (index, CodeType::If)
        } else {
            (index, CodeType::None)
        };

        code_hash.insert(key, code_line.to_string());
    }

    // --- index でソートして視覚化 ---
    let mut items: Vec<_> = code_hash.iter().collect();
    items.sort_by_key(|((index, _), _)| *index);

    for ((index, codetype), value) in items {
        println!("{} | {:?} | {}", index, codetype, value);
    }
}