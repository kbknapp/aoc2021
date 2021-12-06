use std::{
    fmt,
    io::{self, BufRead},
    mem,
    result::Result as StdResult,
    str::FromStr,
};

use color_eyre::eyre::Result;

#[derive(Debug)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    #[inline]
    fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    #[inline]
    fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ();
    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        Ok(Self { x, y })
    }
}

struct Diagram {
    size: usize,
    matrix: Vec<usize>,
}

impl Diagram {
    fn new(size: usize) -> Self {
        Self {
            size,
            matrix: vec![0; size * size], // 0 is special cased for fast allocation
        }
    }

    fn plot_lines(&mut self, lines: Vec<Line>) {
        for line in lines {
            let mut start = line.a.x + line.a.y * self.size;
            let mut end = line.b.x + line.b.y * self.size;
            let mut swapped = false;
            if start > end {
                mem::swap(&mut start, &mut end);
                swapped = true;
            }
            let step = if line.is_horizontal() {
                1
            } else if line.is_vertical() {
                self.size
            } else if (line.a.x < line.b.x && !swapped) || (line.a.x > line.b.x && swapped) {
                self.size + 1
            } else {
                self.size - 1
            };
            (start..=end)
                .step_by(step)
                .for_each(|cell| self.matrix[cell] += 1)
        }
    }

    fn plot_most_dangerous(&self) -> usize {
        self.matrix.iter().filter(|c| **c > 1).count()
    }
}

impl fmt::Debug for Diagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = writeln!(f);
        for (i, cell) in self.matrix.iter().enumerate() {
            if i > 0 && i % self.size == 0 {
                let _ = writeln!(f);
            }
            if *cell == 0 {
                let _ = write!(f, ".");
            } else {
                let _ = write!(f, "{}", self.matrix[i]);
            }
        }
        let _ = writeln!(f);
        Ok(())
    }
}

fn parse_input() -> Vec<Line> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let mut parts = line.as_ref().unwrap().split_ascii_whitespace();
            let a = parts.next().unwrap().parse().unwrap();
            let _ = parts.next();
            let b = parts.next().unwrap().parse().unwrap();
            Line { a, b }
        })
        .collect()
}

fn main() -> Result<()> {
    let mut d = Diagram::new(1000);
    d.plot_lines(parse_input());
    println!("Answer: {}", d.plot_most_dangerous());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const PART_ONE: &str = "\
0,9 -> 5,9
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
0,9 -> 2,9
3,4 -> 1,4";

    const PART_TWO: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    const TL_BR: &str = "\
0,0 -> 3,3";
    const TR_BL: &str = "\
3,0 -> 0,3";
    const BL_TR: &str = "\
0,3 -> 3,0";
    const BR_TL: &str = "\
3,3 -> 0,0";
    const VERT_UD: &str = "\
0,0 -> 0,3";
    const VERT_DU: &str = "\
0,3 -> 0,0";
    const HORI_LR: &str = "\
0,2 -> 3,2";
    const HORI_RL: &str = "\
3,2 -> 0,2";

    fn parse_test_input(input: &str) -> Vec<Line> {
        input
            .lines()
            .map(|line| {
                let mut parts = line.split_ascii_whitespace();
                let a = parts.next().unwrap().parse().unwrap();
                let _ = parts.next();
                let b = parts.next().unwrap().parse().unwrap();
                Line { a, b }
            })
            .collect()
    }

    #[test]
    fn part_one() {
        let mut d = Diagram::new(10);
        d.plot_lines(parse_test_input(PART_ONE));
        assert_eq!(d.plot_most_dangerous(), 5, "{:?}", d);
    }

    #[test]
    fn vert_ud() {
        let mut d = Diagram::new(4);
        d.plot_lines(parse_test_input(VERT_UD));
        assert_eq!(
            d.matrix,
            &[1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
            "{:?}",
            d
        );
    }
    #[test]
    fn vert_du() {
        let mut d = Diagram::new(4);
        d.plot_lines(parse_test_input(VERT_DU));
        assert_eq!(
            d.matrix,
            &[1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
            "{:?}",
            d
        );
    }

    #[test]
    fn hori_lr() {
        let mut d = Diagram::new(4);
        d.plot_lines(parse_test_input(HORI_LR));
        assert_eq!(
            d.matrix,
            &[0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
            "{:?}",
            d
        );
    }

    #[test]
    fn hori_rl() {
        let mut d = Diagram::new(4);
        d.plot_lines(parse_test_input(HORI_RL));
        assert_eq!(
            d.matrix,
            &[0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
            "{:?}",
            d
        );
    }

    #[test]
    fn part_two() {
        let mut d = Diagram::new(10);
        d.plot_lines(parse_test_input(PART_TWO));
        assert_eq!(d.plot_most_dangerous(), 12, "{:?}", d);
    }

    #[test]
    fn diag_tl_br() {
        let mut d = Diagram::new(4);
        d.plot_lines(parse_test_input(TL_BR));
        assert_eq!(
            d.matrix,
            &[1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
            "{:?}",
            d
        );
    }

    #[test]
    fn diag_br_tl() {
        let mut d = Diagram::new(4);
        d.plot_lines(parse_test_input(BR_TL));
        assert_eq!(
            d.matrix,
            &[1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
            "{:?}",
            d
        );
    }

    #[test]
    fn diag_tr_bl() {
        let mut d = Diagram::new(4);
        d.plot_lines(parse_test_input(TR_BL));
        assert_eq!(
            d.matrix,
            &[0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0],
            "{:?}",
            d
        );
    }

    #[test]
    fn diag_bl_tr() {
        let mut d = Diagram::new(4);
        d.plot_lines(parse_test_input(BL_TR));
        assert_eq!(
            d.matrix,
            &[0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0],
            "{:?}",
            d
        );
    }
}
