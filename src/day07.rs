#![feature(int_abs_diff)]

use std::io::{self, BufRead};

use color_eyre::eyre::Result;

fn parse_input() -> Vec<usize> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn main() -> Result<()> {
    let input = parse_input();
    println!("part 1: {}", part_one(&*input));
    println!("part 2: {}", part_two(&*input));
    Ok(())
}

fn part_one(input: &[usize]) -> usize {
    // Alternative method vai "brute force" which on these inputs is just as fast since you have to
    // traverse the input multiple times to find the mean value anyways
    //
    // (0..input.len()).map(|i|input.iter().map(|c|(0..=usize::abs_diff(*casusize,i)).sum()).sum()).min().unwrap();

    let mean_val = input
        .iter()
        .map(|i| input.iter().filter(|j| *i == **j).count() - 1)
        .max()
        .unwrap();
    input
        .iter()
        .map(|c| usize::abs_diff(*c as usize, mean_val))
        .sum::<usize>()
}

fn part_two(input: &[usize]) -> usize {
    let half_way = input.len() as f64 / 2.0;
    input
        .iter()
        .map(|c| (0..=usize::abs_diff(*c, half_way as usize)).sum::<usize>())
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;
    static TEST_INPUT: [usize; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn p_one() {
        assert_eq!(part_one(&TEST_INPUT), 37);
    }

    #[test]
    fn p_two() {
        assert_eq!(part_two(&TEST_INPUT), 168);
    }
}
