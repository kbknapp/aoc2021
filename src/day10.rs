use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

#[derive(Clone, Debug, PartialEq)]
enum ErrorKind {
    // Expected, Found
    Illegal(BraceKind, BraceKind),
    Incomplete(Vec<BraceKind>),
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum BraceKind {
    Paren,
    Sqr,
    Angl,
    Curl,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Brace {
    Open(BraceKind),
    Close(BraceKind),
}

impl From<char> for Brace {
    fn from(c: char) -> Self {
        match c {
            '(' => Brace::Open(BraceKind::Paren),
            ')' => Brace::Close(BraceKind::Paren),
            '[' => Brace::Open(BraceKind::Sqr),
            ']' => Brace::Close(BraceKind::Sqr),
            '{' => Brace::Open(BraceKind::Curl),
            '}' => Brace::Close(BraceKind::Curl),
            '<' => Brace::Open(BraceKind::Angl),
            '>' => Brace::Close(BraceKind::Angl),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = parse_input();
    println!("part one: {}", part_one(&*input));
    println!("part two: {}", part_two(&*input));
}

fn parse_input() -> Vec<Vec<char>> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .collect()
}

fn part_one(input: &[Vec<char>]) -> usize {
    use BraceKind::*;
    let mut e_paren = 0;
    let mut e_ang = 0;
    let mut e_sqr = 0;
    let mut e_curl = 0;
    for v in input {
        match find_errs(v) {
            Ok(_) => (),
            Err(ErrorKind::Illegal(_bo, bc)) => match bc {
                Paren => e_paren += 1,
                Angl => e_ang += 1,
                Sqr => e_sqr += 1,
                Curl => e_curl += 1,
            },
            _ => (),
        }
    }

    e_paren * 3 + e_ang * 25137 + e_sqr * 57 + e_curl * 1197
}

fn find_errs(v: &[char]) -> Result<(), ErrorKind> {
    use Brace::*;
    use BraceKind::*;
    let mut vd = VecDeque::new();
    for c in v {
        match Brace::from(*c) {
            Open(bo) => vd.push_back(bo),
            Close(bc) => {
                if let Some(bo) = vd.pop_back() {
                    if bo != bc {
                        match bo {
                            Paren => return Err(ErrorKind::Illegal(bo, bc)),
                            Angl => return Err(ErrorKind::Illegal(bo, bc)),
                            Sqr => return Err(ErrorKind::Illegal(bo, bc)),
                            Curl => return Err(ErrorKind::Illegal(bo, bc)),
                        }
                    }
                }
            }
        }
    }

    Err(ErrorKind::Incomplete(vd.into_iter().rev().collect()))
}

fn part_two(input: &[Vec<char>]) -> usize {
    let mut scores: Vec<_> = input
        .iter()
        .map(|v| find_errs(v))
        .filter(|e| e.is_err())
        .filter_map(|e| match e {
            Err(ErrorKind::Incomplete(v)) => Some(incomplete_score(&v)),
            _ => None,
        })
        .collect();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn incomplete_score(v: &[BraceKind]) -> usize {
    use BraceKind::*;
    v.iter().fold(0, |mut acc, b| {
        acc *= 5;
        match b {
            Paren => acc += 1,
            Sqr => acc += 2,
            Curl => acc += 3,
            Angl => acc += 4,
        }
        acc
    })
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    fn parse_test_input() -> Vec<Vec<char>> {
        TEST_INPUT
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect()
    }

    #[test]
    fn missing_angled() {
        assert_eq!(
            find_errs(&"[<(<(<(<{}))><([]([]()".chars().collect::<Vec<_>>()),
            Err(ErrorKind::Illegal(BraceKind::Angl, BraceKind::Paren))
        );
    }

    #[test]
    fn missing_paren() {
        assert_eq!(
            find_errs(&"[{[{({}]{}}([{[{{{}}([]".chars().collect::<Vec<_>>()),
            Err(ErrorKind::Illegal(BraceKind::Paren, BraceKind::Sqr))
        );
    }

    #[test]
    fn missing_sq() {
        assert_eq!(
            find_errs(&"{([(<{}[<>[]}>{[]{[(<()>".chars().collect::<Vec<_>>()),
            Err(ErrorKind::Illegal(BraceKind::Sqr, BraceKind::Curl))
        );
        assert_eq!(
            find_errs(&"[[<[([]))<([[{}[[()]]]".chars().collect::<Vec<_>>()),
            Err(ErrorKind::Illegal(BraceKind::Sqr, BraceKind::Paren))
        );
        assert_eq!(
            find_errs(&"<{([([[(<>()){}]>(<<{{".chars().collect::<Vec<_>>()),
            Err(ErrorKind::Illegal(BraceKind::Sqr, BraceKind::Angl))
        );
    }

    #[test]
    fn complete() {
        assert_eq!(
            find_errs(&"[({(<(())[]>[[{[]{<()<>>".chars().collect::<Vec<_>>()),
            Err(ErrorKind::Incomplete(
                "}}]])})]"
                    .chars()
                    .map(|c| {
                        match Brace::from(c) {
                            Brace::Open(b) => b,
                            Brace::Close(b) => b,
                        }
                    })
                    .collect()
            ))
        );
    }

    #[test]
    fn p_one() {
        assert_eq!(part_one(&*parse_test_input()), 26397);
    }
    #[test]
    fn p_two() {
        assert_eq!(part_two(&*parse_test_input()), 288957);
    }
}
