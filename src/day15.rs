fn reindeer_hash(word: &str) -> u64 {
    word.chars()
        .filter(|c| *c != '\n')
        .fold(0, |acc, c| ((acc + c as u64) * 17) % 256)
}

pub fn part1(input: &str) -> u64 {
    input.split(',').map(reindeer_hash).sum()
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day15.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 503487);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 22);
    }
}
