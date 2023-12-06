use itertools::Itertools;

/* 7
0 0
1 6
2 10
3 12
4 12
5 10
6 6
7 0
 */

/* 8
0 0
1 7
2 2 * 6
3 3 * 5
4 4 * 4
5 5 * 3
6 6 * 2
7 7
8 0
 */
pub fn part1(input: &str) -> u64 {
    let (time, distance) = input
        .lines()
        .map(|l| {
            l.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|v| v.parse::<u64>().unwrap())
        })
        .collect_tuple()
        .unwrap();
    time.zip(distance)
        .map(|(time, distance)| ways_to_win(time, distance))
        .product()
}

fn ways_to_win(race_time: u64, record_distance: u64) -> u64 {
    let mut b = 0;
    let mut e = race_time / 2;
    while b < e {
        let m = b + (e - b) / 2;
        let d = m * (race_time - m);
        if d <= record_distance {
            b = m + 1;
        } else {
            e = m;
        }
    }
    ((race_time / 2) - b) * 2 + 1 + race_time % 2
}

pub fn part2(input: &str) -> u64 {
    let (time, distance) = input
        .lines()
        .map(|l| {
            l.split_once(':')
                .unwrap()
                .1
                .replace(' ', "")
                .parse::<u64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();
    ways_to_win(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day6.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 588588);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 22);
    }
}
