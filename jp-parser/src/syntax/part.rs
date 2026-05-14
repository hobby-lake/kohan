// 段落

use std::vec;

use crate::syntax::meta::{DANRAKU, KOUBUN};

pub enum Kutouten {
    Kuten,
    Touten,
}
impl Kutouten {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "。" => Some(Self::Kuten),
            "、" => Some(Self::Touten),
            _ => None,
        }
    }
    
    pub fn all_words() -> &'static [&'static str] {
        &["。", "、"]
    }
}

trait KutoutenExtract {
    fn extract_kutouten(&self) -> Vec<Kutouten>;
}

impl KutoutenExtract for str {
    fn extract_kutouten(&self) -> Vec<Kutouten> {
        self.split_whitespace()
            .filter_map(|w| Kutouten::from_str(w))
            .collect()
    }
}

/// 接続詞の位置を検出
fn kutouichi(text: &str) -> Vec<(usize, &'static str)> {
    let mut result = Vec::new();

    for &word in Kutouten::all_words() {
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

pub fn danraku_bunkai(text: &str) -> Vec<String> {
    let positions = kutouichi(text);

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

// 段落から構文へ
pub fn bunkai(danraku_s:Vec<String>) -> Result<Vec<String>, &'static str> {
    let mut m_danraku_s = danraku_s;
    let id = m_danraku_s.remove(0);
    if id == String::from(DANRAKU) {
        let mut temp: Vec<String> = vec![String::from(KOUBUN)];
        for danraku in m_danraku_s {
            let ress = danraku_bunkai(&danraku.to_string());
            for res in ress {
                temp.push(res);
            }
        }
        Ok(temp)
    } else {
        Err("[ERR] 入力が 段落 ではありません")
    }
}