mod array;
mod categorize;
mod typescript;

pub fn execute(encoded_data:String) {
    let code_array = array::convert(encoded_data);
    categorize::execute(code_array);
}