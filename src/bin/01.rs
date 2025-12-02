advent_of_code::solution!(1);

fn parse_input(input: &str) -> Vec<i64> {
    input
        .replace("L", "-")
        .replace("R", "+")
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {

    let parsed: Vec<i64> = parse_input(input);

    let mut position: i64 = 50;
    let mut count: u64 = 0;

    for step in parsed {
        position += step;
        position %= 100;
        if position == 0 {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {

    let parsed: Vec<i64> = parse_input(input);

    let mut position: i64 = 50;
    let mut count: u64 = 0;

    for step in parsed {
        let mut step1 = step;

        count += (step1 / 100).unsigned_abs();
        step1 = step1 % 100;

        if position != 0 && (step1 < 0 && position + step1 < 0) || (step1 > 0 && position + step1 >= 101) {
            count += 1;
        }

        position += step;
        position = ((position % 100) + 100) % 100;
        if position == 0 {
            count += 1;
        }
    }

    Some(count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
