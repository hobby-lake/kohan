use std::{collections::HashMap, hash::Hash};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum CodeType {
    SyntaxError,
    None,
    StRelation, //関連性のはじめ
    CnRelation, //関連性の継ぎ目
    EdRelation, //関連性のおわり
    Define,
    If,
    Ok,
    Ng,
    Repeat
}

pub struct ElementAttribute {
    prev_is_define:bool,
    is_main_fn:bool,
    is_in_branch:bool,
}
impl ElementAttribute {
    pub fn init() -> ElementAttribute{
        ElementAttribute {
            prev_is_define: false,
            is_main_fn: false,
            is_in_branch: false,
        }
    }
}

pub fn execute(code_array: Vec<String>) -> HashMap<(usize, CodeType), String>{
    let mut code_hash: HashMap<(usize, CodeType), String> = HashMap::new();
    let mut status = ElementAttribute::init();

    for (index, code_line) in code_array.iter().enumerate() {
        let key = if code_line.contains(":def") {
            status.prev_is_define = true;
            (index, CodeType::Define)
        } else if code_line.contains("if:") {
            status.is_in_branch = true;
            (index, CodeType::If)
        } else if code_line.contains("ok:") {
            if status.is_in_branch == true {
                (index, CodeType::Ok)
            } else {
                (index, CodeType::SyntaxError)
            }
        } else if code_line.contains("ng:") {
            if status.is_in_branch == true {
                (index, CodeType::Ng)
            } else {
                (index, CodeType::SyntaxError)
            }
        } else if code_line.contains(":repeat") {
            status.is_in_branch = false;
            (index, CodeType::Repeat)
        } else if code_line.contains("[and]") {
            (index, CodeType::CnRelation)
        } else {
            status = ElementAttribute::init();
            (index, CodeType::None)
        };

        code_hash.insert(key, code_line.to_string());
    }

    // --- index でソートして視覚化 ---
    let mut items: Vec<_> = code_hash.iter().collect();
    items.sort_by_key(|((index, _), _)| *index);

    let max_len = items
        .iter()
        .map(|((_, codetype), _)| format!("{:?}", codetype).chars().count())
        .max()
        .unwrap_or(0);

    let width = max_len + 3;

    for ((index, codetype), value) in items {
        println!(
            "{:03} | {:<width$} | {}",
            index,
            format!("{:?}", codetype),
            value,
            width = width
        );
    }

    code_hash
}