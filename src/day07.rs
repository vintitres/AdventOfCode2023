use std::collections::BTreeMap;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    type_: u8,
    val: u64,
}

impl Hand {
    fn parse(hand: &str) -> Hand {
        let mut handmap: BTreeMap<char, u64> = BTreeMap::new();
        for c in hand.chars() {
            handmap.entry(c).and_modify(|v| *v = *v + 1).or_insert(1);
        }
        let type_ = 
        if *handmap.iter().next().unwrap().1 == 5 {  // five of kind
            0
        } else if handmap.iter().any(|(_, &c)| c == 4) {  // four of kind
            1
        } else if handmap.iter().any(|(_, &c)| c == 3) {
            if handmap.iter().any(|(_, &c)| c == 2) {  // full house
                2
            } else {  // three of a kind
                3
            }
        } else if handmap.iter().filter(|(_, &c)| c == 2).count() == 2 {  // two pair
            4
        } else if handmap.iter().any(|(_, &c)| c == 2) {  // pair
            5
        } else {  // high card
            6
        };
        let val = hand.chars().map(|c| match c {
            'A' => 0,
            'K' => 1,
            'Q' => 2,
            'J' => 3,
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
        }).fold(0, |acc, n| acc * 13 + n);
        Hand { type_, val }
    }
}

pub fn part1(input: &str) -> u64 {
    let mut hands = input
        .lines()
        .map(|l| {
            let (hand, bet) = l.split_once(' ').unwrap();
            let bet = bet.parse::<u64>().unwrap();
            let hand = Hand::parse(hand);
            (hand, bet)
        })
        .collect_vec();
    hands.sort();
    hands
        .iter()
        .rev()
        .enumerate()
        .map(|(rank, (_hand, bet))| (rank as u64 + 1) * bet)
        .map(|v| {dbg!(v); v })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input.len()
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

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 22);
    }
}
