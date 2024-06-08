use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

fn read_layout(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(|l| l.chars().map(|c| c as usize - '0' as usize).collect()).collect()
}

fn next(x: usize, y: usize, xlen: usize, ylen: usize, d: Direction) -> Option<(usize, usize)> {
    match d {
        Direction::Down => if x + 1 < xlen { Some((x + 1, y)) } else { None },
        Direction::Up => if x > 0 { Some((x - 1, y)) } else { None },
        Direction::Right => if y + 1 < ylen { Some((x, y + 1)) } else { None },
        Direction::Left => if y > 0 { Some((x, y - 1)) } else { None },
    }

}

pub fn part1(input: &str) -> i64 {
    let layout = read_layout(input);
    let xlen = layout.len();
    let ylen = layout[0].len();
    let mut seen = HashMap::<((usize, usize), Direction, usize), i64>::new();
    let mut queue = BinaryHeap::new();
    // (total_cost, pos(x,y), straight_steps, direction)
    let start = (0, (0, 0), 0, Direction::Right);
    queue.push(start);
    while !queue.is_empty() {
        let (cost, (x, y), straight, direction) = queue.pop().unwrap();
        let seen_key = ((x,y), direction, straight);
        if let Some(seen_cost) = seen.get(&seen_key) {
            if *seen_cost > cost {
                continue;
            }
        }
        // dbg!(cost);
        if x == xlen - 1 && y == ylen - 1 {
            return -cost-layout[x][y] as i64 ;
        }
        for d in vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            if d == direction && straight == 3 {
                continue;
            }
            match (d, direction) {
                (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) | (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => {continue;},
                _ => {}
            }
            if let Some(pos) = next(x, y, xlen, ylen, d) {
                let cost = cost - layout[pos.0][pos.1] as i64;
                let straight = if d == direction { straight + 1 } else {0};
                // check if seen pos+d+<=straight with lower cost already and skip if so
                let seen_key = (pos, d, straight);
                if let Some(seen_cost) = seen.get(&seen_key) {
                    if *seen_cost >= cost {
                        continue;
                    }
                }
                queue.push((cost, pos, straight, d));
                for s in 0..=straight {
                    let seen_val = seen.entry((pos,d,s)).or_insert(cost);
                    *seen_val = std::cmp::max(cost, *seen_val);
                }
            }
        }
    }
    unreachable!();
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "input"
        // include_str!("../input/2023/day01.txt")
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
