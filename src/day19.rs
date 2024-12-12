use std::collections::HashMap;

use itertools::Itertools;

#[derive(Hash, PartialEq, Eq)]
enum Category {
    Extremely,
    Musical,
    Aerodynamic,
    Shiny,
}

impl Category {
    fn read(c: char) -> Category {
        match c {
            'x' => Category::Extremely,
            'm' => Category::Musical,
            'a' => Category::Aerodynamic,
            's' => Category::Shiny,
            _ => unimplemented!("unknown category: {}", c),
        }
    }
}

enum Cmp {
    Greater,
    Lesser,
}

impl Cmp {
    fn read(c: char) -> Cmp {
        match c {
            '<' => Cmp::Lesser,
            '>' => Cmp::Greater,
            _ => unimplemented!("unknown cmp: {}", c),
        }
    }
    fn compare(&self, l: usize, r: usize) -> bool {
        match self {
            Cmp::Greater => l > r,
            Cmp::Lesser => l < r,
        }
    }
}

enum Rule {
    Cmp(Category, Cmp, usize, RuleResult),
    Move(String),
    Accept,
    Reject,
}

impl Rule {
    fn read(s: &str) -> Rule {
        match s {
            "A" => Rule::Accept,
            "R" => Rule::Reject,
            s if s.contains(':') => {
                let (cmp, res) = s.split(':').collect_tuple().unwrap();
                let mut cmp = cmp.chars();
                Rule::Cmp(
                    Category::read(cmp.next().unwrap()),
                    Cmp::read(cmp.next().unwrap()),
                    cmp.collect::<String>().parse::<usize>().unwrap(),
                    RuleResult::read(res),
                )
            }
            s => Rule::Move(s.to_string()),
        }
    }

    fn run(&self, part: &Part) -> Option<RuleResult> {
        match self {
            Self::Accept => Some(RuleResult::Accept),
            Self::Reject => Some(RuleResult::Reject),
            Self::Move(new_rule) => Some(RuleResult::Move(new_rule.clone())),
            Self::Cmp(category, cmp, val, result) => {
                if cmp.compare(part.categories[category], *val) {
                    Some(result.clone())
                } else {
                    None
                }
            }
        }
    }
}
#[derive(Clone)]
enum RuleResult {
    Move(String),
    Accept,
    Reject,
}

impl RuleResult {
    fn read(s: &str) -> RuleResult {
        match s {
            "A" => RuleResult::Accept,
            "R" => RuleResult::Reject,
            s => RuleResult::Move(s.to_string()),
        }
    }
}

struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn read(line: &str) -> (String, Workflow) {
        let (name, rules) = line[..(line.len() - 1)].split('{').collect_tuple().unwrap();
        (
            name.to_string(),
            Workflow {
                rules: rules.split(',').map(Rule::read).collect(),
            },
        )
    }
    fn run(&self, part: &Part) -> RuleResult {
        for rule in &self.rules {
            if let Some(result) = rule.run(part) {
                return result;
            }
        }
        unreachable!("Last rule should yield result");
    }
}

struct Part {
    categories: HashMap<Category, usize>,
}

impl Part {
    fn read(line: &str) -> Part {
        let (x, m, a, s) = line[1..(line.len() - 1)]
            .split(',')
            .map(|category| category[2..].parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        Part {
            categories: HashMap::from([
                (Category::Extremely, x),
                (Category::Musical, m),
                (Category::Aerodynamic, a),
                (Category::Shiny, s),
            ]),
        }
    }

    fn score(&self) -> usize {
        self.categories.values().sum()
    }
}

pub fn part1(input: &str) -> usize {
    let workflows: HashMap<String, Workflow> = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(Workflow::read)
        .collect();
    input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(Part::read)
        .map(|p| {
            let mut res = RuleResult::Move("in".to_string());
            while let RuleResult::Move(ref workflow) = res {
                res = workflows[workflow].run(&p);
            }
            match res {
                RuleResult::Accept => p.score(),
                RuleResult::Reject => 0,
                RuleResult::Move(_) => unreachable!("unexpected Move state"),
            }
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day19.txt")
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 11);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 22);
    }
}
