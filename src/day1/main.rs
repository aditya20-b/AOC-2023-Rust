use common::read_input;
mod part1;
mod part2;
fn main() {
    let input_string: String = read_input("input.txt").unwrap();
    let input_array: Vec<&str> = input_string.lines().collect();
    // let output = part1::trebuchet(input_array);
    // let input_array = vec![
    //     "9sixonefour",
    //     "eighttwo2twofour9",
    //     "7eightseveneightthree",
    //     "tlnllks2jcfdlgsjbhpfnineone",
    //     "one44fivesevenjzsfzddg",
    // ];
    let mut out1:Vec<Vec<(i32, i32)>> = Vec::new();
    let mut out2:Vec<Vec<(char, i32)>> = Vec::new();
    for line in &input_array {
        out1.push(part2::string_number_occurences(line));
    }
    for line in &input_array {
        out2.push(part2::find_integers(line));
    }

    dbg!(part2::find_integers("cmpptgjc3qhcjxcbcqgqkxhrms"));
    dbg!(&out1);
    dbg!(&out2);

}
