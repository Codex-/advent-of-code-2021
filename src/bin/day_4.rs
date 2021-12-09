use advent_of_code_2021::read_lines_from_input;

struct Board {
    rows: Vec<Vec<(u32, bool)>>,
    won: bool,
}

fn draw_input_str_to_vec(input: &str) -> Vec<u32> {
    input
        .split(',')
        .into_iter()
        .map(|val| val.parse::<u32>().unwrap())
        .collect()
}

fn update_board_with_draw(draw: u32, board: &mut Board) {
    for row in &mut board.rows {
        for mut val in row {
            if val.0 == draw {
                val.1 = true;
            }
        }
    }
}

fn update_boards_with_draw(
    draw: u32,
    boards: &mut Vec<Board>,
    should_check_win: bool,
) -> (Option<&Board>, bool) {
    let mut last_board: Option<&Board> = None;
    let mut found: bool = false;
    for board in boards {
        update_board_with_draw(draw, board);
        if should_check_win && !board.won {
            let win = check_board_win(board);
            if win {
                board.won = true;
                last_board = Some(board);
                found = true;
            }
        }
    }

    (last_board, found)
}

fn check_board_win(board: &Board) -> bool {
    for i in 0..5 {
        let row = &board.rows[i].clone();
        let column: &Vec<(u32, bool)> = &board.rows.iter().map(|row| row[i]).collect();
        '_rowcol: for collection in [row, column] {
            for (matching, val) in collection.iter().enumerate() {
                if !val.1 {
                    continue '_rowcol;
                };
                if matching == 4 {
                    return true;
                }
            }
        }
    }

    false
}

fn boards_input_str_to_boards(input: &[String]) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();
    let mut board_collector: Vec<Vec<(u32, bool)>> = Vec::new();
    for i in 1..input.len() {
        let row = &input[i];
        if !row.is_empty() {
            let row_nums: Vec<_> = row
                .split_whitespace()
                .into_iter()
                .map(|val| (val.parse::<u32>().unwrap(), false))
                .collect();
            board_collector.push(row_nums);
        }
        if (row.is_empty() && !board_collector.is_empty()) || i == input.len() - 1 {
            boards.push(Board {
                rows: board_collector,
                won: false,
            });
            board_collector = Vec::new();
        }
    }

    boards
}

fn sum_uncalled_numbers(board: &Board) -> u32 {
    let mut sum: u32 = 0;
    for row in &board.rows {
        let row_sum: u32 = row
            .iter()
            .map(|val| if !val.1 { val.0 } else { 0 })
            .collect::<Vec<u32>>()
            .iter()
            .sum();
        sum += row_sum;
    }
    sum
}

fn main() {
    let input = read_lines_from_input("day_4_input");
    let draw_input = draw_input_str_to_vec(&input[0]);

    let mut boards = boards_input_str_to_boards(&input);

    for (count, draw) in draw_input.into_iter().enumerate() {
        let (board, is_winner) = update_boards_with_draw(draw, &mut boards, count >= 4);
        if is_winner {
            let board_unwrapped = board.unwrap();
            let sum = sum_uncalled_numbers(board_unwrapped);
            println!(
                "Draw: {}, Undrawn: {}, Multiplied: {}",
                draw,
                sum,
                draw * sum
            );
        }
    }
}
