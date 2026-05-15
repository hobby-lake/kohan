#[allow(unused)]
mod jp_parser;
#[allow(unused)]
mod compile;

fn main() {
    let f_path = r"D:/.prj/Language/kohan/test_file/test.khn";
    let encoded = jp_parser::parse(f_path);
    compile::execute(encoded);
}