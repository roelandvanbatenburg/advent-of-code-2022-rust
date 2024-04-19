use regex::Regex;

advent_of_code::solution!(5);

fn reverse_stacks(stacks: &mut Vec<Vec<String>>) {
    for stack in stacks.iter_mut() {
        stack.reverse();
    }
}

fn parse_cranes(input: &str) -> Vec<Vec<String>> {
    let cranes_cnt: usize = input
        .lines()
        .last()
        .and_then(|line| line.split_whitespace().last())
        .and_then(|word| word.parse::<usize>().ok())
        .unwrap();
    let mut vec: Vec<Vec<String>> = vec![Vec::new(); cranes_cnt];
    for line in input.lines() {
        if line.starts_with(" 1") {
            break;
        }
        let row: Vec<String> = line
            .chars()
            .skip(1)
            .step_by(4)
            .map(|c| c.to_string())
            .collect();
        for (stack_i, elf_crate) in row.iter().enumerate() {
            if elf_crate != " " {
                vec[stack_i].push(elf_crate.clone());
            }
        }
    }
    reverse_stacks(&mut vec);
    vec
}

fn move_crate(stacks: &mut Vec<Vec<String>>, origin: usize, target: usize) {
    let elf_crate = stacks[origin - 1].pop().unwrap();
    stacks[target - 1].push(elf_crate);
}

fn solution(stacks: Vec<Vec<String>>) -> Option<String> {
    let solution = stacks
        .iter()
        .map(|s| s.last())
        .filter_map(|opt| opt) // Filter out None values
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join("");
    Some(solution)
}

pub fn part_one(input: &str) -> Option<String> {
    let (crane_input, instructions_input) = input.split_once("\n\n").unwrap();

    let mut stacks = parse_cranes(crane_input);

    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for instruction in instructions_input.lines() {
        let captures = re.captures(instruction).unwrap();
        let count = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let origin = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let target = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
        for _ in 0..count {
            move_crate(&mut stacks, origin, target);
        }
    }
    solution(stacks)
}

fn move_crate_2(stacks: &mut Vec<Vec<String>>, count: usize, origin: usize, target: usize) {
    let at = stacks[origin - 1].len();
    let mut moved_stack = stacks[origin - 1].split_off(at - count);
    stacks[target - 1].append(&mut moved_stack);
}

pub fn part_two(input: &str) -> Option<String> {
    let (crane_input, instructions_input) = input.split_once("\n\n").unwrap();

    let mut stacks = parse_cranes(crane_input);

    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for instruction in instructions_input.lines() {
        let captures = re.captures(instruction).unwrap();
        let count = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let origin = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let target = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
        move_crate_2(&mut stacks, count, origin, target);
    }
    solution(stacks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("MCD".to_string()));
    }
}
