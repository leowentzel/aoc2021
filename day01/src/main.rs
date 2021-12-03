use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


fn get_solution_part1(values: &Vec<i32>) -> i32 {
    return get_solution_part(values, 1);
}

fn get_solution_part2(values: &Vec<i32>) -> i32 {
    return get_solution_part(values, 3);
}

fn get_solution_part(values: &Vec<i32>, window: usize) -> i32 {
    return values.windows(window)
        .fold((0, None::<&[i32]>), |(count, prev), cur| (match prev {
            Some(prev) if prev.iter().sum::<i32>() < cur.iter().sum() => count + 1,
            _ => count
        }, Some(cur))).0;
}

fn read_input_as_integers(path: &str) -> Vec<i32> {
    let file = File::open(path).expect("not found");
    let reader = BufReader::new(file);

    return reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
}

fn main() {
    let parsed_input = read_input_as_integers("input.txt");
    let part = match env::var("part") {
        Ok(val) => val,
        Err(_e) => "part1".to_string(),
    };

    println!("Result: {}", match part.as_str() {
        "part2" => get_solution_part2(&parsed_input),
        _ => get_solution_part1(&parsed_input),
    });
}

#[cfg(test)]
mod tests {
    use crate::{get_solution_part1, get_solution_part2};

    #[test]
    fn get_solution_part_1_works() {
        assert_eq!(get_solution_part1(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]), 7);
    }

    #[test]
    fn get_solution_part_2_works() {
        assert_eq!(get_solution_part2(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]), 5);
    }
}
