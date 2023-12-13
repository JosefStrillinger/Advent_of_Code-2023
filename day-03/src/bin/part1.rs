fn main() {
    let input = include_str!("input1.txt");

    let symbol_locations = parse_input(input);

    println!("{:?}", symbol_locations);
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let mut output = Vec::new();
    for (y, line) in input.lines().enumerate() {
        println!("{} {}", y, line);
        for (x, content) in line.chars().enumerate() {
            println!("{} {}", x, content);
            if content == '#' || content == '*' || content == '$' || content == '+' {
                output.push((y, x))
            }
        }
    }
    output
}
// TODO: Implement
// Ideas:
// 1 parse input into a grid / matrix (maybe made from two vectors and then work with that)
// Check the the Correct values based on the position relative to the symbols
