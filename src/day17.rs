use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u64,
    position: (usize, usize),
    straight_steps: usize,
    direction: Direction,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.straight_steps.cmp(&other.straight_steps))
            .then_with(|| self.direction.cmp(&other.direction))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read_layout(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c as usize - '0' as usize).collect())
        .collect()
}

fn next(x: usize, y: usize, xlen: usize, ylen: usize, d: Direction) -> Option<(usize, usize)> {
    match d {
        Direction::Down => {
            if x + 1 < xlen {
                Some((x + 1, y))
            } else {
                None
            }
        }
        Direction::Up => {
            if x > 0 {
                Some((x - 1, y))
            } else {
                None
            }
        }
        Direction::Right => {
            if y + 1 < ylen {
                Some((x, y + 1))
            } else {
                None
            }
        }
        Direction::Left => {
            if y > 0 {
                Some((x, y - 1))
            } else {
                None
            }
        }
    }
}

pub fn part1(input: &str) -> u64 {
    doit(input, 0, 3)
}

fn doit(input: &str, min_straight: usize, max_straight: usize) -> u64 {
    let layout = read_layout(input);
    let xlen = layout.len();
    let ylen = layout[0].len();
    let mut seen = HashMap::<((usize, usize), Direction, usize), u64>::new();
    let mut queue = BinaryHeap::new();
    let start = State {
        cost: 0,
        position: (0, 0),
        straight_steps: 0,
        direction: Direction::Right,
    };
    queue.push(start);
    let start = State {
        cost: 0,
        position: (0, 0),
        straight_steps: 0,
        direction: Direction::Down,
    };
    queue.push(start);
    while !queue.is_empty() {
        let State {
            cost,
            position,
            straight_steps,
            direction,
        } = queue.pop().unwrap();
        let seen_key = (position, direction, straight_steps);
        if let Some(seen_cost) = seen.get(&seen_key) {
            if *seen_cost < cost {
                continue;
            }
        }
        let (x, y) = position;
        if x == xlen - 1 && y == ylen - 1 && straight_steps + 1 >= min_straight {
            return cost;
        }
        // dbg!(x, y, direction);
        for d in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if d != direction && straight_steps + 1 < min_straight {
                continue;
            }
            if d == direction && straight_steps == max_straight - 1 {
                continue;
            }
            match (d, direction) {
                (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left) => {
                    continue;
                }
                _ => {}
            }
            // dbg!(d);
            if let Some(pos) = next(x, y, xlen, ylen, d) {
                let cost = cost + layout[pos.0][pos.1] as u64;
                let straight = if d == direction {
                    straight_steps + 1
                } else {
                    0
                };
                let seen_key = (pos, d, straight);
                if let Some(seen_cost) = seen.get(&seen_key) {
                    if *seen_cost <= cost {
                        continue;
                    }
                }
                queue.push(State {
                    cost,
                    position: pos,
                    straight_steps: straight,
                    direction: d,
                });
                let seen_val = seen.entry((pos, d, straight)).or_insert(cost);
                *seen_val = std::cmp::min(cost, *seen_val);
            }
        }
    }
    unreachable!();
}

pub fn part2(input: &str) -> u64 {
    doit(input, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day17.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1001);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 22);
    }
}
