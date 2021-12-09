use advent_of_code_2021::read_lines_from_input;

/// Collect counts for the bits in the range specified
fn count_bits(
    binary_strings: &Vec<String>,
    binary_str_len: u32,
    count_bit_start: Option<u32>,
    count_bit_end: Option<u32>,
) -> std::vec::Vec<(u32, u32)> {
    let mut counts: Vec<_> = Vec::new();
    for _ in 0..binary_str_len {
        counts.push((0, 0))
    }

    for binary_str in binary_strings {
        let chars: Vec<_> = binary_str.chars().collect();
        let bit_char_range = (count_bit_start.unwrap_or_default() as usize)
            ..=(std::cmp::min(
                binary_str_len - 1,
                count_bit_end.unwrap_or(binary_str_len - 1),
            ) as usize);
        for i in bit_char_range {
            let bit = chars.get(i).unwrap();
            match bit {
                '0' => counts[i].0 += 1,
                '1' => counts[i].1 += 1,
                _ => println!("Invalid bit detected {}", bit),
            }
        }
    }

    counts
}

fn count_total_bits(bit_counts: &[(u32, u32)]) -> (Vec<u8>, Vec<u8>) {
    let mut bits_zero: Vec<u8> = Vec::new();
    let mut bits_one: Vec<u8> = Vec::new();
    for count in bit_counts {
        if count.0 > count.1 {
            bits_zero.push(0);
            bits_one.push(1);
        } else {
            bits_zero.push(1);
            bits_one.push(0);
        }
    }

    (bits_zero, bits_one)
}

#[derive(Eq, PartialEq)]
enum Operator {
    Most,
    Least,
}

fn get_common_bit(binary_strings: &Vec<String>, binary_str_len: u32, op: Operator) -> Vec<u8> {
    let mut filtered_binary_strings: Vec<String> = Vec::new();
    let mut filtered_binary_strings_ref: &Vec<String> = binary_strings;
    for i in 0..binary_str_len {
        if filtered_binary_strings.len() == 1 {
            break;
        }
        filtered_binary_strings =
            reduce_binary_strings(filtered_binary_strings_ref, binary_str_len, i, &op);
        filtered_binary_strings_ref = &filtered_binary_strings;
    }

    let result = filtered_binary_strings
        .get(0)
        .unwrap_or(&"".to_string())
        .clone();

    result
        .chars()
        .map(|ch| if ch == '0' { 0 } else { 1 })
        .collect()
}

fn reduce_binary_strings(
    binary_strings: &Vec<String>,
    binary_str_len: u32,
    column: u32,
    op: &Operator,
) -> Vec<String> {
    // count bits in column
    let counts_for_col = *count_bits(binary_strings, binary_str_len, Some(column), Some(column))
        .get(column as usize)
        .unwrap();
    let eql_count = counts_for_col.0 == counts_for_col.1;
    let filter_by = match op {
        Operator::Most => {
            if eql_count || counts_for_col.0 < counts_for_col.1 {
                '1'
            } else {
                '0'
            }
        }
        Operator::Least => {
            if eql_count || counts_for_col.0 < counts_for_col.1 {
                '0'
            } else {
                '1'
            }
        }
    };

    // reduce to most common or least common
    binary_strings
        .iter()
        .filter(|binary_str| binary_str.chars().nth(column as usize).unwrap() == filter_by)
        .cloned()
        .collect::<Vec<String>>()
}

fn bit_collection_to_decimal(u8s: &[u8]) -> i32 {
    let str: String = u8s.iter().map(|val| val.to_string()).collect();
    i32::from_str_radix(&str, 2).unwrap()
}

fn main() {
    let contents = read_lines_from_input("day_3_input");
    let inputs: Vec<_> = contents.iter().map(|val| val.to_string()).collect();

    let binary_str_len: u32 = inputs[0].len().try_into().unwrap();
    let bit_counts = count_bits(&inputs, binary_str_len, None, None);

    let (gamma_bits, epsilon_bits) = count_total_bits(&bit_counts);
    let gamma = bit_collection_to_decimal(&gamma_bits);
    let epsilon = bit_collection_to_decimal(&epsilon_bits);

    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);
    println!("Consumption: {}", gamma * epsilon);

    let oxygen_generator_rating =
        bit_collection_to_decimal(&get_common_bit(&inputs, binary_str_len, Operator::Most));
    dbg!(oxygen_generator_rating);
    println!("Oxygen Generator Rating: {}", oxygen_generator_rating);

    let co2_scrubber_rating =
        bit_collection_to_decimal(&get_common_bit(&inputs, binary_str_len, Operator::Least));
    println!("CO2 Scrubber Rating: {}", co2_scrubber_rating);

    println!(
        "Oxygen Generator Rating: {}",
        oxygen_generator_rating * co2_scrubber_rating
    );
}
