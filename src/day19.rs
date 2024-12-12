use std::{collections::HashMap, iter::repeat};

use itertools::Itertools;

#[derive(Hash, PartialEq, Eq, Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

    fn eval(&self, range: PartRange) -> (PartRange, RuleResult, PartRange) {
        match self {
            Self::Accept => (range, RuleResult::Accept, PartRange::empty()),
            Self::Reject => (PartRange::empty(), RuleResult::Reject, PartRange::empty()),
            Self::Move(new_workflow) => (
                range,
                RuleResult::Move(new_workflow.clone()),
                PartRange::empty(),
            ),
            Self::Cmp(category, cmp, val, result) => {
                let (range1, range2) = range.split(category, cmp, val);
                (range1, result.clone(), range2)
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

#[derive(Clone)]
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

#[derive(Clone)]
struct PartRange {
    ranges: HashMap<Category, (usize, usize)>,
}

impl PartRange {
    fn new() -> PartRange {
        PartRange {
            ranges: HashMap::from_iter(
                [
                    Category::Extremely,
                    Category::Musical,
                    Category::Aerodynamic,
                    Category::Shiny,
                ]
                .into_iter()
                .zip(repeat((1, 4001))),
            ),
        }
    }

    fn empty() -> PartRange {
        PartRange {
            ranges: HashMap::from_iter(
                [
                    Category::Extremely,
                    Category::Musical,
                    Category::Aerodynamic,
                    Category::Shiny,
                ]
                .into_iter()
                .zip(repeat((1, 1))),
            ),
        }
    }

    fn is_empty(&self) -> bool {
        self.count() == 0
    }

    fn count(&self) -> usize {
        self.ranges
            .values()
            .map(|r| if r.1 < r.0 { 0 } else { r.1 - r.0 })
            .product()
    }

    fn split(&self, category: &Category, cmp: &Cmp, val: &usize) -> (PartRange, PartRange) {
        let mut range1 = self.clone();
        let mut range2 = self.clone();

        let (begin, end) = self.ranges[category];

        let (r1, r2) = match cmp {
            Cmp::Greater => ((val + 1, end), (begin, val + 1)),
            Cmp::Lesser => ((begin, *val), (*val, end)),
        };
        *range1.ranges.get_mut(category).unwrap() = r1;
        *range2.ranges.get_mut(category).unwrap() = r2;
        (range1, range2)
    }
}

fn go(workflow: &str, workflows: &HashMap<String, Workflow>, range: PartRange) -> usize {
    if range.is_empty() {
        return 0;
    }
    let workflow = workflows[&workflow.to_string()].clone();
    let mut range = range.clone();
    let mut sum = 0;
    for rule in workflow.rules {
        let (pass_range, pass_result, fail_range) = rule.eval(range);
        sum += match pass_result {
            RuleResult::Accept => pass_range.count(),
            RuleResult::Reject => 0,
            RuleResult::Move(new_workflow) => go(&new_workflow, workflows, pass_range),
        };
        if fail_range.is_empty() {
            break;
        }
        range = fail_range;
    }
    sum
}

pub fn part2(input: &str) -> usize {
    let workflows: HashMap<String, Workflow> = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(Workflow::read)
        .collect();
    go("in", &workflows, PartRange::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day19.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 362930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 116365820987729);
    }
}
