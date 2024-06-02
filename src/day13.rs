use std::hash::{DefaultHasher, Hash, Hasher};

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn find_smudges(s1: &str, s2: &str) -> usize {
    s1.chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

fn find_mirrors(hashes: &[(u64, String)], smudges: usize) -> Vec<u64> {
    let mut mirrors = vec![];
    for i in 0..(hashes.len() - 1) {
        let mut j = i;
        let mut k = i + 1;
        let mut mirror = true;
        let mut smudges_found = 0;
        while k < hashes.len() {
            if hashes[j].0 != hashes[k].0 {
                if smudges_found < smudges {
                    smudges_found += find_smudges(&hashes[j].1, &hashes[k].1);
                    if smudges_found > smudges {
                        mirror = false;
                        break;
                    }
                } else {
                    mirror = false;
                    break;
                }
            }
            if j == 0 {
                break;
            }
            j -= 1;
            k += 1;
        }
        if mirror && smudges_found == smudges {
            mirrors.push((i + 1).try_into().unwrap());
        }
    }
    mirrors
}

fn doit(input: &str, smudges: usize) -> u64 {
    input.split("\n\n").map(|s| doith(s, smudges)).sum()
}
fn doith(input: &str, smudges: usize) -> u64 {
    let chars: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut lines_hashes = Vec::new();
    for i in 0..chars.len() {
        let mut line = String::new();
        for j in 0..chars[0].len() {
            line.push(chars[i][j]);
        }
        lines_hashes.push((calculate_hash(&line), line));
    }
    let mut column_hashes = Vec::new();
    for i in 0..chars[0].len() {
        let mut column = String::new();
        for j in 0..chars.len() {
            column.push(chars[j][i]);
        }
        column_hashes.push((calculate_hash(&column), column));
    }
    // dbg!(&column_hashes, find_mirrors(&column_hashes));
    // dbg!(&lines_hashes, find_mirrors(&lines_hashes));

    let l: u64 = find_mirrors(&lines_hashes, smudges).into_iter().sum();
    let c: u64 = find_mirrors(&column_hashes, smudges).into_iter().sum();
    100 * l + c
}

pub fn part1(input: &str) -> u64 {
    doit(input, 0)
}
pub fn part2(input: &str) -> u64 {
    doit(input, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day13.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 27742);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 32728);
    }
}
