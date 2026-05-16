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
    And,
}
impl Special {
    pub const ALL: [Special; 13] = [
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
        Special::And,
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
            Special::And => &["また"],
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