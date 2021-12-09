use advent_of_code_2021::read_lines_from_input;
use std::convert::TryFrom;

fn main() {
    let contents = read_lines_from_input("day_1_input");
    let input: Vec<u16> = contents
        .iter()
        .map(|val| val.parse::<u16>().unwrap())
        .collect::<Vec<u16>>();

    let mut measured = false;
    let mut num_increases: u16 = 0;

    let mut last: [u16; 3] = [0, 0, 0];

    for value in input.windows(3) {
        let last_sum: u16 = last.iter().sum();
        let latest_sum: u16 = value.iter().sum();
        if !measured {
            println!("{} (N/A - no previous measurement)", latest_sum);
            measured = true;
        } else if latest_sum == last_sum {
            println!("{} (no change)", latest_sum);
        } else if latest_sum > last_sum {
            println!("{} (increase)", latest_sum);
            num_increases += 1
        } else {
            println!("{} (decreased)", latest_sum);
        }
        last = <[u16; 3]>::try_from(value).unwrap();
    }
    println!("Number of increases: {}", num_increases);
}
