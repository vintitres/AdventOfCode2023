use std::collections::BTreeMap;

use itertools::Itertools;

fn group_exp(exp: &[usize]) -> BTreeMap<usize, usize> {
    let mut last = None;
    let mut gexp = BTreeMap::new();
    exp.iter().for_each(|&e| match last {
        None => {
            last = Some((e, 1));
        }
        Some((b, l)) => {
            if b + l == e {
                last = Some((b, l + 1));
            } else {
                gexp.insert(b, l);
                last = Some((e, 1));
            }
        }
    });
    if let Some(l) = last {
        gexp.insert(l.0, l.1);
    }
    gexp
}

fn dist_(g1: usize, g2: usize, exp: &BTreeMap<usize, usize>, mul: u64) -> u64 {
    let b = std::cmp::min(g1, g2);
    let e = std::cmp::max(g1, g2);

    let mut bb = b;
    let mut d = 0;
    for (&s, &l) in exp
        .iter()
        .skip_while(|(&k, _)| k < b)
        .take_while(|(&k, _)| k < e)
    {
        d += s as u64 - bb as u64 + l as u64 * mul;
        bb = s + l;
        // dbg!(s, l);
        // dbg!(d);
        // dbg!(bb);
    }
    // dbg!(d);
    d + e as u64 - bb as u64
}
fn dist(
    g1: &(usize, usize),
    g2: &(usize, usize),
    exp_x: &BTreeMap<usize, usize>,
    exp_y: &BTreeMap<usize, usize>,
    mul: u64,
) -> u64 {
    dist_(g1.0, g2.0, exp_x, mul) + dist_(g1.1, g2.1, exp_y, mul)
}

fn doit(input: &str, mul: u64) -> u64 {
    let galax = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let max_x = galax.len();
    let max_y = galax[0].len();
    let mut exp_x = Vec::new();
    let mut exp_y = Vec::new();
    for (x, galax_x) in galax.iter().enumerate() {
        if galax_x.iter().all(|&g| g == '.') {
            exp_x.push(x);
        }
    }
    for y in 0..max_y {
        if (0..max_x).all(|x| galax[x][y] == '.') {
            exp_y.push(y);
        }
    }
    let exp_x = group_exp(&exp_x);
    let exp_y = group_exp(&exp_y);
    let galax = galax
        .iter()
        .enumerate()
        .flat_map(|(x, gg)| {
            gg.iter()
                .enumerate()
                .flat_map(move |(y, &g)| if g == '#' { Some((x, y)) } else { None })
        })
        .collect_vec();
    // dist(&galax[7], &galax[8], &exp_x, &exp_y)
    galax
        .iter()
        .enumerate()
        .map(|(gi, g1)| {
            galax
                .iter()
                .enumerate()
                .skip(gi + 1)
                .map(|(_gj, g2)| {
                    /*dbg!((gi + 1, gj + 1, d));*/
                    dist(g1, g2, &exp_x, &exp_y, mul)
                })
                .sum::<u64>()
        })
        .sum()
}

pub fn part1(input: &str) -> u64 {
    doit(input, 2)
}

pub fn part2(input: &str) -> u64 {
    doit(input, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day11.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 10228230);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 22);
    }
}
