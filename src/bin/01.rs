advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut ans = 0;
    for line in input.lines() {
        const NOT_FOUND: u32 = 11;
        let mut first_digit = NOT_FOUND;
        let mut last_digit = NOT_FOUND;
        for char in line.chars() {
            if char.is_ascii_digit() {
                if first_digit == NOT_FOUND {
                    first_digit = char::to_digit(char, 10).unwrap();
                }
                last_digit = char::to_digit(char, 10).unwrap();
            }
        }
        assert_ne!(first_digit, NOT_FOUND);
        assert_ne!(last_digit, NOT_FOUND);
        ans += first_digit * 10 + last_digit;
    }
    Some(ans as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut ans = 0;
    for line in input.lines() {
        // first convert to digits
        let mut line: String = line.to_string();
        // let mut line_with_digits: String = line.clone();
        for (i, v) in digits.iter().enumerate() {
            let digit: u32 = (i + 1).try_into().unwrap();
            // Insert the digit in the middle of the number string to account for situations where words meld together
            // Like "twone"
            if let Some(idx) = line.find(v) {
                line.insert(idx + 1, char::from_digit(digit, 10).unwrap());
            }
            if let Some(idx) = line.rfind(v) {
                line.insert(idx + 1, char::from_digit(digit, 10).unwrap());
            }
            // line = line.replace(v, &char::from_digit(digit, 10).unwrap().to_string());
        }

        const NOT_FOUND: u32 = 11;
        let mut first_digit = NOT_FOUND;
        let mut last_digit = NOT_FOUND;
        for char in line.chars() {
            if char.is_ascii_digit() {
                if first_digit == NOT_FOUND {
                    first_digit = char::to_digit(char, 10).unwrap();
                }
                last_digit = char::to_digit(char, 10).unwrap();
            }
        }
        assert_ne!(first_digit, NOT_FOUND);
        assert_ne!(last_digit, NOT_FOUND);
        ans += first_digit * 10 + last_digit;
    }
    Some(ans as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let inp = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let result = part_two(inp);
        assert_eq!(result, Some(281));
    }
}
