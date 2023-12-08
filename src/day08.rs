use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};

fn read(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let mut lines = input.lines();
    let rl = lines.next().unwrap();
    lines.next();
    let map: HashMap<&str, (&str, &str)> = lines
        .map(|l| {
            let (node, tonodes) = l.split_once(" = (").unwrap();
            let (l, r) = tonodes.split_once(", ").unwrap();
            let r = r.strip_suffix(')').unwrap();
            (node, (l, r))
        })
        .collect();
    (rl, map)
}

pub fn part1(input: &str) -> usize {
    let (rl, map) = read(input);
    rl.chars()
        .cycle()
        .scan("AAA", |node, rl| {
            // dbg!(&node);
            if *node == "ZZZ" {
                None
            } else {
                let nodem = map[node];
                match rl {
                    'R' => {
                        *node = nodem.1;
                        Some(rl)
                    }
                    'L' => {
                        *node = nodem.0;
                        Some(rl)
                    }
                    _ => unreachable!(""),
                }
            }
        })
        .count()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Cycle {
    start: u64,
    len: u64,
    zs: Vec<u64>,
}

pub fn part2(input: &str) -> u64 {
    let (rl, map) = read(input);
    let mut visited: HashMap<(usize, &str), u64> = HashMap::new();
    let cycles = map
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(k, _)| {
            let mut i = 0u64;
            let mut node = k;
            let mut zs = Vec::new();
            loop {
                if node.ends_with('Z') {
                    zs.push(i);
                }
                let rli = (i % (rl.len() as u64)) as usize;
                if visited.contains_key(&(rli, *node)) {
                    break;
                }
                visited.insert((rli, node), i);
                // dbg!(&visited);
                match rl.chars().nth(rli).unwrap() {
                    'R' => {
                        node = &map[node].1;
                    }
                    'L' => {
                        node = &map[node].0;
                    }
                    _ => unreachable!(""),
                }
                i += 1;
            }
            let rli = (i % (rl.len() as u64)) as usize;
            let cycle_start = visited.get(&(rli, *node)).unwrap();
            Cycle {
                start: *cycle_start,
                len: i - *cycle_start,
                // NOTE only care about Zs after cycle start because assuming it will be a lot of steps
                zs: zs
                    .iter()
                    .filter(|i| i >= &cycle_start)
                    .map(|i| i - *cycle_start)
                    .collect(),
            }
        })
        .collect_vec();
    // dbg!(&cycles);
    let mut pq: BTreeSet<(u64, usize)> = BTreeSet::new();
    let mut ghost_positions: Vec<u64> = Vec::new();
    cycles.iter().enumerate().for_each(|(i, c)| {
        let first_z_on_cycle = c.start + c.zs.first().unwrap();
        ghost_positions.push(first_z_on_cycle);
        c.zs.iter().for_each(|z| {
            pq.insert((c.start + z, i));
        })
    });
    loop {
        // dbg!(&ghost_positions, &pq);
        let (ghost_pos, ghost_i) = pq.pop_first().unwrap();
        ghost_positions[ghost_i] = ghost_pos;
        if ghost_positions.iter().all(|&p| p == ghost_pos) {
            return ghost_pos;
        }
        let ghost = &cycles[ghost_i];
        pq.insert((ghost_pos + ghost.len, ghost_i));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day8.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 15517);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 22);
    }
}
