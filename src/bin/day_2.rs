use advent_of_code_2021::read_lines_from_input;

fn main() {
    let contents = read_lines_from_input("day_2_input");
    let inputs: Vec<(String, u32)> = contents
        .iter()
        .map(|val| {
            let val_str = val.to_string();
            let mut val_split = val_str.split_whitespace();
            let pair: (String, u32) = (
                val_split.next().unwrap().to_string(),
                val_split.next().unwrap().parse::<u32>().unwrap(),
            );

            pair
        })
        .collect::<Vec<(String, u32)>>();

    let mut pos_horizontal: i32 = 0;
    let mut pos_depth: i32 = 0;
    let mut aim: i32 = 0;
    for input in inputs {
        let direction = input.0;
        let units = input.1 as i32;
        match direction.as_ref() {
            "forward" => {
                pos_horizontal += units;
                pos_depth += aim * units;
            }
            "down" => {
                aim += units;
            }
            "up" => {
                aim -= units;
            }
            _ => println!("unknown direction"),
        }
    }
    println!("Position Horizontal: {}", pos_horizontal);
    println!("Position Depth: {}", pos_depth);
    println!("Total Multiplied: {}", pos_horizontal * pos_depth);
}
