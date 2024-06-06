use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Beam {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Beam {
    fn left(&self) -> Beam {
        Beam {
            direction: Direction::Left,
            ..*self
        }
    }
    fn right(&self) -> Beam {
        Beam {
            direction: Direction::Right,
            ..*self
        }
    }
    fn up(&self) -> Beam {
        Beam {
            direction: Direction::Up,
            ..*self
        }
    }
    fn down(&self) -> Beam {
        Beam {
            direction: Direction::Down,
            ..*self
        }
    }
    fn encounter(&self, tile: char) -> Vec<Beam> {
        match tile {
            '.' => vec![*self],
            '|' => match self.direction {
                Direction::Up | Direction::Down => vec![*self],
                Direction::Left | Direction::Right => vec![
                    Beam {
                        direction: Direction::Up,
                        ..*self
                    },
                    Beam {
                        direction: Direction::Down,
                        ..*self
                    },
                ],
            },
            '-' => match self.direction {
                Direction::Up | Direction::Down => vec![
                    Beam {
                        direction: Direction::Right,
                        ..*self
                    },
                    Beam {
                        direction: Direction::Left,
                        ..*self
                    },
                ],
                Direction::Left | Direction::Right => vec![*self],
            },
            '/' => match self.direction {
                Direction::Down => vec![self.left()],
                Direction::Up => vec![self.right()],
                Direction::Left => vec![self.down()],
                Direction::Right => vec![self.up()],
            },
            '\\' => match self.direction {
                Direction::Down => vec![self.right()],
                Direction::Up => vec![self.left()],
                Direction::Left => vec![self.up()],
                Direction::Right => vec![self.down()],
            },
            _ => unreachable!(),
        }
    }
    fn next(&self, xlen: usize, ylen: usize) -> Option<Beam> {
        match self.direction {
            Direction::Left => {
                if self.y != 0 {
                    Some(Beam {
                        y: self.y - 1,
                        ..*self
                    })
                } else {
                    None
                }
            }
            Direction::Up => {
                if self.x != 0 {
                    Some(Beam {
                        x: self.x - 1,
                        ..*self
                    })
                } else {
                    None
                }
            }
            Direction::Down => {
                if self.x + 1 < xlen {
                    Some(Beam {
                        x: self.x + 1,
                        ..*self
                    })
                } else {
                    None
                }
            }
            Direction::Right => {
                if self.y + 1 < ylen {
                    Some(Beam {
                        y: self.y + 1,
                        ..*self
                    })
                } else {
                    None
                }
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let layout: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let xlen = layout.len();
    let ylen = layout[0].len();
    let mut queue = VecDeque::<Beam>::new();
    let mut visited = HashSet::<Beam>::new();
    let start_beam = Beam {
        x: 0,
        y: 0,
        direction: Direction::Right,
    };
    queue.push_front(start_beam);
    visited.insert(start_beam);
    while !queue.is_empty() {
        let beam = queue.pop_front().unwrap();
        for b in beam.encounter(layout[beam.x][beam.y]) {
            match b.next(xlen, ylen) {
                Some(b) => {
                    if !visited.contains(&b) {
                        visited.insert(b);
                        queue.push_back(b);
                    }
                }
                None => {}
            }
        }
    }
    HashSet::<(usize, usize)>::from_iter(visited.iter().map(|beam| (beam.x, beam.y))).len()
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day16.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 6740);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 22);
    }
}
