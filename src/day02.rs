use itertools::Itertools;

const RED: usize = 12;
const GREEN: usize = 13;
const BLUE: usize = 14;

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            if line.split(": ").nth(1).unwrap().split("; ").all(|pull| {
                pull.split(", ")
                    .all(|num_color| match num_color.split(' ').collect_tuple() {
                        Some((num, "red")) => num.parse::<usize>().unwrap() <= RED,
                        Some((num, "green")) => num.parse::<usize>().unwrap() <= GREEN,
                        Some((num, "blue")) => num.parse::<usize>().unwrap() <= BLUE,
                        _ => unimplemented!("unknown entry: {}", num_color),
                    })
            }) {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;
            line.split(": ")
                .nth(1)
                .unwrap()
                .split("; ")
                .for_each(|pull| {
                    pull.split(", ").for_each(|num_color| {
                        match num_color.split(' ').collect_tuple() {
                            Some((num, "red")) => {
                                max_red = std::cmp::max(max_red, num.parse::<u32>().unwrap())
                            }
                            Some((num, "green")) => {
                                max_green = std::cmp::max(max_green, num.parse::<u32>().unwrap())
                            }
                            Some((num, "blue")) => {
                                max_blue = std::cmp::max(max_blue, num.parse::<u32>().unwrap())
                            }
                            _ => unimplemented!("unknown entry: {}", num_color),
                        }
                    })
                });
            max_red * max_green * max_blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day2.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 2237);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 66681);
    }
}
