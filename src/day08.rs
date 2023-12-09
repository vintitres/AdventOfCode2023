use itertools::Itertools;
use std::collections::HashMap;

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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Cycle {
    start: u64,
    len: u64,
    zs: Vec<u64>,
}

/*
Code for finding shifted lowest common multiply translated from python from
https://math.stackexchange.com/questions/2218763/how-to-find-lcm-of-two-numbers-when-one-starts-with-an-offset
answer by Eric Langlois
*/
fn combine_phased_rotations(
    a_period: i64,
    a_phase: i64,
    b_period: i64,
    b_phase: i64,
) -> (i64, i64) {
    /*mbine two phased rotations into a single phased rotation

    Returns: combined_period, combined_phase

    The combined rotation is at its reference point if and only if both a and b
    are at their reference points.
    */
    let (gcd, s, _t) = extended_gcd(a_period, b_period);
    let phase_difference = a_phase - b_phase;
    let (pd_mult, pd_remainder) = divmod(phase_difference, gcd);
    if pd_remainder > 0 {
        panic!("Rotation reference points never synchronize.");
    }
    let combined_period = a_period / gcd * b_period;
    let combined_phase = (a_phase - s * pd_mult * a_period) % combined_period;
    (combined_period, combined_phase)
}

fn divmod(a: i64, b: i64) -> (i64, i64) {
    (a / b, a % b)
}

fn align(c1: &Cycle, c2: &Cycle) -> Cycle {
    assert_eq!(c1.zs.len(), 1);
    assert_eq!(c2.zs.len(), 1);
    let (period, phase) = combine_phased_rotations(
        c1.len as i64,
        (c1.start + c1.zs.first().unwrap()) as i64,
        c2.len as i64,
        (c2.start + c2.zs.first().unwrap()) as i64,
    );
    Cycle {
        start: phase as u64,
        len: period as u64,
        zs: vec![0],
    }
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    /*Extended Greatest Common Divisor Algorithm

    Returns:
        gcd: The greatest common divisor of a and b.
        s, t: Coefficients such that s*a + t*b = gcd

    Reference:
        https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode
    */
    let mut rr = (a, b);
    let mut ss = (1, 0);
    let mut tt = (0, 1);
    while rr.1 > 0 {
        let (quotient, remainder) = divmod(rr.0, rr.1);
        rr = (rr.1, remainder);
        ss = (ss.1, ss.0 - quotient * ss.1);
        tt = (tt.1, tt.0 - quotient * tt.1);
    }
    (rr.0, ss.0, tt.0)
}

pub fn part2(input: &str) -> u64 {
    let (rl, map) = read(input);
    let cycles = map
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(k, _)| {
            let mut visited: HashMap<(usize, &str), u64> = HashMap::new();
            let mut i = 0u64;
            let mut node = k;
            let mut zs = Vec::new();
            loop {
                if node.ends_with('Z') {
                    zs.push(i);
                }
                let rli = (i % (rl.len() as u64)) as usize;
                if node.starts_with("22") {
                    dbg!(&visited);
                }
                if visited.contains_key(&(rli, *node)) {
                    break;
                }
                visited.insert((rli, node), i);
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
    dbg!(&cycles);
    let mut c: Cycle = cycles.first().unwrap().clone();
    for cc in cycles.iter().skip(1) {
        c = align(&c, cc);
    }
    dbg!(&c);
    c.len

    /*
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
    }*/
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 14935034899483);
    }
}
