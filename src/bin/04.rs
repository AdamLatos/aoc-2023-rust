advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut points = 0;

    for line in input.lines() {
        let mut wins = 0;
        let line = line.split(":").skip(1).next().unwrap();
        let mut it = line.split("|");
        let winning_nums = it.next().unwrap();
        let my_nums = it.next().unwrap();

        let winning_nums: Vec<u32> = winning_nums.split(" ").filter_map(|num| u32::from_str_radix(num, 10).ok()).collect();
        let my_nums: Vec<u32> = my_nums.split(" ").filter_map(|num| u32::from_str_radix(num, 10).ok()).collect();
        println!("{:?}", winning_nums);
        println!("{:?}", my_nums);

        for win_num in winning_nums {
            wins += my_nums.iter().filter(|n| **n == win_num).count();
        }

        if wins > 0 {
            points += 2_u32.pow(wins as u32 - 1);
        }
    }

    Some(points)
}

use std::collections::BTreeMap;

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards = 0;

    let mut clones: BTreeMap<u32, u32> = BTreeMap::new();
    for line in input.lines() {
        let game: String = line.split(":").next().unwrap().chars().filter(|c| c.is_ascii_digit()).collect();
        let game = u32::from_str_radix(&game, 10).unwrap();
        clones.insert(game, 0);
    }

    for line in input.lines() {
        let mut wins: u32 = 0;

        let game: String = line.split(":").next().unwrap().chars().filter(|c| c.is_ascii_digit()).collect();
        let game = u32::from_str_radix(&game, 10).unwrap();

        let line = line.split(":").skip(1).next().unwrap();
        let mut it = line.split("|");
        let winning_nums = it.next().unwrap();
        let my_nums = it.next().unwrap();

        let winning_nums: Vec<u32> = winning_nums.split(" ").filter_map(|num| u32::from_str_radix(num, 10).ok()).collect();
        let my_nums: Vec<u32> = my_nums.split(" ").filter_map(|num| u32::from_str_radix(num, 10).ok()).collect();

        for win_num in winning_nums {
            wins += my_nums.iter().filter(|n| **n == win_num).count() as u32;
        }

        let clones_num = clones[&game];
        if wins > 0 {
            for g in game+1..=game+wins {
                let current_clones = clones[&g];
                clones.insert(g, current_clones + (1 + clones_num));
            }
        }
        cards += 1;
        cards += clones_num;
    }

    Some(cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, Some(30));
    }
}
