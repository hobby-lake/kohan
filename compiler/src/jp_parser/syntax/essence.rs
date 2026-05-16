use crate::jp_parser::{
    syntax::part::Bunkugiri
};

/// 基本型・基本要素
pub enum KihonYouso {
    Wa, // 和
    Sa, // 差
    Hazu, // はず(推論)
    Naru, // 成る(代入)
    Beki, // べき(強制)
    Aru, // である・であり(肯定)
    Moshi, // もし・もしも(仮定)
    Nara, // なら(仮定)
    Nai, // ない(否定)
    Hikaku, // 比較子
    Kurikaesu, // 繰り返す
}

impl KihonYouso {
    pub fn from_str(s: &str) -> Option<Self> {
        match  Bunkugiri::from_str(s) {
            None => match s {
                "はず" => Some(Self::Hazu),
                "成る" | "なる" => Some(Self::Naru),
                "べき" => Some(Self::Beki),
                "ある" | "あり" => Some(Self::Aru),
                "もし" | "もしも" => Some(Self::Moshi),
                "なら" => Some(Self::Nara),
                "ない" => Some(Self::Nai),
                "以上" | "以下" | "未満" | "より大きい" | "より小さい" | "等しい" => Some(Self::Hikaku),
                "繰り返す" | "繰り返し" => Some(Self::Kurikaesu),
                _ => None,
            },
            _ => None,
        }
        
    }
    
    pub fn all_words() -> &'static [&'static str] {
        &[
            "はず", 
            "成る", 
            "べき", 
            "ある", 
            "あり" , 
            "もし", "もしも", 
            "なら", 
            "ない", 
            "以上", "以下", "未満", "より大きい", "より小さい", "等しい", 
            "繰り返す", "繰り返し"
        ]
    }
}

trait YousoTyuusyutsu {
    fn youso_tyuusyutsu(&self) -> Vec<KihonYouso>;
}

impl YousoTyuusyutsu for str {
    fn youso_tyuusyutsu(&self) -> Vec<KihonYouso> {
        self.split_whitespace()
            .filter_map(|w| KihonYouso::from_str(w))
            .collect()
    }
}

/// 位置を検出
fn youso_ichi(text: &str) -> Vec<(usize, &'static str)> {
    let mut result = Vec::new();

    for &word in KihonYouso::all_words() {
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

pub fn kutou_bunkai(text: &str) -> Vec<String> {
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
        let ress = kutou_bunkai(&ron.to_string());
        for res in ress {
            temp.push(res);
        }
    }
    temp
}
