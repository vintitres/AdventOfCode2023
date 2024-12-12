enum Pulse {
    Low,
    High,
}

type Destinations = Vec<String>;

enum Module {
    Broadcast(BroadcastModule),
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule),
}

struct BroadcastModule {
    destinations: Destinations,
}

struct ConjunctionModule {
    destinations: Destinations,
}

struct FlipFlopModule {
    destinations: Destinations,
}

pub fn part1(input: &str) -> usize {
    input.len()
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day20.txt")
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
