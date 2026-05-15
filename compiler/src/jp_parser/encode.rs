use crate::jp_parser::meta::{ENCODED, FUZOKU};

// To-Do いくつかの関数に分割すること。ながい。
pub fn encode(array: Vec<String>) -> Result<Vec<String>, &'static str> {
    let mut m_array = array;
    let id = m_array.remove(0);
    if id == String::from(FUZOKU) {
        let mut temp: Vec<String> = vec![String::from(ENCODED)];

        let mut is_definer = false;
        let mut is_value = false;
        let mut is_grouped = false;
        let mut is_parallel = false;
        let mut is_condition = false;
        let mut is_action = false;
        let mut is_denied = false;
        let mut is_refered = false;

        for element in m_array {
            let elm: String;
            if is_definer == true && is_value == false {
                temp.push(element);
                is_value = true;
                is_grouped = false;
                continue;
            } else if ((((is_definer && is_value) || is_parallel) == true) && (is_grouped == false)) == true {
                elm = String::from(format!("{} {}", element.as_str(), ")"));
                is_grouped = true;
                is_parallel = false;
            } else if (element == "") | (element == "。") | (element == "、") {
                temp.push(element);
                continue;
            } else {
                match element.as_str() {
                    "は" => {
                        elm = String::from("=");
                        is_definer = true;
                    },
                    "もし" | "もしも" => {
                        elm = String::from("if(");
                        is_condition = true;
                    },
                    "である" | "であり" | "なら" | "でない" => {
                        if is_condition == true {
                            match element.as_str() {
                                "である" => {
                                    elm = String::from("|");
                                    is_denied = false;
                                },
                                "でない" => {
                                    elm = String::from("|");
                                    is_denied = true;
                                },
                                "なら" => {
                                    if is_denied == true {
                                        elm = String::from("ng:");
                                        is_denied = false;
                                    } else {
                                        elm = String::from("ok:");
                                    }
                                    is_action = true;
                                },
                                _ => elm = String::from(element),
                            }
                        } else if is_definer == true {
                            match element.as_str() {
                                "である" | "であり" => {
                                    elm = String::from("!");
                                    is_definer = false;
                                    is_value = false;
                                },
                                _ => elm = String::from(element),
                            }
                        } else {
                            elm = String::from(element)
                        }
                    },
                    "を" => {
                        elm = String::from("->");
                        is_action = true;
                    },
                    "する" | "して" => {
                        if is_action == true {
                            elm = String::from(".");
                            is_action = false;
                        } else {
                            elm = String::from(element)
                        }
                    },
                    "と" => {
                        elm = String::from("&");
                        is_parallel = true;
                    },
                    "の" => {
                        elm = String::from("=>");
                        is_refered = true;
                    },
                    "和" | "差" => {
                        if is_refered == true {
                            match element.as_str() {
                                "和" => elm = String::from("SUM"),
                                "差" => elm = String::from("DIF"),
                                _ => elm = String::from(element),
                            }
                        } else {
                            elm = String::from(element);
                        }
                    },
                    // if(x&y)=>SUM-is-larger-than-5|ok:...
                    "以上" | "以下" | "未満" | "より大きい" | "より小さい" | "と等しい" => {
                        match element.as_str() {
                            "以上" => elm = String::from("[=<]"),
                            "以下" => elm = String::from("[>=]"),
                            "より大きい" => elm = String::from("[<]"),
                            "より小さい" | "未満" => elm = String::from("[>]"),
                            "と等しい" => elm = String::from("[==]"),
                            _ => elm = String::from(element),
                        }
                    },
                    "が" => elm = String::from("-is"),
                    _ => {
                        is_value = true;
                        let mut prefix = "(";
                        if (is_parallel || is_action || is_condition) == true {
                            prefix = "";
                        }
                        temp.push(String::from(format!("{}{}", prefix, element.as_str())));
                        continue;
                    }
                }
                is_grouped = false;
            }
            temp.push(elm);
            
        }
        Ok(temp)
    } else {
        Err("[ERR] 品詞分解が完了していません。")
    }
}