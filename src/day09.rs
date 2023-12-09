use itertools::Itertools;

fn find_next(seq: &Vec<i64>) -> i64 {
    if seq.iter().all(|&n| n == 0) {
        0
    } else {
        seq.last().unwrap()
            + find_next(
                &seq.iter()
                    .skip(1)
                    .scan(seq.first().unwrap(), |last, n| {
                        let ret = Some(n - *last);
                        *last = n;
                        ret
                    })
                    .collect_vec(),
            )
    }
}

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            find_next(
                &l.split_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect_vec(),
            )
        })
        .sum()
}

fn find_prev(seq: &Vec<i64>) -> i64 {
    if seq.iter().all(|&n| n == 0) {
        0
    } else {
        seq.first().unwrap()
            - find_prev(
                &seq.iter()
                    .skip(1)
                    .scan(seq.first().unwrap(), |last, n| {
                        let ret = Some(n - *last);
                        *last = n;
                        ret
                    })
                    .collect_vec(),
            )
    }
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            find_prev(
                &l.split_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect_vec(),
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day9.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1974232246);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 928);
    }
}
