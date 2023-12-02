use std::collections::HashMap;

fn create_number_map<'a>() -> HashMap<&'a str, i32> {
    HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ])
}

pub fn string_number_occurences(input: &str) -> Vec<(char, i32)> {
    let number_map: HashMap<&str, i32> = create_number_map();
    let mut number_index: Vec<(char, i32)> = Vec::new();
    // for (word, num) in number_map {
    //     if input.contains(word) {
    //         let index: i32 = input.find(word).unwrap() as i32;
    //         // Lots of casting, basically turn 1 -> '1'
    //         let char_num =(num as u8 + b'0') as char;
    //         number_index.push((char_num, index))
    //     }
    // }


    for (word, num) in number_map {
        let mut start = 0;
        while let Some(found) = input[start..].find(word) {
            let index = start as i32 + found as i32;
            let char_num = (num as u8 + b'0') as char;
            number_index.push((char_num, index as i32));
            start += found + 1;
        }
    }

    let mut chars_from_num: Vec<(char, i32)> = find_integers(input);

    number_index.append(&mut chars_from_num);
    // sort to make the index comparisons easier for later
    number_index.sort_by_key(|k| k.1);
    return number_index
}

pub fn find_integers(input: &str) -> Vec<(char, i32)> {
    // Here we map the character ie. the digit along with the index
    // We dont need to sort this one since this goes by indexing order 
    let mut chars_from_num: Vec<(char, i32)> = input
        .chars()
        .enumerate()
        .filter_map(|(index, ch)| {
            if ch.is_digit(10) {
                Some((ch, index as i32))
            } else {
                None
            }
        })
        .collect();
    // Sort them by their index value
    // chars_from_num.sort_by_key(|k| k.1);
    return chars_from_num
}

pub fn trebuchet(input: Vec<&str>) -> i32 {

    let mut result_vector: Vec<Vec<(char, i32)>> = Vec::new();
    let mut sum: i32 = 0;
    for line in input {
        result_vector.push(string_number_occurences(line));
    }
    dbg!(&result_vector);
    dbg!(&result_vector.len());
    common::write_output(format!("{:#?}", &result_vector).as_str());
    for elem in result_vector {
        let first_number = elem.first().unwrap().0;
        let last_number = elem.last().unwrap().0;
        let mut number_string = String::new();
        number_string.push(first_number);
        number_string.push(last_number);
        dbg!(&number_string);
        sum = sum + number_string.parse::<i32>().unwrap();
    }

    sum
}