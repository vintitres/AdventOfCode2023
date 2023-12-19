use itertools::Itertools;
use std::collections::HashMap;

struct HotSpringRow {
    row: Vec<char>,
    conditions: Vec<usize>,
}

impl HotSpringRow {
    pub fn parse(s: &str) -> Self {
        let (row, conditions) = s.split_ascii_whitespace().collect_tuple().unwrap();
        Self {
            row: row.chars().collect_vec(),
            conditions: conditions.split(',').flat_map(|n| n.parse()).collect_vec(),
        }
    }

    pub fn unfold(&self) -> Self {
        Self {
            row: self
                .row
                .clone()
                .into_iter()
                .chain(['?'])
                .cycle()
                .take((self.row.len() + 1) * 5 - 1)
                .collect_vec(),
            conditions: self
                .conditions
                .clone()
                .into_iter()
                .cycle()
                .take(self.conditions.len() * 5)
                .collect_vec(),
        }
    }

    pub fn combinations(&self) -> usize {
        Self::combinations_(
            '.',
            0,
            0,
            0,
            &self.row,
            &self.conditions,
            &mut HashMap::new(),
        )
    }

    fn combinations_(
        row_head: char,
        row_index: usize,
        conditions_index: usize,
        springs_just_before: usize,
        row: &[char],
        conditions: &[usize],
        mem: &mut HashMap<(char, usize, usize, usize), usize>,
    ) -> usize {
        let k = (row_head, row_index, conditions_index, springs_just_before);
        if !mem.contains_key(&k) {
            let v = match row_head {
                '$' => {
                    if conditions_index >= conditions.len() - 1
                        && *conditions.get(conditions_index).unwrap_or(&0) == springs_just_before
                    {
                        1
                    } else {
                        0
                    }
                }
                '.' => {
                    if springs_just_before == 0 {
                        Self::combinations_(
                            *row.get(row_index).unwrap_or(&'$'),
                            row_index + 1,
                            conditions_index,
                            0,
                            row,
                            conditions,
                            mem,
                        )
                    } else if *conditions.get(conditions_index).unwrap_or(&0) == springs_just_before
                    {
                        Self::combinations_(
                            *row.get(row_index).unwrap_or(&'$'),
                            row_index + 1,
                            conditions_index + 1,
                            0,
                            row,
                            conditions,
                            mem,
                        )
                    } else {
                        0
                    }
                }
                '#' => Self::combinations_(
                    *row.get(row_index).unwrap_or(&'$'),
                    row_index + 1,
                    conditions_index,
                    springs_just_before + 1,
                    row,
                    conditions,
                    mem,
                ),
                '?' => {
                    Self::combinations_(
                        '#',
                        row_index,
                        conditions_index,
                        springs_just_before,
                        row,
                        conditions,
                        mem,
                    ) + Self::combinations_(
                        '.',
                        row_index,
                        conditions_index,
                        springs_just_before,
                        row,
                        conditions,
                        mem,
                    )
                }
                _ => panic!("!"),
            };
            mem.insert(k, v);
            v
        } else {
            mem[&k]
        }
    }
}
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(HotSpringRow::parse)
        .map(|hsr| hsr.combinations())
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(HotSpringRow::parse)
        .map(|hsr| hsr.unfold().combinations())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day12.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 8022);
    }

    #[ignore = "slow"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 4968620679637);
    }
}
