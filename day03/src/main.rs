use std::{env, usize};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::ops::BitXor;

fn get_solution_part1<R: BufRead>(reader: &mut R) -> u32 {
    let mut count_vec = Vec::new();
    let mut line_count = 0u32;
    for line in reader.lines() {
        line_count += 1;
        for (i, val) in line.unwrap().chars().enumerate() {
            let digit = val.to_digit(10).unwrap();
            if count_vec.len() <= i { count_vec.push(digit) } else { count_vec[i] += digit }
        }
    };

    let gamma_string = count_vec.iter()
        .map(|&value| (if line_count / value == 1 { 1 } else { 0 }).to_string())
        .collect::<Vec<_>>().concat();

    let gamma = u32::from_str_radix(gamma_string.as_str(), 2).unwrap();
    let epsilon = (u32::MAX >> gamma.leading_zeros()) ^ gamma;
    return gamma * epsilon;
}

fn get_solution_part2<R: BufRead>(reader: &mut R) -> u32 {
    let mut values = Vec::new();
    for line in reader.lines() {
        let mut val_vec = line.unwrap().chars().into_iter()
            .map(|c| c.to_digit(2).unwrap() == 1).collect::<Vec<_>>();
        values.push(val_vec);
    }

    let mut oxy_values = Vec::from(values.as_slice());
    let mut position = 0usize;
    while oxy_values.len() > 1 {
        let mut count = 0u32;
        let remaining_value_count = oxy_values.len() as u32;
        for mut val_vec in &oxy_values {
            count += val_vec[position] as u32;
        }
        oxy_values.retain(|val_vec| val_vec[position] == (2 * count >= remaining_value_count));
        position += 1;
    }

    let mut scrub_values = Vec::from(values.as_slice());
    let mut position = 0usize;
    while scrub_values.len() > 1 {
        let mut count = 0u32;
        let remaining_value_count = scrub_values.len() as u32;
        for mut val_vec in &scrub_values {
            count += val_vec[position] as u32;
        }
        scrub_values.retain(|val_vec| val_vec[position] == (2 * count < remaining_value_count));
        position += 1;
    }

    let oxy_rating_string = oxy_values.concat().iter().map(|&b| (b as u32).to_string()).collect::<Vec<_>>().concat();
    let co2_scrub_rating_string = scrub_values.concat().iter().map(|&b| (b as u32).to_string()).collect::<Vec<_>>().concat();

    return u32::from_str_radix(oxy_rating_string.as_str(), 2).unwrap() *
        u32::from_str_radix(co2_scrub_rating_string.as_str(), 2).unwrap();
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

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::{get_solution_part1, get_solution_part2};

    #[test]
    fn get_solution_part_1_works() {
        let input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010".as_bytes();
        assert_eq!(get_solution_part1(&mut BufReader::new(input)), 198);
    }

    #[test]
    fn get_solution_part_2_works() {
        let input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010".as_bytes();
        assert_eq!(get_solution_part2(&mut BufReader::new(input)), 230);
    }
}
