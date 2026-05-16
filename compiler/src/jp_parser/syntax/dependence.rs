use crate::jp_parser::{
    syntax::{
        essence::KihonYouso, part::Bunkugiri
    }
};


/// 基本型・基本要素
pub enum Fuzokugo {
    Ha, // 定義
    Wo, // 指名
    No, // 関連
    Ga, // 確認
    To, // 並列
    De, // 
    Suru, // 動作
}

impl Fuzokugo {
    pub fn from_str(s: &str) -> Option<Self> {
        match KihonYouso::from_str(s) {
            None => {
                match s {
                    "は" => Some(Self::Ha),
                    "を" => Some(Self::Wo),
                    "の" => Some(Self::No),
                    "が" => Some(Self::Ga),
                    "と" => Some(Self::To),
                    "で" => Some(Self::De),
                    _ => None,
                }
            },
            _ => None
        }
    }
    
    pub fn all_words() -> &'static [&'static str] {
        &["は", "を", "の", "が", "と", "で"]
    }
}

trait FuzokugoTyuusyutu {
    fn fuzokugo_tyuusyutsu(&self) -> Vec<Fuzokugo>;
}

impl FuzokugoTyuusyutu for str {
    fn fuzokugo_tyuusyutsu(&self) -> Vec<Fuzokugo> {
        self.split_whitespace()
            .filter_map(|w| Fuzokugo::from_str(w))
            .collect()
    }
}

/// 位置を検出
fn youso_ichi(text: &str) -> Vec<(usize, &'static str)> {
    let mut result = Vec::new();

    for &word in Fuzokugo::all_words() {
        let mut start = 0;
        while let Some(pos) = text[start..].find(word) {
            let abs = start + pos;
            result.push((abs, word));
            start = abs + word.len();
        }
    }

    // 出現順にソート
    result.sort_by_key(|x| x.0);

    result
}

pub fn fuzoku_bunkai(text: &str) -> Vec<String> {
    let positions = youso_ichi(text);

    if positions.is_empty() {
        return vec![text.to_string()];
    }

    let mut result = Vec::new();
    let mut last = 0;

    for (pos, word) in positions {
        if last < pos {
            result.push(text[last..pos].to_string());
        }
        result.push(word.to_string());
        last = pos + word.len();
    }

    if last < text.len() {
        result.push(text[last..].to_string());
    }

    result
}

// 構文から要素の配列へ
pub fn bunkai(rons: &mut Vec<String>) -> Vec<String> {
    let mut m_rons = rons;
    let mut temp: Vec<String> = vec![];
    for ron in m_rons {
        let ress = fuzoku_bunkai(&ron.to_string());
        for res in ress {
            temp.push(res);
        }
    }
    temp
}

