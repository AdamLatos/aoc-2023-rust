advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let races_vec: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .skip(1)
                .map(|n| u32::from_str_radix(n, 10).unwrap())
                .collect()
        })
        .collect();
    let times = &races_vec[0];
    let records = &races_vec[1];

    println!("{:?}", times);
    println!("{:?}", records);

    let mut res = 1;

    for (time, record) in times.iter().zip(records) {
        let mut wins = 0;
        for button_press_time in 0..*time {
            let speed = button_press_time;
            let time_left = time - button_press_time;
            let distance_covered = speed * time_left;
            if distance_covered > *record {
                wins += 1;
            }
        }
        res *= wins;
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {

    let races_vec: Vec<String> = input
        .lines()
        .map(|l| l.split_ascii_whitespace().skip(1).collect::<String>())
        .collect();

    let time = u64::from_str_radix(&races_vec[0], 10).unwrap();
    let record = u64::from_str_radix(&races_vec[1], 10).unwrap();

    println!("{} {}", time, record);

    let mut wins = 0;
    for button_press_time in 0..time {
        let speed = button_press_time;
        let time_left = time - button_press_time;
        let distance_covered = speed * time_left;
        if distance_covered > record {
            wins += 1;
        }
    }

    Some(wins)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "Time:      7  15   30
    Distance:  9  40  200";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, Some(71503));
    }
}
