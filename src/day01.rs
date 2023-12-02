pub fn part1(input: &str) -> u32 {
    input.lines().map(
        |l| {
            let mut digits = l.chars().flat_map(|c| c.to_digit(10));
            let f = digits.next().unwrap();
            let l = digits.last().unwrap_or(f);
            f * 10 + l
        }
    ).sum()
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day1.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 55029);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 22);
    }
}
