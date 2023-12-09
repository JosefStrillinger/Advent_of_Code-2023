#[derive(Debug)]
pub struct Round {
    blue: i32,
    red: i32,
    green: i32,
} 

fn main() {
    let input = include_str!("./input1.txt");

    let data = parse(input);
    //dbg!(data);

    let compare = Round{
        blue: 14,
        red: 12,
        green: 13,
    };

    let output = process(&data, &compare);
    println!("Output: {}", output);

}

pub fn parse(input: &str) -> Vec<Round> { // Final Outpout will be Result with Error and A Vect of Rounds
    let mut v: Vec<Round> = Vec::new();
    for line in input.lines(){
        let mut round = Round{
            blue: 0,
            red: 0,
            green: 0,
        };
        let data = line.split(":").collect::<Vec<&str>>()[1].trim();
        // replace ; with , in data
        let data = data.replace(";", ",");

        // split data by ,
        let data = data.split(",").collect::<Vec<&str>>();

        for entry in data {
            let entry_data = entry.trim().split(" ").collect::<Vec<&str>>();
            let int_entry_data = entry_data[0].parse::<i32>().unwrap();
            match entry_data[1]{
                "blue" => {
                    if int_entry_data > round.blue {
                        round.blue = int_entry_data;
                    }
                },
                "red" => {
                    if int_entry_data > round.red {
                        round.red = int_entry_data;
                    }
                },
                "green" => {
                    if int_entry_data > round.green {
                        round.green = int_entry_data;
                    }
                },
                _ => {},
            }
        }
        v.push(round);
    }
    v
}

pub fn process(input: &Vec<Round>, compare: &Round) -> usize{
    let mut output = 0; 
    for (i, e) in input.iter().enumerate() {
        if e.blue <= compare.blue && e.red <= compare.red && e.green <= compare.green {
            output += i+1;
        }
    }
    output
} 
