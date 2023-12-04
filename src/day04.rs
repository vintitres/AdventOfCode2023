use itertools::Itertools;
use std::collections::HashSet;

fn read_cards(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| {
            let (win, my) = l
                .split(": ")
                .nth(1)
                .unwrap()
                .split(" | ")
                .map(|nums| {
                    HashSet::<u64>::from_iter(nums.split(' ').flat_map(|n| n.parse::<u64>()))
                })
                .collect_tuple()
                .unwrap();
            my.intersection(&win).collect_vec().len()
        })
        .collect_vec()
}

pub fn part1(input: &str) -> u64 {
    read_cards(input).iter().map(|l| if *l == 0 { 0 } else { 1u64 << (l - 1) }).sum()
}

pub fn part2(input: &str) -> usize {
    let cards = read_cards(input);
    let mut owned_cards = cards.iter().map(|_| 1).collect_vec();
    for card in 0..owned_cards.len() {
        for new_card in (card + 1)..(card + 1 + cards[card]) {
            owned_cards[new_card] += owned_cards[card];
        }
    }
    owned_cards.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day4.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 23441);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 5923918);
    }
}
