use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

fn mv((px, py): (usize, usize), d: char) -> (usize, usize) {
    match d {
        'N' => (px.overflowing_sub(1).0, py),
        'S' => (px + 1, py),
        'E' => (px, py + 1),
        'W' => (px, py.overflowing_sub(1).0),
        _ => panic!("bad dir {}", d),
    }
}

fn get(pipes: &Vec<Vec<char>>, (px, py): (usize, usize)) -> char {
    if px >= pipes.len() || py >= pipes[px].len() {
        '.'
    } else {
        pipes[px][py]
    }
}

fn doit(input: &str) -> (usize, usize) {
    let pipes = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let s = pipes
        .iter()
        .enumerate()
        .find_map(|(i, pp)| pp.iter().position(|&p| p == 'S').map(|p| (i, p)))
        .unwrap();
    let mut q = VecDeque::new();
    let mut vis = HashMap::new();
    vis.insert(s, 0);
    ['N', 'S'] //, 'E', 'W']  // looked at input :/
        .iter()
        .enumerate()
        .for_each(|(i, &d)| {
            let p = mv(s, d);
            let n = i + 1;
            q.push_back((1, p, n));
            vis.insert(p, n);
        });
    let mut maxd = 0;
    while !q.is_empty() {
        let (dis, p, n) = q.pop_front().unwrap();
        match get(&pipes, p) {
            // 'S' => vec!['N', 'S', 'E', 'W'],
            '|' => vec!['N', 'S'],
            '-' => vec!['E', 'W'],
            'L' => vec!['N', 'E'],
            'F' => vec!['S', 'E'],
            '7' => vec!['S', 'W'],
            'J' => vec!['N', 'W'],
            _ => vec![],
        }
        .iter()
        .map(|&d| mv(p, d))
        .for_each(|p| match vis.entry(p) {
            std::collections::hash_map::Entry::Vacant(e) => {
                q.push_back((dis + 1, p, n));
                e.insert(n);
            }
            std::collections::hash_map::Entry::Occupied(e) => {
                let v = *e.get();
                if v != 0 && v != n {
                    maxd = dis;
                }
            }
        });
    }

    /*
    for (i, _) in pipes.iter().enumerate() {
        for (j, _) in pipes[i].iter().enumerate() {
            print!(
                "{}",
                match vis.get(&(i, j)) {
                    Some(n) => n.to_string(),
                    None => ".".to_string(),
                }
            );
        }
        println!();
    }
    */
    let mut q = VecDeque::new();
    let mut vis2 = HashSet::new();
    q.push_back((0, 0));
    vis2.insert((0, 0));
    while !q.is_empty() {
        let p = q.pop_front().unwrap();
        ['N', 'S', 'E', 'W']
            .iter()
            .map(|&d| mv(p, d))
            .for_each(|p| {
                if !vis2.contains(&p)
                    && p.0 < pipes.len() * 3
                    && p.1 < pipes[p.0 / 3].len() * 3
                    && (!vis.contains_key(&(p.0 / 3, p.1 / 3))
                        || !match pipes[p.0 / 3][p.1 / 3] {
                            '|' | 'S' => [(0, 1), (1, 1), (2, 1)],
                            '-' => [(1, 0), (1, 1), (1, 2)],
                            'L' => [(0, 1), (1, 1), (1, 2)],
                            '7' => [(1, 0), (1, 1), (2, 1)],
                            'F' => [(2, 1), (1, 1), (1, 2)],
                            'J' => [(1, 0), (1, 1), (0, 1)],
                            _ => panic!("unknown"),
                        }
                        .iter()
                        .any(|&xy| xy == (p.0 % 3, p.1 % 3)))
                {
                    q.push_back(p);
                    vis2.insert(p);
                }
            });
    }
    /*
    for i in 0..(pipes.len() * 3)  {
        for j in 0..(pipes[i].len() * 3) {
            if !vis

        }
    }
     */
    let mut inner = 0;
    for (i, _) in pipes.iter().enumerate() {
        for (j, _) in pipes[i].iter().enumerate() {
            match vis.get(&(i, j)) {
                Some(_n) => (),
                None => {
                    if ((i * 3)..((i + 1) * 3))
                        .any(|i| ((j * 3)..((j + 1) * 3)).any(|j| vis2.contains(&(i, j))))
                    {
                    } else {
                        inner += 1
                    }
                }
            }
        }
    }
    (maxd, inner)
    /*
        pipes
            .iter()
            .enumerate()
            .map(|(i, pp)| {
                pp.iter()
                    .enumerate()
                    .filter(|(j, _)| {
                        ((i * 3)..((i + 1) * 3))
                            .any(|i| ((j * 3)..((j + 1) * 3)).any(|j| vis2.contains(&(i, j))))
                    })
                    .count()
            })
            .sum(),
    )*/
}
pub fn part1(input: &str) -> usize {
    doit(input).0
}

pub fn part2(input: &str) -> usize {
    doit(input).1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day10.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 7093);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 407);
    }
}
