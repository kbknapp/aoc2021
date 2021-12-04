use color_eyre::eyre::Result;

use aoc21::day02::{parse_input, Direction, Location};

fn run(dirs: &[Direction]) -> i64 {
    let Location { horiz, depth, .. } = dirs.iter().fold(Location::default(), |mut acc, d| {
        match d {
            Direction::Up(i) => acc.aim -= i,
            Direction::Down(i) => acc.aim += i,
            Direction::Forward(i) => {
                acc.horiz += i;
                acc.depth += acc.aim * i;
            }
        }
        acc
    });

    horiz * depth
}

fn main() -> Result<()> {
    println!("part 2: {}", run(&*parse_input()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn part_two() {
        assert_eq!(
            run(&INPUT
                .lines()
                .map(|l| l.parse::<Direction>().unwrap())
                .collect::<Vec<Direction>>(),),
            900
        );
    }
}
