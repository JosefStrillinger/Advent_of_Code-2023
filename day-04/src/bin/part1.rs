fn main() {
    let input = include_str!("input1.txt");
    let parsed_input = parse_input(&input);
    println!("{}", process(&parsed_input));
}

fn parse_input(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    let mut output = Vec::new();
    for line in input.lines() {
        let mut winning_numbers = Vec::new();
        let mut drawn_numbers = Vec::new();
        let parts = line.split(":").collect::<Vec<&str>>()[1]
            .split("|")
            .collect::<Vec<&str>>();
        // Get through the parts and put numbers into vec
        for part in parts[0].split_whitespace() {
            winning_numbers.push(part);
        }
        for part in parts[1].split_whitespace() {
            drawn_numbers.push(part);
        }
        output.push((winning_numbers, drawn_numbers));
    }
    output
}

fn process(input: &Vec<(Vec<&str>, Vec<&str>)>) -> i32 {
    let mut count = Vec::new();
    for (winning_numbers, drawn_numbers) in input {
        let mut correct_numbers = Vec::new();
        let mut line_count = 0;
        for drawn_number in drawn_numbers {
            if winning_numbers.contains(&drawn_number) {
                correct_numbers.push(drawn_number);
            }
        }
        for i in 0..correct_numbers.len() {
            line_count = line_count * 2;
            if i == 0 {
                line_count = line_count + 1;
            }
        }
        count.push(line_count);
    }
    count.iter().sum()
}
