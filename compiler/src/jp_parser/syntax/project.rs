// 書籍

// 書籍から論へ
pub fn bunkai (syoseki:String) -> Vec<String>{
    let mut temp: Vec<String> = vec![];
    let ron_s: Vec<String> = syoseki.lines().map(|s| s.to_string()).collect();
    for ron in ron_s {
        temp.push(ron);
    }
    temp
}