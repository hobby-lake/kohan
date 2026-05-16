use crate::jp_parser::encode::enums::{
    Calculator,
    Bracket
};

pub fn normalize(mut array: Vec<String>) -> Vec<String> {
    println!("[LOG] Normalizing...");
    let mut temp = vec![];

    for mut elm in array {
        loop {
            let mut changed = false;

            // --- Calculator ---
            for kind in Calculator::ALL {
                for w in kind.words() {
                    if elm.contains(w) {
                        elm = elm.replace(w, kind.symbol());
                        changed = true;
                    }
                }
            }

            // --- Bracket ---
            for kind in Bracket::ALL {
                for w in kind.words() {
                    if elm.contains(w) {
                        elm = elm.replace(w, kind.symbol());
                        changed = true;
                    }
                }
            }

            if !changed {
                break;
            }
        }

        temp.push(elm);
    }

    temp
}