use std::collections::BinaryHeap;

advent_of_code::solution!(1);

fn get_calories(input: &str) -> BinaryHeap<u32> {
    let mut elf_calorie_heap: BinaryHeap<u32> = BinaryHeap::new();
    let mut elf_calories: u32 = 0;
    for line in input.lines() {
        if line.len() == 0 {
            elf_calorie_heap.push(elf_calories);
            elf_calories = 0;
        } else {
            let new_value: u32 = u32::from_str_radix(line, 10).unwrap();
            elf_calories += new_value;
        }
    }
    elf_calorie_heap.push(elf_calories);
    elf_calorie_heap
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut calories: BinaryHeap<u32> = get_calories(input);
    calories.pop()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calories: BinaryHeap<u32> = get_calories(input);
    Some(calories.pop().unwrap() + calories.pop().unwrap() + calories.pop().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45000));
    }
}
