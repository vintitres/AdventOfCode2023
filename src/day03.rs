use core::cmp::min;

use itertools::Itertools;

pub fn part1(input: &str) -> u64 {
    struct State {
        number: u64,
        touched_symbol: bool,
    }
    const EMPTY_STATE: State = State {
        number: 0,
        touched_symbol: false,
    };
    let schem = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .scan(EMPTY_STATE, |state, (j, c)| {
                    let mut r = None;
                    if c.is_ascii_digit() {
                        *state = State {
                            number: state.number * 10 + c.to_digit(10).unwrap() as u64,
                            touched_symbol: state.touched_symbol
                                || (i.saturating_sub(1)..=min(i + 1, schem.len() - 1)).any(|ii| {
                                    (j.saturating_sub(1)..=min(j + 1, schem[ii].len() - 1)).any(
                                        |jj| {
                                            let c = schem[ii][jj];
                                            !c.is_ascii_digit() && c != '.'
                                        },
                                    )
                                }),
                        };
                    }
                    if state.number > 0 && (!c.is_ascii_digit() || j == schem[i].len() - 1) {
                        if state.touched_symbol {
                            r = Some(state.number);
                        }
                        *state = EMPTY_STATE;
                    }
                    Some(r)
                })
                .flatten()
                .sum::<u64>()
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let schem = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .scan((0, None), |(number, touched_star), (j, c)| {
                    let mut n = 0;
                    let mut s: Option<(usize, usize)> = None;
                    if c.is_ascii_digit() {
                        *number *= 10;
                        *number += c.to_digit(10).unwrap() as u64;
                        if touched_star.is_none() {
                            *touched_star = (i.saturating_sub(1)..=min(i + 1, schem.len() - 1))
                                .flat_map(|ii| {
                                    (j.saturating_sub(1)..=min(j + 1, schem[ii].len() - 1))
                                        .flat_map(|jj| {
                                            if schem[ii][jj] == '*' {
                                                Some((ii, jj))
                                            } else {
                                                None
                                            }
                                        })
                                        .next()
                                })
                                .next();
                        }
                    }
                    if *number > 0 && (!c.is_ascii_digit() || j == schem[i].len() - 1) {
                        if touched_star.is_some() {
                            n = *number;
                            s = *touched_star;
                        }
                        // dbg!(*touched_star, *number);
                        *touched_star = None;
                        *number = 0;
                    }
                    if let Some(s) = s {
                        Some(Some((s, n)))
                    } else {
                        Some(None)
                    }
                })
                .flatten()
                .collect_vec()
        })
        .sorted()
        .scan(((0, 0), 0), |last, cur| {
            let mut r = Some(0);
            if last.0 == cur.0 {
                r = Some(last.1 * cur.1);
            }
            *last = cur;
            r
        })
        .sum()
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 83279367);
    }
}
