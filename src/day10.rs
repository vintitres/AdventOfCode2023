use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

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

pub fn part1(input: &str) -> usize {
    let pipes = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let s = pipes
        .iter()
        .enumerate()
        .find_map(|(i, ref pp)| match pp.iter().position(|&p| p == 'S') {
            Some(p) => Some((i,p)),
            None => None,
        } )
        .unwrap();
    let mut q = VecDeque::new();
    let mut vis = HashMap::new();
    vis.insert(s, 0);
    vec!['N', 'S'] //, 'E', 'W']  // input
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
        dbg!((dis, p, n));
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
        .for_each(|p| {
            if !vis.contains_key(&p) {
                q.push_back((dis + 1, p, n));
                vis.insert(p, n);
            } else {
                let vp = vis[&p];
                if vp != 0 && vp != n {
                    maxd = dis;
                }
            }
        });
    }
    for i in 0..pipes.len() {
        for j in 0..pipes[i].len() {
            print!("{}", match vis.get(&(i,j)) {Some(n) => n.to_string(), None => ".".to_string()});
        }
        println!("");
    }
    maxd
}

pub fn part2(input: &str) -> usize {
    input.len()
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

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 22);
    }
}
