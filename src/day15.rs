fn reindeer_hash(word: &str) -> usize {
    word.chars()
        .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

pub fn part1(input: &str) -> u64 {
    input.replace('\n', "").split(',').map(reindeer_hash).map(|h| h as u64).sum()
}

enum LensInstrAction {
    Add(usize),
    Remove,
}

struct LensInstr {
    label: String,
    action: LensInstrAction,
}

impl LensInstr {
    fn new(spec: &str) -> LensInstr{
        if spec.ends_with('-') {
            LensInstr {
                label: spec[0..(spec.len() - 1)].to_string(),
                action: LensInstrAction::Remove,
            }
        } else {
            // dbg!(spec.chars().last().unwrap());
            LensInstr {
                label: spec[0..(spec.len() - 2)].to_string(),
                action: LensInstrAction::Add(spec.chars().last().unwrap() as usize - '0' as usize),
            }
        }
    }
}

#[derive(Debug,Clone)]
struct Lens {
    power: usize,
    label: String,
}

#[derive(Debug,Clone)]
struct LensBox {
    lenses: Vec<Lens>,
}
impl LensBox {
    fn new() -> LensBox {
        LensBox { lenses: vec![] }
    }
    fn add(&mut self, added_lens: Lens) {
        for lens in &mut self.lenses {
            if lens.label == added_lens.label {
                lens.power = added_lens.power;
                return;
            }
        }
        self.lenses.push(added_lens);
    }
    fn remove(&mut self, label: String) {
        if let Some(index) = self.lenses.iter().position(|lens| *lens.label == label) {
            self.lenses.remove(index);
        }
    }
    fn val(&self) -> u64 {
        self.lenses.iter().enumerate().map(|(i,l)| (i + 1) as u64 * l.power as u64).sum()
    }
}

pub fn part2(input: &str) -> u64 {
    let mut boxes = Vec::new();
    boxes.resize(256, LensBox::new());
    for lens_instr in input.replace('\n', "").split(',').map(LensInstr::new) {
        match lens_instr.action {
            LensInstrAction::Add(power) => {
                let lens = Lens { power, label: lens_instr.label };
                boxes[reindeer_hash(&lens.label.clone())].add(lens);
            },
            LensInstrAction::Remove => {
                boxes[reindeer_hash(&lens_instr.label.clone())].remove(lens_instr.label);
            },
        }
    };
    boxes.iter().enumerate().map(|(i,b)| (i + 1) as u64 * b.val()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2023/day15.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 503487);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 261505);
    }
}
