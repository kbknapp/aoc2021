use std::io::{self, BufRead};

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    let input = parse_input();
    println!("part one: {}", part_one(&input, 100, 100));
    println!("part two: {}", part_two(&input, 100, 100));
    Ok(())
}

fn parse_input() -> Vec<u8> {
    let mut v = Vec::with_capacity(10_000);
    io::stdin()
        .lock()
        .lines()
        .for_each(|line| v.extend(line.unwrap().chars().map(|c| c.to_digit(10).unwrap() as u8)));
    v
}

fn part_one(input: &[u8], w: usize, h: usize) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(i, &n)| {
            if let Some(up) = get_adjacent_up(i, w) {
                if n >= input[up] {
                    return 0;
                }
            }
            if let Some(left) = get_adjacent_left(i, w) {
                if n >= input[left] {
                    return 0;
                }
            }
            if let Some(right) = get_adjacent_right(i, w, h) {
                if n >= input[right] {
                    return 0;
                }
            }
            if let Some(down) = get_adjacent_down(i, w, h) {
                if n >= input[down] {
                    return 0;
                }
            }
            n as usize + 1
        })
        .sum()
}

fn part_two(input: &[u8], w: usize, h: usize) -> usize {
    let mut new_input: Vec<_> = input.iter().map(|n| *n != 9).collect();

    let mut basins = Vec::new();
    let mut i = 0;
    while i < new_input.len() {
        if !new_input[i] {
            i += 1;
            continue;
        }
        let end_of_row = (i / w) * w + w;

        let mut cur_basin = 0;
        let start = i;
        for j in start..end_of_row {
            i += 1;
            if !new_input[j] {
                break;
            }

            new_input[j] = false;
            cur_basin += 1;
            if let Some(down) = get_adjacent_down(j, w, h) {
                cur_basin += count_and_mark_adjacent(&mut new_input, down, w, h);
            }
        }
        if cur_basin > 0 {
            basins.push(cur_basin);
        }
    }

    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

fn count_and_mark_adjacent(input: &mut [bool], i: usize, w: usize, h: usize) -> usize {
    if !input[i] {
        return 0;
    }
    input[i] = false;

    1 + get_adjacent_left(i, w)
        .map(|x| count_and_mark_adjacent(input, x as usize, w, h))
        .unwrap_or(0)
        + get_adjacent_down(i, w, h)
            .map(|x| count_and_mark_adjacent(input, x as usize, w, h))
            .unwrap_or(0)
        + get_adjacent_right(i, w, h)
            .map(|x| count_and_mark_adjacent(input, x as usize, w, h))
            .unwrap_or(0)
        + get_adjacent_up(i, w)
            .map(|x| count_and_mark_adjacent(input, x as usize, w, h))
            .unwrap_or(0)
}

fn get_adjacent_up(i: usize, w: usize) -> Option<usize> {
    let up: isize = i as isize - w as isize;
    if up >= 0 {
        Some(up as usize)
    } else {
        None
    }
}

fn get_adjacent_left(i: usize, w: usize) -> Option<usize> {
    let left: isize = i as isize - 1;
    if left >= 0 && left < i as isize && left >= ((i / w) * w) as isize {
        Some(left as usize)
    } else {
        None
    }
}

fn get_adjacent_right(i: usize, w: usize, h: usize) -> Option<usize> {
    let right: usize = i + 1;
    if right < w * h && right > i && right < (i / w) * w + w {
        Some(right)
    } else {
        None
    }
}

fn get_adjacent_down(i: usize, w: usize, h: usize) -> Option<usize> {
    let down = i + w;
    if down < w * h {
        Some(down)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

    fn parse_test_input() -> Vec<u8> {
        TEST_INPUT
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect()
    }

    #[test]
    fn p_one() {
        assert_eq!(part_one(&parse_test_input(), 10, 5), 15);
    }

    #[test]
    fn p_two() {
        assert_eq!(part_two(&parse_test_input(), 10, 5), 1134);
    }
}
