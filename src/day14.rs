use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

fn tilt_up(platform: &mut [Vec<char>]) {
    for i in 0..platform.len() {
        for j in 0..platform[0].len() {
            if platform[i][j] == 'O' {
                for k in (0..=i).rev() {
                    if k > 0 && platform[k - 1][j] == '.' {
                        platform[k][j] = '.';
                        platform[k - 1][j] = 'O';
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn transpose(platform: &mut [Vec<char>]) {
    for i in 0..platform.len() {
        for j in i..platform[0].len() {
            let x = platform[i][j];
            platform[i][j] = platform[j][i];
            platform[j][i] = x;
        }
    }
}
fn flip_rows(platform: &mut [Vec<char>]) {
    let row_len = platform[0].len();
    for i in 0..platform.len() {
        for j in 0..(row_len / 2) {
            platform[i].swap(j, row_len - 1 - j);
        }
    }
}

fn flip(platform: &mut [Vec<char>]) {
    transpose(platform);
    flip_rows(platform);
}

fn calc_load(platform: &[Vec<char>]) -> u64 {
    let mut load = 0;
    for i in 0..platform.len() {
        for j in 0..platform[0].len() {
            if platform[i][j] == 'O' {
                load += (platform.len() - i) as u64;
            }
        }
    }
    load
}

fn print(platform: &[Vec<char>]) {
    for i in 0..platform.len() {
        let s: String = platform[i].clone().into_iter().collect();
        dbg!(s);
    }
}
pub fn part1(input: &str) -> u64 {
    let mut platform: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    tilt_up(&mut platform);
    calc_load(&platform)
}

fn cycle(platform: &mut [Vec<char>]) {
    for _ in 0..4 {
        tilt_up(platform);
        flip(platform);
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn calc_hash(platform: &[Vec<char>]) -> u64 {
    let mut s = String::new();
    for row in platform {
        s.extend(row.iter());
    }
    calculate_hash(&s)
}

pub fn part2(input: &str) -> u64 {
    let mut platform: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut seen_hashes = HashMap::<u64, usize>::new();
    const CYCLES: usize = 1000000000;
    seen_hashes.insert(calc_hash(&platform), 0);
    let mut left_cycles = 0;
    for i in 1..CYCLES {
        cycle(&mut platform);
        let hash = calc_hash(&platform);
        if seen_hashes.contains_key(&hash) {
            left_cycles = (CYCLES - i) % (i - seen_hashes.get(&hash).unwrap());
            break;
        }
        seen_hashes.insert(hash, i);
    }
    for _ in 0..left_cycles {
        cycle(&mut platform);
    }
    print(&platform);
    calc_load(&platform)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day14.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 110407);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 87273);
    }
}
