use std::{
    io::{self, BufRead},
    str::FromStr,
};

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    let input = parse_input();
    println!("part 1: {}", part_one(&input));
    println!("part 2: {}", part_two(&input));

    Ok(())
}

fn parse_input() -> Vec<RawInput> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<RawInput>().unwrap())
        .collect()
}

struct RawInput {
    signals: Vec<Vec<char>>,
    outputs: Vec<Vec<char>>,
}

impl FromStr for RawInput {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('|');

        Ok(Self {
            signals: parts
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| s.chars().collect())
                .collect(),
            outputs: parts
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| s.chars().collect())
                .collect(),
        })
    }
}

fn part_one(input: &[RawInput]) -> usize {
    input
        .iter()
        .flat_map(|ri| ri.outputs.iter().map(|s| s.len()))
        .filter(|c| matches!(c, 2 | 4 | 3 | 7))
        .count()
}

fn part_two(input: &[RawInput]) -> usize {
    input
        .iter()
        .map(|ri| {
            let n = Numeral::new(&*ri.signals);
            ri.outputs
                .iter()
                .map(|o| n.decode(o))
                .map(|n| format!("{}", n))
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

// Find the char from 'a' that is missing from 'b'
fn diff(a: &[char], b: &[char]) -> char {
    *a.iter().find(|c| !b.contains(c)).unwrap()
}
// true if 'a' is missing at least one from 'b'
fn missing_any(a: &[char], b: &[char]) -> bool {
    b.iter().any(|c| !a.contains(c))
}
// returns the char that is shared between 'a' and 'b'
fn shared(a: &[char], b: &[char]) -> char {
    *a.iter().find(|c| b.contains(c)).unwrap()
}

//  aaaa
// b    c
// b    c
//  dddd
// e    f
// e    f
//  gggg
#[derive(Default, Debug)]
struct Numeral {
    a: Option<char>,
    b: Option<char>,
    c: Option<char>,
    d: Option<char>,
    e: Option<char>,
    f: Option<char>,
    g: Option<char>,
}

impl Numeral {
    fn new(signals: &[Vec<char>]) -> Self {
        let one = signals.iter().find(|v| v.len() == 2).unwrap();
        let four = signals.iter().find(|v| v.len() == 4).unwrap();
        let seven = signals.iter().find(|v| v.len() == 3).unwrap();
        let eight = signals.iter().find(|v| v.len() == 7).unwrap();
        let six = signals
            .iter()
            .filter(|v| v.len() == 6)
            .find(|v| missing_any(v, seven))
            .unwrap();

        let nine = signals
            .iter()
            .filter(|v| v.len() == 6)
            .find(|v| !missing_any(v, four))
            .unwrap();
        let zero = signals
            .iter()
            .filter(|v| v.len() == 6)
            .filter(|v| missing_any(v, four))
            .find(|v| !missing_any(v, one))
            .unwrap();

        let mut numeral = Numeral {
            a: Some(diff(seven, one)),
            c: Some(diff(one, six)),
            d: Some(diff(eight, zero)),
            e: Some(diff(eight, nine)),
            f: Some(shared(one, six)),
            ..Default::default()
        };

        let custom = [numeral.c.unwrap(), numeral.d.unwrap(), numeral.f.unwrap()];
        numeral.b = Some(
            *four
                .iter()
                .find(|c| !&custom.contains(c))
                .unwrap_or_else(|| custom.iter().find(|c| !four.contains(c)).unwrap()),
        );

        numeral.g = Some(diff(
            eight,
            &[
                numeral.a.unwrap(),
                numeral.b.unwrap(),
                numeral.c.unwrap(),
                numeral.d.unwrap(),
                numeral.e.unwrap(),
                numeral.f.unwrap(),
            ],
        ));
        numeral
    }

    fn decode(&self, coded: &[char]) -> usize {
        match (
            coded.contains(&self.a.unwrap()),
            coded.contains(&self.b.unwrap()),
            coded.contains(&self.c.unwrap()),
            coded.contains(&self.d.unwrap()),
            coded.contains(&self.e.unwrap()),
            coded.contains(&self.f.unwrap()),
            coded.contains(&self.g.unwrap()),
        ) {
            (true, true, true, false, true, true, true) => 0,
            (false, false, true, false, false, true, false) => 1,
            (true, false, true, true, true, false, true) => 2,
            (true, false, true, true, false, true, true) => 3,
            (false, true, true, true, false, true, false) => 4,
            (true, true, false, true, false, true, true) => 5,
            (true, false, true, false, false, true, false) => 7,
            (true, true, false, true, true, true, true) => 6,
            (true, true, true, true, true, true, true) => 8,
            (true, true, true, true, false, true, true) => 9,
            _ => {
                unreachable!()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    static EXAMPLE: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    static TEST_INPUT: &str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    fn parse_test_input(input: &str) -> Vec<RawInput> {
        input
            .lines()
            .map(|line| line.parse::<RawInput>().unwrap())
            .collect()
    }

    #[test]
    fn p_one() {
        let inputs = parse_test_input(TEST_INPUT);
        assert_eq!(part_one(&inputs), 26);
    }

    #[test]
    fn p_two_example() {
        let inputs = parse_test_input(EXAMPLE);
        assert_eq!(part_two(&inputs), 5353);
    }
    #[test]
    fn p_two() {
        let inputs = parse_test_input(TEST_INPUT);
        assert_eq!(part_two(&inputs), 61229);
    }
}
