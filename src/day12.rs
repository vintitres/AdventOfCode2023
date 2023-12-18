use itertools::Itertools;

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
        Self::combinations_(self.row.split_first(), &self.conditions, 0)
    }

    fn combinations_(
        row: Option<(&char, &[char])>,
        conditions: &[usize],
        springs_just_before: usize,
    ) -> usize {
        match row {
            None => {
                if conditions.len() <= 1 && *conditions.first().unwrap_or(&0) == springs_just_before
                {
                    1
                } else {
                    0
                }
            }
            Some(('.', row)) => {
                if springs_just_before == 0 {
                    Self::combinations_(row.split_first(), conditions, 0)
                } else if *conditions.first().unwrap_or(&0) == springs_just_before {
                    Self::combinations_(row.split_first(), &conditions[1..], 0)
                } else {
                    0
                }
            }
            Some(('#', row)) => {
                Self::combinations_(row.split_first(), conditions, springs_just_before + 1)
            }
            Some(('?', row)) => {
                Self::combinations_(Some((&'#', row)), conditions, springs_just_before)
                    + Self::combinations_(Some((&'.', row)), conditions, springs_just_before)
            }
            _ => panic!("!"),
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

    // #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 22);
    }
}
