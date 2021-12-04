use std::io::{self, BufRead};

use color_eyre::eyre::Result;

#[derive(Default)]
struct Board {
    inner: Vec<Vec<Num>>,
    winner: bool,
}

impl Board {
    fn mark(&mut self, n: usize) {
        self.inner
            .iter_mut()
            .flatten()
            .filter(|col| col.n == n)
            .for_each(|col| col.marked = true);
    }

    fn is_winner(&self) -> bool {
        self.inner.iter().any(|r| r.iter().all(|n| n.marked))
            || (0..5).any(|i| self.inner.iter().all(|r| r[i].marked))
    }

    fn score(&self) -> usize {
        self.inner
            .iter()
            .flatten()
            .filter(|n| !n.marked)
            .map(|n| n.n)
            .sum()
    }
}

struct Num {
    n: usize,
    marked: bool,
}

impl Num {
    fn new(n: usize) -> Self {
        Self { n, marked: false }
    }
}

fn parse_input() -> (Vec<usize>, Vec<Board>) {
    let mut turns = vec![];
    let mut boards = vec![];
    let mut first = Some(());
    let mut curr_board = Some(Board::default());
    let mut row = 1;

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        } else if first.take().is_some() {
            turns = line.split(',').map(|n| n.parse().unwrap()).collect();
            continue;
        }

        if let Some(b) = curr_board.as_mut() {
            b.inner.push(
                line.split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .map(Num::new)
                    .collect(),
            )
        }

        row += 1;
        if row > 5 {
            boards.push(curr_board.replace(Board::default()).unwrap());
            row = 1;
        }
    }

    (turns, boards)
}

fn main() -> Result<()> {
    let (turns, mut boards) = parse_input();
    let mut last;
    let mut scores = vec![];
    for turn in turns {
        last = turn;
        for board in &mut boards {
            board.mark(turn);
            if !board.winner && board.is_winner() {
                board.winner = true;
                let score = board.score();
                scores.push(score * last);
            }
        }
    }
    println!("Part 1: {:?}", scores.get(0));
    println!("Part 2: {:?}", scores.iter().last());
    Ok(())
}
