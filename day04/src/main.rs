use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn get_solution_part1<R: BufRead>(reader: &mut R) -> u32 {
    let mut lines = reader.lines();
    let numbers = parse_numbers(&mut lines);
    let mut boards = parse_boards(&mut lines);
    let (number, winning_board) = find_winning_board(numbers, &mut boards).unwrap();

    number * winning_board.remaining_sum()
}

fn get_solution_part2<R: BufRead>(reader: &mut R) -> u32 {
    let mut lines = reader.lines();
    let numbers = parse_numbers(&mut lines);
    let mut boards = parse_boards(&mut lines);

    for number in numbers {
        boards.iter_mut().for_each(|board| { board.apply_draw(number); });
        let remaining = boards.len();
        boards.retain(|board| !board.has_won() || remaining == 1);
        if remaining == 1 && boards[0].has_won() {return number * boards.pop().unwrap().remaining_sum()}
    }

    0
}

fn find_winning_board(numbers: Vec<u32>, boards: &mut Vec<Bingo5x5Board>) -> Option<(u32, &Bingo5x5Board)> {
    for number in numbers {
        for (idx, board) in boards.iter_mut().enumerate() {
            if board.apply_draw(number) { return Some((number, &boards[idx])); }
        }
    }
    return None;
}

fn parse_numbers<R: BufRead>(lines: &mut Lines<&mut R>) -> Vec<u32> {
    let numbers = lines.next().unwrap().unwrap().split(',')
        .map(|number| number.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    numbers
}

fn parse_boards<R: BufRead>(lines: &mut Lines<&mut R>) -> Vec<Bingo5x5Board> {
    let mut boards = Vec::new();
    let mut row = 0;
    let mut board = [[0u32; 5]; 5];
    for line in lines.map(|l| l.unwrap()) {
        if line.is_empty() {
            board = [[0u32; 5]; 5];
            row = 0;
            continue;
        }
        let board_row = parse_bingo5x5_row(line.as_str());
        board[row] = board_row;
        row += 1;
        if row == 5 { boards.push(Bingo5x5Board::from(board)) }
    }
    boards
}


fn read_input(path: &str) -> BufReader<File> {
    let file = File::open(path).expect("not found");
    BufReader::new(file)
}

fn main() {
    let part = match env::var("part") {
        Ok(val) => val,
        Err(_e) => "part1".to_string(),
    };

    let reader = &mut read_input("input.txt");
    println!("Result: {}", match part.as_str() {
        "part2" => get_solution_part2(reader),
        _ => get_solution_part1(reader),
    });
}

#[derive(Debug)]
struct Bingo5x5Board {
    matched_rows: Vec<HashSet<u32>>,
    matched_cols: Vec<HashSet<u32>>,
}

impl From<[[u32; 5]; 5]> for Bingo5x5Board {
    fn from(numbers: [[u32; 5]; 5]) -> Self {
        let mut matched_rows = Vec::new();
        let mut matched_cols: Vec<HashSet<u32>> = (0..5).map(|_i| HashSet::new()).collect();
        for &row in numbers.iter() {
            matched_rows.push(HashSet::from_iter(row));
            for (j, &col) in row.iter().enumerate() {
                matched_cols[j].insert(col);
            }
        }
        Bingo5x5Board { matched_rows, matched_cols }
    }
}

impl Bingo5x5Board {
    fn apply_draw(&mut self, draw: u32) -> bool {
        Bingo5x5Board::mark_matched(&mut self.matched_rows, draw);
        Bingo5x5Board::mark_matched(&mut self.matched_cols, draw);
        self.has_won()
    }

    fn has_won(&self) -> bool {
        Bingo5x5Board::has_bingo(&self.matched_rows) || Bingo5x5Board::has_bingo(&self.matched_cols)
    }

    fn has_bingo(matched: &Vec<HashSet<u32>>) -> bool { matched.iter().any(|row| row.is_empty()) }

    fn mark_matched(matched: &mut Vec<HashSet<u32>>, draw: u32) {
        for row in matched { row.remove(&draw); }
    }

    fn remaining_sum(&self) -> u32 {
        Bingo5x5Board::wrapped(&self.matched_rows)
    }

    fn wrapped(matched: &Vec<HashSet<u32>>) -> u32 {
        matched.into_iter().fold(0 as u32, |acc, col| acc + col.into_iter().fold(0 as u32, |acc, val| acc + val))
    }
}

fn parse_bingo5x5_row(row: &str) -> [u32; 5] {
    row.split_ascii_whitespace()
        .map(|number| number.parse::<u32>().unwrap())
        .collect::<Vec<u32>>().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::{Bingo5x5Board, get_solution_part1, get_solution_part2, parse_bingo5x5_row};

    #[test]
    fn parse_bingo5x5_row_works() {
        assert_eq!(parse_bingo5x5_row("1 2 3 4 5"), [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bingo_5x5_row_win() {
        let mut bingo_board: Bingo5x5Board = [
            [14, 21, 17, 24, 4],
            [10, 16, 15, 9, 19],
            [18, 8, 23, 26, 20],
            [22, 11, 13, 6, 5],
            [2, 0, 12, 3, 7]
        ].into();

        assert_eq!(bingo_board.apply_draw(14), false);
        assert_eq!(bingo_board.apply_draw(21), false);
        assert_eq!(bingo_board.apply_draw(17), false);
        assert_eq!(bingo_board.apply_draw(24), false);
        assert_eq!(bingo_board.apply_draw(4), true);
    }

    #[test]
    fn test_bingo_5x5_column_win() {
        let mut bingo_board: Bingo5x5Board = [
            [0, 2, 3, 4, 5],
            [6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20],
            [21, 22, 23, 24, 25]
        ].into();

        assert_eq!(bingo_board.apply_draw(2), false);
        assert_eq!(bingo_board.apply_draw(7), false);
        assert_eq!(bingo_board.apply_draw(12), false);
        assert_eq!(bingo_board.apply_draw(17), false);
        assert_eq!(bingo_board.apply_draw(22), true);
    }


    const TEST_INPUT: &'static str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn get_solution_part_1_works() {
        let input = TEST_INPUT.as_bytes();
        assert_eq!(get_solution_part1(&mut BufReader::new(input)), 4512);
    }

    #[test]
    fn get_solution_part_2_works() {
        let input = TEST_INPUT.as_bytes();
        assert_eq!(get_solution_part2(&mut BufReader::new(input)), 1924);
    }
}
