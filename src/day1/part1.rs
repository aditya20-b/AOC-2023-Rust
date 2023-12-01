pub fn trebuchet(input: Vec<&str>) -> i32 {
    let mut number_list: Vec<i32> = Vec::new();
    for item in input {
        number_list.push(find_integers(item))
    }
    number_list.iter().sum()
}

pub fn find_integers(input: &str) -> i32 {
    let mut number_string = String::new();

    let chars_from_num: Vec<char> = input.chars().filter(|x| x.is_digit(10)).collect();

    // idk why this turns into a reference here so we own it 
    if let (Some(first), Some(last)) = (chars_from_num.first(), chars_from_num.last()) {
        number_string.push(first.to_owned());
        number_string.push(last.to_owned());
    }

    return number_string.parse::<i32>().unwrap();
}
