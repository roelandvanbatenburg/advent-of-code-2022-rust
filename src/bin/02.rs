advent_of_code::solution!(2);

fn score_game_1(you: &str, elf: &str) -> u32 {
    match (you, elf) {
        ("X", "A") => 3 + 1, // Draw with Rock
        ("X", "B") => 0 + 1, // Lose with Rock
        ("X", "C") => 6 + 1, // Win with Rock
        ("Y", "A") => 6 + 2, // Win with Paper
        ("Y", "B") => 3 + 2, // Draw with Paper
        ("Y", "C") => 0 + 2, // Lose with Paper
        ("Z", "A") => 0 + 3, // Lose with Scissors
        ("Z", "B") => 6 + 3, // Win with Scissors
        ("Z", "C") => 3 + 3, // Draw with Scissors
        _ => panic!(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut score: u32 = 0;
    for line in input.lines() {
        let (elf, you) = match line.split_once(" ") {
            Some((a, b)) => (a, b),
            None => panic!(),
        };
        score += score_game_1(you, elf);
    }
    Some(score)
}

fn score_game_2(you: &str, elf: &str) -> u32 {
    match (you, elf) {
        ("X", "A") => 0 + 3, // Lose vs Rock (Scissors)
        ("X", "B") => 0 + 1, // Lose vs Paper (Rock)
        ("X", "C") => 0 + 2, // Lose vs Scissors (Paper)
        ("Y", "A") => 3 + 1, // Draw vs Rock (Rock)
        ("Y", "B") => 3 + 2, // Draw vs Paper (Paper)
        ("Y", "C") => 3 + 3, // Draw vs Scissors (Scissors)
        ("Z", "A") => 6 + 2, // Win vs Rock (Paper)
        ("Z", "B") => 6 + 3, // Win vs Paper (Scissors)
        ("Z", "C") => 6 + 1, // Win vs Scissors (Rock)
        _ => panic!(),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score: u32 = 0;
    for line in input.lines() {
        let (elf, you) = match line.split_once(" ") {
            Some((a, b)) => (a, b),
            None => panic!(),
        };
        score += score_game_2(you, elf);
    }
    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
