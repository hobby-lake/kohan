mod project;// 書籍
mod argument;// 論
mod part;// 段落
mod essence;// 基本型・基本要素
mod dependence;// 付属語

pub fn koubun_kaiseki(syoseki:String) -> Result<Vec<String>, &'static str> {
    let ron_s = project::bunkai(syoseki);
    let danraku_s = argument::bunkai(ron_s);
    let koubun_s = part::bunkai(danraku_s.unwrap());
    let youso_arrays = essence::bunkai(koubun_s.unwrap());
    let segment_arrays = dependence::bunkai(youso_arrays.unwrap());
    segment_arrays
}