use std::io::{self, BufRead};

use color_eyre::eyre::Result;

fn parse_input() -> Vec<u64> {
    let mut v = vec![0; 9];
    io::stdin().lock().lines().for_each(|line| {
        line.unwrap()
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .for_each(|n| v[n] += 1);
    });

    v
}

fn multiply(v: &mut Vec<u64>) {
    assert!(v.len() >= 9);
    let new_fish = v[0];
    for i in 0..8 {
        v[i] = v[i + 1];
    }
    v[8] = new_fish;
    v[6] += new_fish;
}

fn main() -> Result<()> {
    let mut school = parse_input();
    for _ in 0..256 {
        multiply(&mut school);
    }

    println!("part 2: {}", school.iter().sum::<u64>());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse_test_input(s: &str) -> Vec<u64> {
        let mut v = vec![0; 9];
        s.split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .for_each(|n| v[n] += 1);

        v
    }

    #[test]
    fn part_one() {
        let mut school = parse_test_input("3,4,3,1,2");
        for _ in 0..80 {
            multiply(&mut school);
        }
        assert_eq!(school.iter().sum::<u64>(), 5934);
    }

    #[test]
    fn part_two() {
        let mut school = parse_test_input("3,4,3,1,2");
        for _ in 0..256 {
            multiply(&mut school);
        }
        assert_eq!(school.iter().sum::<u64>(), 26984457539);
    }
}
