advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input.split(",").map(|s| s.split("-").collect()).collect()
}

fn check_part_one(num: u64) -> bool {
    let s = num.to_string();
    if s.len() % 2 != 0 {
        return false;
    }
    s[0..(s.len() / 2)] == s[(s.len() / 2)..s.len()]
}

fn check_part_two(num: u64) -> bool {
    let s = num.to_string();

    if s.len() == 1 {
        return false;
    }

    // 1. all digits are the same
    if s.chars()
        .next()
        .map(|f| s.chars().all(|c| c == f))
        .unwrap_or(false)
    {
        return true;
    }

    // 2. pairs of digits
    if s.len() < 4 {
        return false;
    }

    let chars = s.chars().collect::<Vec<char>>();
    let pair = [chars[0], chars[1]];
    if chars.chunks(2).all(|chunk| chunk == pair) {
        return true;
    }

    // 3. triplets of digits
    if s.len() < 6 {
        return false;
    }
    let triplet = [chars[0], chars[1], chars[2]];
    if chars.chunks(3).all(|chunk| chunk == triplet) {
        return true;
    }

    // 4. quadruplets of digits
    if s.len() < 8 {
        return false;
    }
    let quadruplet = [chars[0], chars[1], chars[2], chars[3]];
    if chars.chunks(4).all(|chunk| chunk == quadruplet) {
        return true;
    }

    // 5. quintuplets of digits
    if s.len() < 10 {
        return false;
    }
    let quintuplet = [chars[0], chars[1], chars[2], chars[3], chars[4]];
    if chars.chunks(5).all(|chunk| chunk == quintuplet) {
        return true;
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_input(input);
    let mut sum = 0;

    for range in ranges {
        let start = range[0];
        let end = range[1];
        let start_num: u64 = start.parse().unwrap();
        let end_num: u64 = end.parse().unwrap();
        for num in start_num..=end_num {
            if check_part_one(num) {
                sum += num;
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_input(input);
    let mut sum = 0;

    for range in ranges {
        let start = range[0];
        let end = range[1];
        let start_num: u64 = start.parse().unwrap();
        let end_num: u64 = end.parse().unwrap();
        for num in start_num..=end_num {
            if check_part_two(num) {
                sum += num;
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
