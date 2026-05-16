use crate::jp_parser::meta::PARSED;

mod project;// 書籍
mod argument;// 論
mod part;// 段落
mod essence;// 基本型・基本要素
mod dependence;// 付属語
mod unit;// 単位

pub fn koubun_kaiseki(syoseki:String) -> Vec<String> {
    let mut result = project::bunkai(syoseki);
    result = argument::bunkai(&mut result);
    result = part::bunkai(&mut result);
    result = essence::bunkai(&mut result);
    result = dependence::bunkai(&mut result);
    result = unit::bunkai(&mut result);
    result
}