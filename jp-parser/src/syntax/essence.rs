use crate::syntax::meta::{KOUBUN, YOUSO};

/// 基本型・基本要素

pub enum Fuzokugo {
    Ha,// は
}

/// 品詞の大枠
/// 使用者定義で使うもの、標準定義されているものを自立語として定義し、それ以外を付属語とする。
pub enum Hinshi {
    Jiritsugo,// 自立語
    Fuzokugo(Fuzokugo),// 付属語
}

pub enum KihonKata {
    Moji,// 文字
    Su,// 数
    Seigo,// 正誤
}

pub enum KihonYouso {
    Touka, // 等価
    Wa, // 和
    Sa, // 差
    Hazu, // はず(推論)
    Naru, // 成る(代入)
    Beki, // べき(強制)
}
impl KihonYouso {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "等価" => Some(Self::Touka),
            "和" => Some(Self::Wa),
            "差" => Some(Self::Sa),
            "はず" => Some(Self::Hazu),
            "成る" | "なる" => Some(Self::Naru),
            "べき" => Some(Self::Beki),
            _ => None,
        }
    }
    
    pub fn all_words() -> &'static [&'static str] {
        &["等価", "和", "差", "はず", "成る", "べき"]
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

/// 接続詞の位置を検出
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
        // 接続詞の直前までを追加
        if last < pos {
            result.push(text[last..pos].to_string());
        }
        // 接続詞そのものを追加
        result.push(word.to_string());
        last = pos + word.len();
    }

    // 最後の残り
    if last < text.len() {
        result.push(text[last..].to_string());
    }

    result
}

// 構文から要素の配列へ
pub fn bunkai(koubun_s:Vec<String>) -> Result<Vec<String>, &'static str> {
    let mut m_koubun_s = koubun_s;
    let id = m_koubun_s.remove(0);
    if id == String::from(KOUBUN) {
        let mut temp: Vec<String> = vec![String::from(YOUSO)];
        for koubun in m_koubun_s {
            let ress = kutou_bunkai(&koubun.to_string());
            for res in ress {
                temp.push(res);
            }
        }
        Ok(temp)
    } else {
        Err("[ERR] 入力が 構文 ではありません")
    }
}
