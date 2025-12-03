advent_of_code::solution!(3);

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<u64>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let banks = parse_input(input);

    let mut output = 0;

    for bank in banks {
        let max_value = bank.iter().take(bank.len() - 1).max().unwrap();
        let max_index = bank.iter().position(|&x| x == *max_value).unwrap();
        let second_max_value = bank.iter().skip(max_index + 1).max().unwrap();
        let joltaje = 10 * max_value + second_max_value;
        output += joltaje;
    }
    Some(output)
}

pub fn part_two(input: &str) -> Option<u64> {
    let banks = parse_input(input);

    let mut output = 0;

    for bank in banks {
        let v1 = bank.iter().take(bank.len() - 11).max().unwrap();
        let i1 = bank.iter().position(|&x| x == *v1).unwrap();

        let v2 = bank.iter().take(bank.len() - 10).skip(i1 + 1).max().unwrap();
        let i2 = i1 + 1 + bank.iter().skip(i1 + 1).position(|&x| x == *v2).unwrap();

        let v3 = bank.iter().take(bank.len() - 9).skip(i2 + 1).max().unwrap();
        let i3 = i2 + 1 + bank.iter().skip(i2 + 1).position(|&x| x == *v3).unwrap();

        let v4 = bank.iter().take(bank.len() - 8).skip(i3 + 1).max().unwrap();
        let i4 = i3 + 1 + bank.iter().skip(i3 + 1).position(|&x| x == *v4).unwrap();

        let v5 = bank.iter().take(bank.len() - 7).skip(i4 + 1).max().unwrap();
        let i5 = i4 + 1 + bank.iter().skip(i4 + 1).position(|&x| x == *v5).unwrap();

        let v6 = bank.iter().take(bank.len() - 6).skip(i5 + 1).max().unwrap();
        let i6 = i5 + 1 + bank.iter().skip(i5 + 1).position(|&x| x == *v6).unwrap();

        let v7 = bank.iter().take(bank.len() - 5).skip(i6 + 1).max().unwrap();
        let i7 = i6 + 1 + bank.iter().skip(i6 + 1).position(|&x| x == *v7).unwrap();

        let v8 = bank.iter().take(bank.len() - 4).skip(i7 + 1).max().unwrap();
        let i8 = i7 + 1 + bank.iter().skip(i7 + 1).position(|&x| x == *v8).unwrap();

        let v9 = bank.iter().take(bank.len() - 3).skip(i8 + 1).max().unwrap();
        let i9 = i8 + 1 + bank.iter().skip(i8 + 1).position(|&x| x == *v9).unwrap();

        let v10 = bank.iter().take(bank.len() - 2).skip(i9 + 1).max().unwrap();
        let i10 = i9 + 1 + bank.iter().skip(i9 + 1).position(|&x| x == *v10).unwrap();

        let v11 = bank.iter().take(bank.len() - 1).skip(i10 + 1).max().unwrap();
        let i11 = i10 + 1 + bank.iter().skip(i10 + 1).position(|&x| x == *v11).unwrap();

        let v12 = bank.iter().skip(i11 + 1).max().unwrap();
        //let i12 = i11 + bank.iter().skip(i11 + 1).position(|&x| x == *v12).unwrap();


        let joltaje = v1 * 100000000000
            + v2 * 10000000000
            + v3 * 1000000000
            + v4 * 100000000
            + v5 * 10000000
            + v6 * 1000000
            + v7 * 100000
            + v8 * 10000
            + v9 * 1000
            + v10 * 100
            + v11 * 10
            + v12;

        output += joltaje;
    }
    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
