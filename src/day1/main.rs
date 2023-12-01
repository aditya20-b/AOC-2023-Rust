use common::read_input;
mod part1;
mod part2;
fn main() {
    let input_string: String = read_input("input.txt").unwrap();
    let input_array: Vec<&str> = input_string.lines().collect();
    let output = part1::trebuchet(input_array);
    dbg!(&output);
}
