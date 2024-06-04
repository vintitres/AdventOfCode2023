pub fn part1(input: &str) -> u64 {
    let mut load = 0;
    let mut platform: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    for i in 0..platform.len() {
        for j in 0..platform[0].len() {
            if platform[i][j] == 'O' {
                for k in (0..=i).rev() {
                    if k > 0 && platform[k - 1][j] == '.' {
                        platform[k][j] = '.';
                        platform[k - 1][j] = 'O';
                    } else {
                        load += (platform.len() - k) as u64;
                        break;
                    }
                }
            }
        }
    }
    load
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day14.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 110407);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 22);
    }
}
