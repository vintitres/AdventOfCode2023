use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn parse(s: &str) -> Self {
        match s {
            "L" => Self::Left,
            "R" => Self::Right,
            "U" => Self::Up,
            "D" => Self::Down,
            _ => panic!("unknown direction: {}", s),
        }
    }
    pub fn next(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Self::Left => (x, y - 1),
            Self::Right => (x, y + 1),
            Self::Up => (x - 1, y),
            Self::Down => (x + 1, y),
        }
    }
}
struct Dig {
    direction: Direction,
    length: usize,
    color: String,
}

impl Dig {
    pub fn parse(s: &str) -> Self {
        let (d, l, c) = s.split_ascii_whitespace().collect_tuple().unwrap();
        Self {
            direction: Direction::parse(d),
            length: l.parse().unwrap(),
            color: c.to_string(),
        }
    }
}

#[derive(Debug)]
struct Bounds {
    max_x: i32,
    max_y: i32,
    min_x: i32,
    min_y: i32,
}

impl Bounds {
    pub fn new() -> Self {
        Self {
            max_x: i32::MIN,
            max_y: i32::MIN,
            min_x: i32::MAX,
            min_y: i32::MAX,
        }
    }

    pub fn add(&mut self, (x, y): (i32, i32)) {
        self.max_x = std::cmp::max(x, self.max_x);
        self.max_y = std::cmp::max(y, self.max_y);
        self.min_x = std::cmp::min(x, self.min_x);
        self.min_y = std::cmp::min(y, self.min_y);
    }
    pub fn extend1(&mut self) {
        self.max_x += 1;
        self.max_y += 1;
        self.min_x -= 1;
        self.min_y -= 1;
    }

    pub fn is_in(&self, (x, y): (i32, i32)) -> bool {
        x <= self.max_x && x >= self.min_x && y <= self.max_y && y >= self.min_y
    }

    pub fn size(&self) -> usize {
        ((self.max_x - self.min_x + 1) * (self.max_y - self.min_y + 1)) as usize
    }
}

pub fn part1(input: &str) -> usize {
    let mut map = HashMap::new();
    let mut bounds = Bounds::new();
    let mut p = (0, 0);
    bounds.add(p);
    map.insert(p, "()".to_string());
    for dig in input.lines().map(Dig::parse) {
        for _ in 0..dig.length {
            p = dig.direction.next(p);
            bounds.add(p);
            map.insert(p, dig.color.clone());
        }
    }
    bounds.extend1();
    let mut q = VecDeque::new();
    let mut ground = HashSet::new();
    let start = (bounds.min_x, bounds.min_y);
    q.push_back(start);
    ground.insert(start);
    while !q.is_empty() {
        let p = q.pop_front().unwrap();
        for direction in [
            Direction::Down,
            Direction::Up,
            Direction::Left,
            Direction::Right,
        ] {
            let p = direction.next(p);
            if bounds.is_in(p) && !map.contains_key(&p) && !ground.contains(&p) {
                // dbg!(p);
                // dbg!(&bounds);
                q.push_back(p);
                ground.insert(p);
            }
        }
    }
    bounds.size() - ground.len()
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day18.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 39039);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 22);
    }
}
