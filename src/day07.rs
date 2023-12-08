use std::collections::BTreeMap;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    type_: u8,
    val: u64,
}

impl Hand {
    fn parse(hand: &str, jokers: bool) -> Hand {
        let mut handmap: BTreeMap<char, usize> = BTreeMap::new();
        for c in hand.chars() {
            handmap.entry(c).and_modify(|v| *v = *v + 1).or_insert(1);
        }
        let jokers_count = if jokers {
            handmap.remove(&'J').unwrap_or(0)
        } else {
            0
        };
        let type_ = if jokers_count == 5 || *handmap.iter().next().unwrap().1 + jokers_count == 5 {
            // five of kind
            0
        } else if jokers_count == 4 || handmap.iter().any(|(_, &c)| c == 4 - jokers_count) {
            // four of kind
            1
        } else if jokers_count == 3 || handmap.iter().any(|(_, &c)| c == 3 - jokers_count) {
            if handmap.iter().any(|(_, &c)| c == 2)
                && (jokers_count != 1 || handmap.iter().filter(|(_, &c)| c == 2).count() == 2)
            {
                // full house
                2
            } else {
                // three of a kind
                3
            }
        } else if handmap.iter().filter(|(_, &c)| c == 2).count() >= 2 - jokers_count {
            // two pair
            4
        } else if jokers_count == 1 || handmap.iter().any(|(_, &c)| c == 2) {
            // pair
            5
        } else {
            // high card
            6
        };
        let val = hand
            .chars()
            .map(|c| match c {
                'A' => 0,
                'K' => 1,
                'Q' => 2,
                'J' => {
                    if jokers {
                        13
                    } else {
                        3
                    }
                }
                'T' => 4,
                '9' => 5,
                '8' => 6,
                '7' => 7,
                '6' => 8,
                '5' => 9,
                '4' => 10,
                '3' => 11,
                '2' => 12,
                _ => unimplemented!(),
            })
            .fold(0, |acc, n| acc * 14 + n);
        Hand { type_, val }
    }
}

fn doit(input: &str, jokers: bool) -> u64 {
    let mut hands = input
        .lines()
        .map(|l| {
            let (hand, bet) = l.split_once(' ').unwrap();
            let bet = bet.parse::<u64>().unwrap();
            let hand = Hand::parse(hand, jokers);
            (hand, bet)
        })
        .collect_vec();
    hands.sort();
    hands
        .iter()
        .rev()
        .enumerate()
        .map(|(rank, (_hand, bet))| (rank as u64 + 1) * bet)
        .map(|v| {
            dbg!(v);
            v
        })
        .sum()
}

pub fn part1(input: &str) -> u64 {
    doit(input, false)
}

pub fn part2(input: &str) -> u64 {
    doit(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day7.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 251287184);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 250757288);
    }
}
