fn main() {
    let input = include_str!("./input1.txt");

    println!("{}", process(input));
}

pub fn process(input: &str) -> u32{
    let output = input
        .lines()
        .map(|line| {
            //TODO: Implement
            // probably use hashmap with custom set that ckecks if value is bigger than current and add that hashmap into a vector
        });
    0
}
