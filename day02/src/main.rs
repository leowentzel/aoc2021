use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::Move::{Down, Forward, Up};

#[derive(PartialEq, Debug)]
enum Move { Forward(u32), Up(u32), Down(u32) }

struct WorkingSubmarine {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

struct DefectSubmarine {
    horizontal: u32,
    depth: u32,
}

trait Submarine {
    fn drive(&mut self, direction: &Move);
}

impl Submarine for DefectSubmarine {
    fn drive(&mut self, direction: &Move) {
        match direction {
            Forward(amount) => self.horizontal += amount,
            Up(amount) => self.depth -= amount,
            Down(amount) => self.depth += amount
        }
    }
}

impl Submarine for WorkingSubmarine {
    fn drive(&mut self, direction: &Move) {
        match direction {
            Forward(amount) => {
                self.horizontal += amount;
                self.depth += self.aim * amount
            }
            Up(amount) => self.aim -= amount,
            Down(amount) => self.aim += amount
        }
    }
}

fn get_solution_part1(values: &Vec<Move>) -> u32 {
    let mut submarine = DefectSubmarine { horizontal: 0, depth: 0 };
    apply_movements(values, &mut submarine);
    return submarine.horizontal * submarine.depth;
}

fn get_solution_part2(values: &Vec<Move>) -> u32 {
    let mut submarine = WorkingSubmarine { horizontal: 0, depth: 0, aim: 0 };
    apply_movements(values, &mut submarine);
    return submarine.horizontal * submarine.depth;
}

fn apply_movements(values: &Vec<Move>, submarine: &mut dyn Submarine) {
    values.iter().for_each(|value| submarine.drive(value));
}

fn read_input_as_integers<T>(path: &str, parse_method: fn(&str) -> Result<T, &str>) -> Vec<T> {
    let file = File::open(path).expect("not found");
    let reader = BufReader::new(file);

    return reader
        .lines()
        .map(|line| parse_method(line.unwrap().as_str()).unwrap())
        .collect();
}

fn parse_move(move_str: &str) -> Result<Move, &str> {
    let mut move_split = move_str.split(' ');
    let direction = move_split.next().unwrap();
    let amount = move_split.next().unwrap().parse::<u32>().unwrap();

    return match direction {
        "forward" => Ok(Forward(amount)),
        "up" => Ok(Up(amount)),
        "down" => Ok(Down(amount)),
        _ => Err("invalid move")
    };
}

fn main() {
    let part = match env::var("part") {
        Ok(val) => val,
        Err(_e) => "part1".to_string(),
    };

    let values = &read_input_as_integers("input.txt", parse_move);
    println!("Result: {}", match part.as_str() {
        "part2" => get_solution_part2(values),
        _ => get_solution_part1(values),
    });
}

#[cfg(test)]
mod tests {
    use crate::{Down, Forward, get_solution_part1, get_solution_part2, parse_move, Up};

    #[test]
    fn parse_input_str_works() {
        assert_eq!(parse_move("forward 1").unwrap(), Forward(1));
    }

    #[test]
    fn get_solution_part_1_works() {
        assert_eq!(get_solution_part1(&vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)]), 150);
    }

    #[test]
    fn get_solution_part_2_works() {
        assert_eq!(get_solution_part2(&vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)]), 900);
    }
}
