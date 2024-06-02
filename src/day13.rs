use std::hash::{DefaultHasher, Hasher, Hash};

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn find_mirrors(hashes: &[u64]) -> Vec<usize> {
    for i in 0..hashes.len() {

    }
    vec![0]
} 

pub fn part1(input: &str) -> usize {
    let lines_hashes: Vec<u64> = input.lines().map(|l| calculate_hash(&l)).collect();
    find_mirrors(&lines_hashes).into_iter().sum()
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "input"
        // include_str!("../input/2023/day01.txt")
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 11);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 22);
    }
}
