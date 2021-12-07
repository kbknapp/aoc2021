# Advent of Code 2021

| Day | Instructions | Inputs | Code |
| :-: | :- | :- | :- |
| 1 |[doc/day01.txt](./doc/day01.txt) |[inputs/day01.txt](./inputs/day01.txt) | [src/day01.rs](./src/day01.rs) |
| 2 |[doc/day02.txt](./doc/day02.txt) |[inputs/day02.txt](./inputs/day02.txt) | [src/day02a.rs](./src/day02a.rs) |
| 2 |[doc/day02.txt](./doc/day02.txt) |[inputs/day02.txt](./inputs/day02.txt) | [src/day02b.rs](./src/day02b.rs) |
| 3 |[doc/day03.txt](./doc/day03.txt) |[inputs/day03.txt](./inputs/day03.txt) | [src/day03.rs](./src/day03.rs) |
| 4 |[doc/day04.txt](./doc/day04.txt) |[inputs/day04.txt](./inputs/day04.txt) | [src/day04.rs](./src/day04.rs) |
| 5 |[doc/day05.txt](./doc/day05.txt) |[inputs/day05.txt](./inputs/day05.txt) | [src/day05.rs](./src/day05.rs) |
| 6 |[doc/day06.txt](./doc/day06.txt) |[inputs/day06.txt](./inputs/day06.txt) | [src/day06.rs](./src/day06.rs) |
| 7 |[doc/day07.txt](./doc/day07.txt) |[inputs/day07.txt](./inputs/day07.txt) | [src/day07.rs](./src/day07.rs) |

## Scenario

You're minding your own business on a ship at sea when the overboard alarm goes
off! You rush to see if you can help. Apparently, one of the Elves tripped and
accidentally sent the sleigh keys flying into the ocean!

Before you know it, you're inside a submarine the Elves keep ready for
situations like this. It's covered in Christmas lights (because of course it
is), and it even has an experimental antenna that should be able to track the
keys if you can boost its signal strength high enough; there's a little meter
that indicates the antenna's signal strength by displaying 0-50 stars.

Your instincts tell you that in order to save Christmas, you'll need to get all
fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each
day in the Advent calendar; the second puzzle is unlocked when you complete the
first. Each puzzle grants one star. Good luck!

## Custom Constraints

The end goal is to run all days (both parts) in under 1 second.

Using [`hyperfine`](https://github.com/sharkdp/hyperfine) to run the naive [`runall.sh`](./runall.sh) script and take the mean time.

This is unscientific as there is no thought going in to optimizing the parsing
of inputs, it's just a naive `cat` to `stdin`.

As of `day07`:

```
$ cargo clean
$ cargo build -q --release
$ hyperfine ./runall.sh
Benchmark 1: ./runall.sh
  Time (mean ± σ):      34.3 ms ±   9.8 ms    [User: 19.0 ms, System: 26.1 ms]
  Range (min … max):    11.1 ms …  49.3 ms    244 runs
```
