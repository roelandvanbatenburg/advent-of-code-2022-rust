use std::collections::HashSet;

advent_of_code::solution!(3);

fn ascii(compartment: &str) -> HashSet<u8> {
    compartment
        .bytes()
        .map(|char| match char {
            b'a'..=b'z' => char - b'a' + 1,
            b'A'..=b'Z' => char - b'A' + 27,
            _ => panic!(),
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum_priority: u32 = 0;
    for line in input.lines() {
        let mid: usize = line.len() / 2;
        let (one, two): (HashSet<u8>, HashSet<u8>) = match line.split_at(mid) {
            (one, two) => (ascii(one), ascii(two)),
        };
        let duplicate = match one.intersection(&two).next() {
            Some(duplicate) => *duplicate,
            None => panic!(),
        };
        sum_priority += u32::from(duplicate);
    }
    Some(sum_priority)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut badge_priorty: u32 = 0;
    let backpacks: Vec<HashSet<u8>> = input.lines().map(ascii).collect();
    for group in backpacks.chunks(3) {
        let badge = if let [elf1, elf2, elf3] = group {
            // Using intersection twice is unwieldy as it returns a HashSet<&u8>
            match elf1.intersection(elf2).filter(|&x| elf3.contains(x)).next() {
                Some(&badge) => badge,
                None => panic!(),
            }
        } else {
            panic!()
        };
        badge_priorty += u32::from(badge)
    }
    Some(badge_priorty)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(157));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(70));
    }
}
