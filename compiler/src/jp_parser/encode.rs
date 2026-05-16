
use crate::jp_parser::meta::{ENCODED, PARSED};

#[derive(Copy, Clone)]
pub enum Special {
    If,
    Repeat,
    Log,
    Is,
    Infer, //推論
    Check,
    Not,
    Affirm,
    Camma,
    Pereod,
    Act,
    Set, //指定
}
impl Special {
    pub const ALL: [Special; 12] = [
        Special::If,
        Special::Repeat,
        Special::Log,
        Special::Is,
        Special::Infer,
        Special::Check,
        Special::Not,
        Special::Affirm,
        Special::Camma,
        Special::Pereod,
        Special::Act,
        Special::Set,
    ];

    pub fn words(&self) -> &'static [&'static str] {
        match self {
            Special::If => &["もしも", "もし"],
            Special::Repeat => &["繰り返す", "繰り返し"],
            Special::Log => &["記録"],
            Special::Is => &["は"],
            Special::Infer => &["で"],
            Special::Check => &["なら"],
            Special::Not => &["ない"],
            Special::Affirm => &["ある", "あり"],
            Special::Camma => &["、"],
            Special::Pereod => &["。"],
            Special::Act => &["を"],
            Special::Set => &["が"],
        }
    }
}

#[derive(Copy, Clone)]
pub enum Calculator{
    Add,
    Sub,
    Mul,
    Div,
}
impl Calculator {
    pub const ALL: [Calculator; 4] = [
        Calculator::Add,
        Calculator::Sub,
        Calculator::Mul,
        Calculator::Div,
    ];

    pub fn words(&self) -> &'static [&'static str] {
        match self {
            Calculator::Add => &["足す", "たす", "＋"],
            Calculator::Sub => &["引く", "ひく", "ー"],
            Calculator::Mul => &["掛ける", "かける", "＊"],
            Calculator::Div => &["割る", "わる", "÷"],
        }
    }

    pub fn matches(&self, word: &str) -> bool {
        self.words().iter().any(|w| word.contains(w))
    }

    pub fn detect(word: &str) -> Option<Calculator> {
        Calculator::ALL
            .iter()
            .find(|op| op.matches(word))
            .copied()
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Calculator::Add => "+",
            Calculator::Sub => "-",
            Calculator::Mul => "*",
            Calculator::Div => "/",
        }
    }
}

#[derive(Copy, Clone)]
pub enum Comparator{
    OrMore,
    OrLess,
    More,
    Less,
    Equal,
}
impl Comparator {
    pub const ALL: [Comparator; 5] = [
        Comparator::OrMore,
        Comparator::OrLess,
        Comparator::More,
        Comparator::Less,
        Comparator::Equal,
    ];

    pub fn words(&self) -> &'static [&'static str] {
        match self {
            Comparator::OrMore => &["以上"],
            Comparator::OrLess => &["以下"],
            Comparator::More => &["より大きい"],
            Comparator::Less => &["より小さい"],
            Comparator::Equal => &["等しい"],
        }
    }
}

#[derive(Copy, Clone)]
pub enum Unit{
    Times,
}
impl Unit {
    pub const ALL: [Unit; 1] = [
        Unit::Times,
    ];

    pub fn words(&self) -> &'static [&'static str] {
        match self {
            Unit::Times => &["回"],
        }
    }
}

#[derive(Copy, Clone)]
pub enum Bracket{
    StSmall,
    StMiddle,
    StLarge,
    EdSmall,
    EdMiddle,
    EdLarge,
}
impl Bracket {
    pub const ALL: [Bracket; 6] = [
        Bracket::StSmall,
        Bracket::StMiddle,
        Bracket::StLarge,
        Bracket::EdSmall,
        Bracket::EdMiddle,
        Bracket::EdLarge,
    ];

    pub fn words(&self) -> &'static [&'static str] {
        match self {
            Bracket::StSmall => &["（"],
            Bracket::StMiddle => &["｛"],
            Bracket::StLarge => &["「"],
            Bracket::EdSmall => &["）"],
            Bracket::EdMiddle => &["｝"],
            Bracket::EdLarge => &["」"],
        }
    }

    pub fn matches(&self, word: &str) -> bool {
        self.words().iter().any(|w| word.contains(w))
    }

    pub fn detect(word: &str) -> Option<Calculator> {
        Calculator::ALL
            .iter()
            .find(|op| op.matches(word))
            .copied()
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Bracket::StSmall => "(",
            Bracket::StMiddle => "{",
            Bracket::StLarge => "[",
            Bracket::EdSmall => ")",
            Bracket::EdMiddle => "}",
            Bracket::EdLarge => "]",
        }
    }
}

pub struct SegmentAttribute {
    will:bool,
    pending:bool,
    done:bool,
}
impl SegmentAttribute {}

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
                _ => {}
            }
            temp.push(elm);
            continue;
        }

        // --- Calculator ---
        if let Some(kind) = Calculator::detect(word) {
            let mut elm = element.clone();
            for w in kind.words() {
                elm = elm.replace(w, kind.symbol());
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