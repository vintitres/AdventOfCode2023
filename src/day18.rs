use std::collections::{BTreeSet, HashSet, VecDeque};

use itertools::Itertools;

#[derive(Debug)]
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
    pub fn parse2(s: &str) -> Self {
        match s {
            "2" => Self::Left,
            "0" => Self::Right,
            "3" => Self::Up,
            "1" => Self::Down,
            _ => panic!("unknown direction: {}", s),
        }
    }
    pub fn next(&self, p: (i32, i32)) -> (i32, i32) {
        self.jump(p, 1)
    }
    pub fn jump(&self, (x, y): (i32, i32), length: usize) -> (i32, i32) {
        let length = length as i32;
        match self {
            Self::Left => (x, y - length),
            Self::Right => (x, y + length),
            Self::Up => (x - length, y),
            Self::Down => (x + length, y),
        }
    }
}
#[derive(Debug)]
struct Dig {
    direction: Direction,
    length: usize,
}

impl Dig {
    pub fn parse(s: &str) -> Self {
        let (d, l, _c) = s.split_ascii_whitespace().collect_tuple().unwrap();
        Self {
            direction: Direction::parse(d),
            length: l.parse().unwrap(),
        }
    }

    pub fn parse2(s: &str) -> Self {
        let (_d, _l, c) = s.split_ascii_whitespace().collect_tuple().unwrap();
        let (length_hex, direction_index) = c
            .strip_prefix("(#")
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_at(5);
        Self {
            direction: Direction::parse2(direction_index),
            length: usize::from_str_radix(length_hex, 16).unwrap(),
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
    let mut map = HashSet::new();
    let mut bounds = Bounds::new();
    let mut p = (0, 0);
    bounds.add(p);
    map.insert(p);
    for dig in input.lines().map(Dig::parse) {
        for _ in 0..dig.length {
            p = dig.direction.next(p);
            bounds.add(p);
            map.insert(p);
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
            if bounds.is_in(p) && !map.contains(&p) && !ground.contains(&p) {
                // dbg!(p);
                // dbg!(&bounds);
                q.push_back(p);
                ground.insert(p);
            }
        }
    }
    bounds.size() - ground.len()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SideWall {
    top_x: i32,
    y: i32,
    bottom_x: i32,
}

impl SideWall {
    pub fn from((x, y): (i32, i32), direction: &Direction, length: usize) -> Option<Self> {
        match direction {
            Direction::Down => Some(Self {
                top_x: x,
                bottom_x: x + length as i32,
                y: y,
            }),
            Direction::Up => Some(Self {
                top_x: x - length as i32,
                y: y,
                bottom_x: x,
            }),
            _ => None,
        }
    }
}

pub fn part2(input: &str) -> usize {
    /* TODO
     * (below algo only works if no hooks sticking out of the hole on the sides)
     * Go clockwise around the hole and each time you go down add to set of left wall x ranges,
     * each time you go up add the covered area on the right of up path and clear blocked part from walls set
     * + ensure start from leftmost y on top x
     * */
    let mut p = (0, 0);
    let mut walls = BTreeSet::new();
    for dig in input.lines().map(Dig::parse2) {
        if let Some(wall) = SideWall::from(p, &dig.direction, dig.length) {
            walls.insert(wall);
        }
        p = dig.direction.jump(p, dig.length);
    }
    dbg!(walls);
    0
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
