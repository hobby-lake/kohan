mod array;
mod categorize;
mod typescript;

pub fn execute(encoded_data:String) {
    let code_vector = array::convert(encoded_data);
    let code_hash = categorize::execute(code_vector);

    
}