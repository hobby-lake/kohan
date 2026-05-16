mod enums;
mod utils;

use crate::jp_parser::meta::{ENCODED, PARSED};
use enums::{
    Special,
    Comparator,
    Unit,
};
use utils::normalize;

pub struct SegmentAttribute {
    will:bool,
    pending:bool,
    done:bool,
}
impl SegmentAttribute {}

pub fn encode(array: Vec<String>) -> Vec<String> {
    println!("[LOG] Encoding...");
    let mut m_array = normalize(array);
    let mut temp:Vec<String> = vec![];

    let mut denied:bool = false;
    let mut doubled_action:bool = false;
    let mut define:SegmentAttribute = SegmentAttribute{ will:false, pending:false, done:false};
    let mut branche:SegmentAttribute = SegmentAttribute{ will:false, pending:false, done:false};

    for element in m_array {
        let word = element.as_str();
        let mut elm = element.clone(); 

        // --- Special ---
        if let Some(kind) = Special::ALL
            .iter()
            .find(|sp| sp.words().contains(&word))
        {
            match kind {
                Special::If => {
                    elm = String::from("if:");
                    branche.will = true;
                }
                Special::Repeat => {
                    elm = String::from(":repeat");
                }
                Special::Log => {
                    elm = String::from("Log");
                }
                Special::Is => {
                    elm = String::from("=");
                    define.will = true;
                }
                Special::Infer => {
                    if define.will == true {
                        elm = String::from(":def");
                        define.done = true;
                        define.will = false;
                    } else if branche.will == true {
                        elm = String::from("|");
                        branche.will = false;
                        branche.pending = true;
                    } else if branche.pending == true {
                        elm = String::from("");
                    }
                }
                Special::Check => {
                    if branche.pending == true {
                        match denied {
                            true => elm = String::from("ng:"),
                            false => elm = String::from("ok:"),
                        }
                        branche.pending = false;
                        branche.done = true;
                    }
                }
                Special::Not => {
                    if branche.pending == true {
                        elm = String::from("");
                        denied = true;
                    }
                }
                Special::Affirm => {
                    if define.done == true {
                        elm = String::from("");
                        define.done = false;
                    } else if branche.pending == true {
                        elm = String::from("");
                        denied = false;
                    }
                }
                Special::Camma => {
                    if branche.done == true {
                        elm = String::from("|");
                        branche.pending = true;
                        branche.done = false;
                        doubled_action = false;
                    }
                }
                Special::Pereod => {
                    branche.done = false;
                    doubled_action = false;
                }
                Special::Act => {
                    if doubled_action == true {
                        elm = String::from("|->");
                        doubled_action = false;
                    } else if branche.done == true {
                        elm = String::from("->");
                        doubled_action = true;
                    }
                }
                Special::Set => {
                    if branche.will == true {
                        elm = String::from("[");
                    }
                }
                Special::And => {
                    elm = String::from("[and]");
                }
                _ => {}
            }
            temp.push(elm);
            continue;
        }

        // --- Comparator ---
        if let Some(kind) = Comparator::ALL
            .iter()
            .find(|cmp| cmp.words().contains(&word))
        {
            match kind {
                Comparator::OrMore => elm = String::from("<=]"),
                Comparator::OrLess => elm = String::from(">=]"),
                Comparator::More => elm = String::from("<]"),
                Comparator::Less => elm = String::from(">]"),
                Comparator::Equal => elm = String::from("==]"),
            }
            temp.push(elm);
            continue;
        }

        // --- Unit ---
        if let Some(kind) = Unit::ALL
            .iter()
            .find(|cmp| cmp.words().contains(&word))
        {
            match kind {
                Unit::Times => elm = String::from(":times"),
            }
            temp.push(elm);
            continue;
        }

        // --- その他 ---
        temp.push(elm);
    }
    println!("[LOG] Done");
    temp
}