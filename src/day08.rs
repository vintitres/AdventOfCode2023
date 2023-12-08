use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let rl = lines.next().unwrap();
    lines.next();
    let map: HashMap<&str, (&str, &str)> = lines
        .map(|l| {
            let (node, tonodes) = l.split_once(" = (").unwrap();
            let (l, r) = tonodes.split_once(", ").unwrap();
            let r = r.strip_suffix(')').unwrap();
            (node, (l, r))
        })
        .collect();
    rl.chars()
        .cycle()
        .scan("AAA", |node, rl| {
            // dbg!(&node);
            if *node == "ZZZ" {
                None
            } else {
                let nodem = map[node];
                match rl {
                    'R' => {
                        *node = nodem.1;
                        Some(rl)
                    }
                    'L' => {
                        *node = nodem.0;
                        Some(rl)
                    }
                    _ => unreachable!(""),
                }
            }
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day8.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 15517);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 22);
    }
}
