use std::io::{self, BufRead};

use color_eyre::eyre::Result;

fn parse_input() -> Vec<usize> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect()
}

fn run_part_one(input: &[usize]) -> usize {
    input.windows(2).filter(|win| win[0] < win[1]).count()
}

fn run_part_two(input: &[usize]) -> usize {
    run_part_one(
        &input
            .windows(3)
            .map(|s| s[0] + s[1] + s[2])
            .collect::<Vec<_>>(),
    )
}

fn main() -> Result<()> {
    let input = parse_input();

    println!("part 1: {}", run_part_one(&input));
    println!("part 2: {}", run_part_two(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn part_one() {
        assert_eq!(
            run_part_one(
                &INPUT
                    .lines()
                    .map(|l| l.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            ),
            7
        );
    }

    #[test]
    fn part_two() {
        assert_eq!(
            run_part_two(
                &INPUT
                    .lines()
                    .map(|l| l.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            ),
            5
        );
    }
}
