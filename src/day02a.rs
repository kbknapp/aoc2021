use color_eyre::eyre::Report;

use aoc21::day02::{parse_input, Direction, Location};

fn run(dirs: &[Direction]) -> i64 {
    let Location { horiz, depth } = dirs.iter().fold(Location::default(), |mut acc, d| {
        match d {
            Direction::Up(i) => acc.depth -= i,
            Direction::Down(i) => acc.depth += i,
            Direction::Forward(i) => acc.horiz += i,
        }
        acc
    });

    horiz * depth
}

fn main() -> Result<()> {
    Ok(println!("part 1: {}", run(&*parse_input())))
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
    fn part_one() {
        assert_eq!(
            run(&INPUT
                .lines()
                .map(|l| l.parse::<Direction>().unwrap())
                .collect::<Vec<Direction>>()),
            150
        );
    }
}
