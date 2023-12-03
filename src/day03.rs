use core::cmp::min;

use itertools::Itertools;

pub fn part1(input: &str) -> u64 {
    let schem = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .scan((0, false), |(number, touched_symbol), (j, c)| {
                    let mut n = 0;
                    if c.is_ascii_digit() {
                        *number *= 10;
                        *number += c.to_digit(10).unwrap() as u64;
                        if !*touched_symbol {
                            *touched_symbol = (i.saturating_sub(1)..=min(i + 1, schem.len() - 1))
                                .any(|ii| {
                                    (j.saturating_sub(1)..=min(j + 1, schem[ii].len() - 1)).any(
                                        |jj| {
                                            let c = schem[ii][jj];
                                            !c.is_ascii_digit() && c != '.'
                                        },
                                    )
                                });
                        }
                    }
                    if *number > 0 && (!c.is_ascii_digit() || j == schem[i].len() - 1) {
                        if *touched_symbol {
                            n = *number;
                        }
                        dbg!(*touched_symbol, *number);
                        *touched_symbol = false;
                        *number = 0;
                    }
                    if n > 0 {
                        Some(Some(n))
                    } else {
                        Some(None)
                    }
                })
                .flatten()
                .sum::<u64>()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day3.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 531561);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 22);
    }
}
