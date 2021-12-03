use std::io::{self, BufRead};

fn parse_input() -> Vec<i64> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| i64::from_str_radix(&line.unwrap(), 2).unwrap())
        .collect()
}

fn count_ones_zeros(input: &[i64], bit: usize) -> (usize, usize) {
    let ones = input.iter().filter(|n| *n & 1 << bit == 1 << bit).count();
    (ones, input.len() - ones)
}

fn ones_most_common(input: &[i64], bit: usize) -> bool {
    let (ones, zeros) = count_ones_zeros(input, bit);
    ones >= zeros
}

fn ones_least_common(input: &[i64], bit: usize) -> bool {
    let (ones, zeros) = count_ones_zeros(input, bit);
    ones < zeros
}

fn run_part_one(input: &[i64], bits: usize) -> i64 {
    let (gamma, epsi) = (0..bits).rev().fold((0, 0), |(mut g, mut e), bit| {
        if ones_most_common(input, bit) {
            g |= 1 << bit;
        } else {
            e |= 1 << bit;
        }
        (g, e)
    });

    gamma * epsi
}

fn run_part_two(input: &[i64], bits: usize) -> i64 {
    let o2 = find_o2(input.to_vec(), bits - 1);
    let co2 = find_co2(input.to_vec(), bits - 1);

    o2[0] * co2[0]
}

fn find_o2(input: Vec<i64>, bit: usize) -> Vec<i64> {
    let is_ones = ones_most_common(&input, bit);
    let smaller = subset(input, is_ones, bit);
    if smaller.len() == 1 {
        return smaller;
    }
    find_o2(smaller, bit - 1)
}

fn find_co2(input: Vec<i64>, bit: usize) -> Vec<i64> {
    let ones = ones_least_common(&input, bit);
    let smaller = subset(input, ones, bit);
    if smaller.len() == 1 {
        return smaller;
    }
    find_co2(smaller, bit - 1)
}

fn subset(input: Vec<i64>, ones: bool, bit: usize) -> Vec<i64> {
    if ones {
        input
            .iter()
            .filter(|n| *n & 1 << bit == 1 << bit)
            .map(|n| *n)
            .collect()
    } else {
        input
            .iter()
            .filter(|n| (*n ^ 1 << bit) & 1 << bit == 1 << bit)
            .map(|n| *n)
            .collect()
    }
}

fn main() {
    let input = parse_input();
    println!("part 1: {}", run_part_one(&input, 12));
    println!("part 2: {}", run_part_two(&input, 12));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    fn parse_test_input(input: &str) -> Vec<i64> {
        input
            .lines()
            .map(|line| i64::from_str_radix(&line, 2).unwrap())
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(run_part_one(&parse_test_input(TEST_INPUT), 5), 198);
    }

    #[test]
    fn test_part2() {
        let inp = parse_test_input(TEST_INPUT);
        assert_eq!(run_part_two(&inp, 5), 230);
    }
}
