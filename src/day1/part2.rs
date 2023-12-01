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


pub fn string_number_occurences(input: &str) -> Vec<(i32, i32)> {
    let number_map: HashMap<&str, i32> = create_number_map();
    let mut number_index: Vec<(i32, i32)> = Vec::new();
    for (word, num) in number_map {
        if input.contains(word) {
            let index: i32 = input.find(word).unwrap() as i32;
            number_index.push((num, index))
        }
    }
    // sort to make the index comparisons easier for later
    number_index.sort_by_key(|k| k.1);
    return number_index
}

pub fn find_integers(input: &str) -> Vec<(char, i32)> {
    // Here we map the character ie. the digit along with the index
    // We dont need to sort this one since this goes by indexing order 
    let chars_from_num: Vec<(char, i32)> = input
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
    return chars_from_num
}

pub fn trebuchet(input: Vec<&str>) -> i32 {
    let mut character_mappings: Vec<Vec<(i32, i32)>> = Vec::new();
    let mut number_mapping:Vec<Vec<(char, i32)>> = Vec::new();
    let mut result_vector: Vec<(char, i32, i32)> = Vec::new();

    for line in &input {
        character_mappings.push(string_number_occurences(line));
        number_mapping.push(find_integers(line));
    }
    // We go into the tuple for each string
    for (char_tuples, number_tuples) in character_mappings.iter().zip(number_mapping.iter()) {
        // Check first and last values in the tuple
        if let (Some(first_char_tuple), Some(first_number_tuple))  = (char_tuples.first(), number_tuples.last()) {
            // TODO: Implement string zipping
            // If the index of string based mapping is lower than that of the number based mapping
            // Then we push the lower indexed string into a vector/string buffer
            if first_char_tuple.1 < first_number_tuple.1 {
                result_vector.push((first_number_tuple.0, first_char_tuple.0, first_char_tuple.1))
            } else {
                // Else here we push the other number in
            }

            // TODO: Repeat the same steps in order to find the least number by comparing their last indices
            
            // TODO: After that, combine both the first + last element of a PARTICULAR string and then
            // TODO: parse it into i32 and add it onto final sum_vector
            // TODO: Repeat process for entire list of string and then find the sum of sum_vector
        }


    };


    0
}