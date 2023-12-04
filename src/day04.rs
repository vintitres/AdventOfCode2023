use itertools::Itertools;
use std::collections::HashSet;

pub fn part1(input: &str) -> u64 {
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
            let mywin = my.intersection(&win).collect_vec();
            let l = mywin.len();
            if l == 0 {
                0
            } else {
                2u64.pow(l.saturating_sub(1) as u32)
            }
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
        include_str!("../input/2023/day4.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 23441);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 22);
    }
}
