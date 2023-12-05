use itertools::Itertools;

struct Spec {
    in_start: u64,
    out_start: u64,
    range: u64,
}

impl Spec {
    fn parse(line: &str) -> Spec {
        let (out_start, in_start, range) = line
            .split_whitespace()
            .flat_map(|n| n.parse::<u64>())
            .collect_tuple()
            .unwrap();
        Spec {
            in_start,
            out_start,
            range,
        }
    }

    fn intoout(&self, inn: u64) -> Option<u64> {
        if inn >= self.in_start && inn < self.in_start + self.range {
            Some(self.out_start + (inn - self.in_start))
        } else {
            None
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse::<u64>().unwrap())
        .collect_vec();
    lines
        .group_by(|l| l.is_empty())
        .into_iter()
        .filter(|(empty_line, _)| !empty_line)
        .map(|(_, specs)| specs.skip(1).map(Spec::parse).collect_vec())
        .for_each(|specs| {
            dbg!(&seeds);
            seeds = seeds
                .iter()
                .map(|seed| {
                    specs
                        .iter()
                        .find_map(|spec| spec.intoout(*seed))
                        .unwrap_or(*seed)
                })
                .collect_vec()
        });
    *seeds.iter().min().unwrap()
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day5.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 388071289);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 22);
    }
}
