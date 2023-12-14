fn main() {
    let input = include_str!("input1.txt");

    let symbol_locations = parse_input_symbol_locations(input);

    println!("{:?}", symbol_locations);

    let number_data = parse_input_number(input);
    println!("{:?}", number_data);

    let solution = get_solution(&symbol_locations, &number_data);
    println!("Solution: {}", solution);
}

fn parse_input_symbol_locations(input: &str) -> Vec<(i32, i32)> {
    let mut output = Vec::new();
    for (y, line) in input.lines().enumerate() {
        //println!("{} {}", y, line);
        for (x, content) in line.chars().enumerate() {
            //println!("{} {}", x, content);
            if !content.is_numeric() && content != '.' {
                output.push((y as i32, x as i32));
            }
        }
    }
    output
}

fn parse_input_number(input: &str) -> Vec<(i32, i32, i32, i32)> {
    let mut output = Vec::new();
    for (y, line) in input.lines().enumerate() {
        //println!("{} {}", y, line);
        let mut help_vec_location = Vec::new();
        let mut help_vec_val = Vec::new();
        for (x, content) in line.chars().enumerate() {
            //println!("{} {}", x, content);
            if content.is_numeric() {
                //println!("Adding to values {} {}", x, content);
                help_vec_location.push(x);
                help_vec_val.push(content);
            }
            if (!content.is_numeric() || x == line.len() - 1)
                && help_vec_val.len() > 0
                && help_vec_location.len() > 0
            {
                let mut help_str = String::new();
                for i in &help_vec_val {
                    help_str.push(*i);
                    //println!("Adding to string {} {}", i, help_str);
                }
                output.push((
                    help_str.parse::<i32>().unwrap(),
                    help_vec_location[0] as i32,
                    help_vec_location[help_vec_location.len() - 1] as i32,
                    y as i32,
                ));
                help_vec_location.clear();
                help_vec_val.clear();
                help_str.clear();
            }
        }
    }
    output
}

fn get_solution(input_symbols: &Vec<(i32, i32)>, input_numbers: &Vec<(i32, i32, i32, i32)>) -> i32 {
    let mut output = Vec::new();
    for number in input_numbers {
        for symbol in input_symbols {
            if (symbol.0 >= number.3 - 1 && symbol.0 <= number.3 + 1)
                && (symbol.1 >= number.1 - 1 && symbol.1 <= number.2 + 1)
            {
                output.push(number.0);
            }
        }
    }
    let sum: i32 = output.iter().sum();
    sum
}
