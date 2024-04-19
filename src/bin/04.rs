use regex::Regex;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut overlap: u32 = 0;
    let re = Regex::new(r#"^(\d+)-(\d+),(\d+)-(\d+)$"#).unwrap();
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let start_first = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let end_first = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let start_second = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();
        let end_second = captures.get(4).unwrap().as_str().parse::<u32>().unwrap();
        if (start_first >= start_second && end_first <= end_second)
            || (start_second >= start_first && end_second <= end_first)
        {
            overlap += 1;
        }
    }
    Some(overlap)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut overlap: u32 = 0;
    let re = Regex::new(r#"^(\d+)-(\d+),(\d+)-(\d+)$"#).unwrap();
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let start_first = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let end_first = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let start_second = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();
        let end_second = captures.get(4).unwrap().as_str().parse::<u32>().unwrap();
        if (start_second <= end_first && start_second >= start_first)
            || (start_first <= end_second && start_first >= start_second)
        {
            overlap += 1;
        }
    }
    Some(overlap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
