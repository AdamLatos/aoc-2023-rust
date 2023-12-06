advent_of_code::solution!(2);
use std::collections::BTreeMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut max_vals = BTreeMap::new();
    max_vals.insert("red", 12);
    max_vals.insert("green", 13);
    max_vals.insert("blue", 14);
    let mut total = 0;

    for line in input.lines() {
        // line:
        //  Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        let mut it = line.split(":");
        let game_num: String = it.next().unwrap().chars().filter(|c| c.is_digit(10)).collect();
        let game_num = u32::from_str_radix(&game_num, 10).unwrap();
        let mut possible = true;

        for game in it.next().unwrap().split(";").map(|g| g.trim()) {
            // game:
            //  1 green, 1 blue
            for cube in game.split(",").map(|g| g.trim()) {
                let mut it = cube.split(" ");
                let num = u32::from_str_radix(it.next().unwrap(), 10).unwrap();
                if num > max_vals[it.next().unwrap()] {
                    possible = false;
                }
            }
        }

        if possible {
            total += game_num;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut powers = Vec::new();

    for line in input.lines() {
        let mut min_vals = BTreeMap::new();
        min_vals.insert("red", 0);
        min_vals.insert("green", 0);
        min_vals.insert("blue", 0);
        let mut it = line.split(":");
        // Skip game number text
        it.next(); 

        for game in it.next().unwrap().split(";").map(|g| g.trim()) {
            for cube in game.split(",").map(|g| g.trim()) {
                let mut it = cube.split(" ");
                let num = u32::from_str_radix(it.next().unwrap(), 10).unwrap();
                let color = it.next().unwrap();
                if num > min_vals[color] {
                    min_vals.insert(color, num);
                }
            }
        }
        powers.push(min_vals["red"] * min_vals["green"] * min_vals["blue"]);
    }


    let powers_sum = powers.iter().fold(0, |acc, e| acc + e);

    Some(powers_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &'static str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part_one() {
        let result = part_one(TEST_INPUT_1);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(TEST_INPUT_1);
        assert_eq!(result, Some(2286));
    }
}
