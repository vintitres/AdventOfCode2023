use core::cmp::{max, min};
use itertools::Itertools;
use std::collections::BTreeMap;

struct Spec {
    in_start: u64,
    out_start: u64,
    length: u64,
}

impl Spec {
    fn parse(line: &str) -> Spec {
        let (out_start, in_start, length) = line
            .split_whitespace()
            .flat_map(|n| n.parse::<u64>())
            .collect_tuple()
            .unwrap();
        Spec {
            in_start,
            out_start,
            length,
        }
    }

    fn intoout(&self, in_: u64) -> Option<u64> {
        if in_ >= self.in_start && in_ < self.in_start + self.length {
            Some(self.out_start + (in_ - self.in_start))
        } else {
            None
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let seeds: Box<dyn Iterator<Item = u64>> = Box::new(
        lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|n| n.parse::<u64>().unwrap()),
    );
    lines
        .group_by(|l| l.is_empty())
        .into_iter()
        .filter(|(empty_line, _)| !empty_line)
        .map(|(_, specs)| specs.skip(1).map(Spec::parse).collect_vec())
        .fold(seeds, move |things, specs| {
            Box::new(things.map(move |seed| {
                specs
                    .iter()
                    .find_map(|spec| spec.intoout(seed))
                    .unwrap_or(seed)
            }))
        })
        .min()
        .unwrap()
}

#[derive(Debug)]
struct Spec2 {
    specs: BTreeMap<u64, i64>, // <range_start, range_shift>
}

impl Spec2 {
    fn add(&mut self, line: &str) {
        let (out_start, in_start, length) = line
            .split_whitespace()
            .flat_map(|n| n.parse::<u64>())
            .collect_tuple()
            .unwrap();
        self.specs
            .insert(in_start, out_start as i64 - in_start as i64);
        self.specs.entry(in_start + length).or_insert(0);
        self.specs.insert(u64::MAX, 0);
    }
    fn intoout(&self, in_: &Range) -> Vec<Range> {
        self.specs
            .iter()
            .scan(
                (0, 0),
                |(last_range_start, last_range_shift), (range_start, range_shift)| {
                    // [ ][ ][ { } ]
                    // [ ][ ][ { ][ ][ } ]
                    // dbg!(&last_range_start, &range_start, in_);
                    let ret = if *range_start < in_.start {
                        Some(None)
                    } else if *last_range_start >= in_.end {
                        None
                    } else {
                        Some(Some(Range {
                            start: max(*last_range_start, in_.start)
                                .checked_add_signed(*last_range_shift)
                                .unwrap(),
                            end: min(*range_start, in_.end)
                                .checked_add_signed(*last_range_shift)
                                .unwrap(),
                        }))
                    };
                    *last_range_start = *range_start;
                    *last_range_shift = *range_shift;
                    // dbg!(&ret);
                    ret
                },
            )
            .flatten()
            .collect_vec()
    }
}

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64, // exclusive
}
pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse::<u64>().unwrap())
        .chunks(2);
    let seeds: Box<dyn Iterator<Item = Range>> = Box::new(seeds.into_iter().map(|c| {
        let (s, l) = c.collect_tuple().unwrap();
        Range {
            start: s,
            end: s + l,
        }
    }));
    lines
        .group_by(|l| l.is_empty())
        .into_iter()
        .filter(|(empty_line, _)| !empty_line)
        .map(|(_, specs)| {
            let mut specs2 = Spec2 {
                specs: BTreeMap::new(),
            };
            specs.skip(1).for_each(|s| specs2.add(s));
            specs2
        })
        .fold(seeds, |things, specs| {
            Box::new(
                things
                    .flat_map(move |seed| specs.intoout(&seed))
                    .filter(|s| s.start != s.end),
            )
        })
        .map(|s| s.start)
        .min()
        .unwrap()
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 84206669);
    }
}
