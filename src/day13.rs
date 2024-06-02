use std::hash::{DefaultHasher, Hasher, Hash};

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn find_mirrors(hashes: &[u64]) -> Vec<u64> {
    let mut mirrors = vec![];
    for i in 0..(hashes.len() - 1) {
        let mut j = i;
        let mut k = i + 1;
        let mut mirror = true;
        while k < hashes.len() {
            if hashes[j] != hashes[k] {
                mirror = false;
                break;
            }
            if j == 0 {
                break;
            }
            j -= 1;
            k += 1;
        }
        if mirror {
            mirrors.push((i + 1).try_into().unwrap());
        }
    }
    mirrors
} 

pub fn part1(input: &str) -> u64 {
    input.split("\n\n").map(part1h).sum()
}
fn part1h(input: &str) -> u64 {
    let chars : Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut lines_hashes = Vec::new();
    for i in 0..chars.len() {
        let mut line = String::new();
        for j in 0..chars[0].len() {
            line.push(chars[i][j]);
        }
        lines_hashes.push(calculate_hash(&line));
    }
    let mut column_hashes = Vec::new();
    for i in 0..chars[0].len() {
        let mut column = String::new();
        for j in 0..chars.len() {
            column.push(chars[j][i]);
        }
        column_hashes.push(calculate_hash(&column));
    }
    // dbg!(&column_hashes, find_mirrors(&column_hashes));
    // dbg!(&lines_hashes, find_mirrors(&lines_hashes));

    let l: u64= find_mirrors(&lines_hashes).into_iter().sum();
    let c: u64 = find_mirrors(&column_hashes).into_iter().sum();
    100 * l + c
}

pub fn part2(input: &str) -> usize {
    input.len()
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

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 22);
    }
}
