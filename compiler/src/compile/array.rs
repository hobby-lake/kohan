pub fn convert(encoded_data:String) -> Vec<String> {
    let mut code_array:Vec<String> = encoded_data
        .split(|c| c == '、' || c == '。')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    let mut temp:Vec<String> = vec![];
    let mut is_doubled = false;

    for index in (0..code_array.len()).rev() {
        let code_line = &code_array[index];
        if is_doubled == true {
            is_doubled = false;
            continue;
        }

        if let Some(prefix) = code_line.chars().next() {
            match prefix {
                '|' => {
                    if index != 0 {
                        temp.push(format!("{}{}", code_array[index - 1], code_line));
                        is_doubled = true;
                    }
                }
                _ => temp.push(code_line.to_string()),
            }
        }
    }
    temp.reverse();
    code_array = temp;

    println!("{:?}", code_array);
    code_array
}