// 論

///順接
pub enum Junsetsu {
    Dakara,
    Shitagatte,
    Yotte,
}
impl Junsetsu {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "だから" => Some(Self::Dakara),
            "したがって" => Some(Self::Shitagatte),
            "よって" => Some(Self::Yotte),
            _ => None,
        }
    }
}

///逆説
pub enum Gyakusetsu {
    Shikashi,
    Tada,
    Daga,
    Demo,
    Tokoroga,
}
impl Gyakusetsu {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "しかし" => Some(Self::Shikashi),
            "ただ" => Some(Self::Tada),
            "だが" => Some(Self::Daga),
            "でも" => Some(Self::Demo),
            "ところが" => Some(Self::Tokoroga),
            _ => None,
        }
    }
}


///並列
pub enum Heiretsu {
    Mata,
    Oyobi,
    Katsu,
    Douyouni,
}
impl Heiretsu {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "また" => Some(Self::Mata),
            "および" => Some(Self::Oyobi),
            "かつ" => Some(Self::Katsu),
            "どうように" => Some(Self::Douyouni),
            _ => None,
        }
    }
}

///添加
pub enum Tenka {
    Soshite,
    Sorekara,
    Sonouede,
}
impl Tenka {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "そして" => Some(Self::Soshite),
            "それから" => Some(Self::Sorekara),
            "その上で" | "そのうえで" => Some(Self::Sonouede),
            _ => None,
        }
    }
}

///対比
pub enum Taihi {
    Gyakuni,
    Ippoude,
    Sonokawari,
}
impl Taihi {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "逆に" | "ぎゃくに" => Some(Self::Gyakuni),
            "一方で" | "いっぽうで" => Some(Self::Ippoude),
            "そのかわり" => Some(Self::Sonokawari),
            _ => None,
        }
    }
}

///補足
pub enum Hosoku {
    Nazenaraba,
    Sonoriyuuha,
    Toiunoha,
}
impl Hosoku {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "なぜならば" => Some(Self::Nazenaraba),
            "その理由は" | "そのりゆうは" => Some(Self::Sonoriyuuha),
            "というのは" => Some(Self::Toiunoha),
            _ => None,
        }
    }
}

///転換
pub enum Tenkan {
    Tsumari,
    Sunawachi,
}
impl Tenkan {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "つまり" => Some(Self::Tsumari),
            "すなわち" => Some(Self::Sunawachi),
            _ => None,
        }
    }
}

///条件

pub enum Kakutei {
    Dattara,
    Sorenaraba,
    Datosureba,
}
impl Kakutei {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "だったら" => Some(Self::Dattara),
            "それならば" => Some(Self::Sorenaraba),
            "だとすれば" => Some(Self::Datosureba),
            _ => None,
        }
    }
}

pub enum Katei {
    Dearunara,
    Moshimo,
    Sounareba,
}
impl Katei {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "であるなら" => Some(Self::Dearunara),
            "もしも" => Some(Self::Moshimo),
            "そうなれば" => Some(Self::Sounareba),
            _ => None,
        }
    }
}

pub enum Jouken {
    Kakutei(Kakutei),
    Katei(Katei),
}
impl Jouken {
    pub fn from_str(s: &str) -> Option<Self> {
        if let Some(k) = Kakutei::from_str(s) {
            return Some(Self::Kakutei(k));
        }
        if let Some(k) = Katei::from_str(s) {
            return Some(Self::Katei(k));
        }
        None
    }
}

pub enum Setsuzokushi {
    Junsetsu(Junsetsu),
    Gyakusetsu(Gyakusetsu),
    Heiretsu(Heiretsu),
    Tenka(Tenka),
    Taihi(Taihi),
    Hosoku(Hosoku),
    Tenkan(Tenkan),
    Jouken(Jouken),
}
impl Setsuzokushi {
    pub fn from_str(s: &str) -> Option<Self> {
        if let Some(v) = Junsetsu::from_str(s) {
            return Some(Self::Junsetsu(v));
        }
        if let Some(v) = Gyakusetsu::from_str(s) {
            return Some(Self::Gyakusetsu(v));
        }
        if let Some(v) = Heiretsu::from_str(s) {
            return Some(Self::Heiretsu(v));
        }
        if let Some(v) = Tenka::from_str(s) {
            return Some(Self::Tenka(v));
        }
        if let Some(v) = Taihi::from_str(s) {
            return Some(Self::Taihi(v));
        }
        if let Some(v) = Hosoku::from_str(s) {
            return Some(Self::Hosoku(v));
        }
        if let Some(v) = Tenkan::from_str(s) {
            return Some(Self::Tenkan(v));
        }
        if let Some(v) = Jouken::from_str(s) {
            return Some(Self::Jouken(v));
        }
        None
    }
    
    pub fn all_words() -> &'static [&'static str] {
        &[
            // Junsetsu
            "だから", "したがって", "よって",
            // Gyakusetsu
            "しかし", "ただ", "だが", "でも", "ところが",
            // Heiretsu
            "また", "および", "かつ", "同様に",
            // Tenka
            "そして", "それから", "その上で",
            // Taihi
            "逆に", "一方で", "その代わり",
            // Hosoku
            "なぜならば", "その理由は", "というのは",
            // Tenkan
            "つまり", "すなわち",
            // Jouken
            "だったら", "それならば", "だとすれば",
            "であるなら", "もしも", "そうなれば",
        ]
    }
}

trait SetsuzokushiExtract {
    fn extract_setsuzokushi(&self) -> Vec<Setsuzokushi>;
}

impl SetsuzokushiExtract for str {
    fn extract_setsuzokushi(&self) -> Vec<Setsuzokushi> {
        self.split_whitespace()
            .filter_map(|w| Setsuzokushi::from_str(w))
            .collect()
    }
}

/// 位置を検出
fn setsuzokuichi(text: &str) -> Vec<(usize, &'static str)> {
    let mut result = Vec::new();

    for &word in Setsuzokushi::all_words() {
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

pub fn ron_bunkai(text: &str) -> Vec<String> {
    let positions = setsuzokuichi(text);

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

// 論から段落へ
pub fn bunkai(rons: &mut Vec<String>) -> Vec<String> {
    let mut m_rons = rons;
    let mut temp: Vec<String> = vec![];
    for ron in m_rons {
        let ress = ron_bunkai(&ron.to_string());
        for res in ress {
            temp.push(res);
        }
    }
    temp
}