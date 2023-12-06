advent_of_code::solution!(3);

fn mark_number(
    schematic: &Vec<Vec<char>>,
    adjacent_map: &mut Vec<Vec<bool>>,
    coords: (usize, usize),
) {
    if !schematic[coords.0][coords.1].is_ascii_digit() {
        return;
    }
    if adjacent_map[coords.0][coords.1] == true {
        return;
    }
    adjacent_map[coords.0][coords.1] = true;
    // check left coords
    if coords.1 > 0 {
        let left = (coords.0, coords.1 - 1);
        if schematic[left.0][left.1].is_ascii_digit() {
            mark_number(schematic, adjacent_map, left);
        }
    }
    // check right coords
    if coords.1 < schematic[coords.0].len() - 1 {
        let right = (coords.0, coords.1 + 1);
        if schematic[right.0][right.1].is_ascii_digit() {
            mark_number(schematic, adjacent_map, right);
        }
    }
}

fn neighbors(schematic: &Vec<Vec<char>>, point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut nbors = Vec::new();
    assert!(point.0 < schematic.len());
    assert!(point.1 < schematic[point.0].len());
    let point = (point.0 as i64, point.1 as i64);

    for i in (point.0 - 1)..=(point.0 + 1) {
        for j in (point.1 - 1)..=(point.1 + 1) {
            if i == point.0 && j == point.1 {
                continue;
            }
            if i < 0 || i >= schematic.len() as i64 {
                continue;
            }
            if j < 0 || j >= schematic[i as usize].len() as i64 {
                continue;
            }
            nbors.push((i as usize, j as usize));
        }
    }

    nbors
}

fn mark_adjacent(schematic: &Vec<Vec<char>>, adjacent_map: &mut Vec<Vec<bool>>) {
    for i in 0..schematic.len() {
        for j in 0..schematic[i].len() {
            let c = schematic[i][j];
            if (!c.is_ascii_digit()) && (c != '.') {
                // We have a symbol
                for (x, y) in neighbors(schematic, (i, j)) {
                    mark_number(schematic, adjacent_map, (x, y));
                }
            }
        }
    }
    println!("")
}

#[allow(dead_code)]
fn print_map(adjacent_map: &Vec<Vec<bool>>) {
    println!("---------------------------");
    for mask in adjacent_map {
        for m in mask {
            print!("{} ", if *m == true { "X" } else { " " });
        }
        println!("");
    }
    println!("---------------------------");
}

#[allow(dead_code)]
fn print_schematic(schematic: &Vec<Vec<char>>) {
    println!("---------------------------");
    for line in schematic {
        for c in line {
            print!("{}", c);
        }
        println!("");
    }
    println!("---------------------------");
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut schematic: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().filter(|c| !c.is_ascii_whitespace()).collect())
        .collect();
    let mut adjacent_map: Vec<Vec<bool>> = Vec::new();
    for i in 0..schematic.len() {
        let map_line = vec![false; schematic[i].len()];
        adjacent_map.push(map_line);
    }
    // print_schematic(&schematic);
    mark_adjacent(&schematic, &mut adjacent_map);
    // print_map(&adjacent_map);

    for (line, mask) in schematic.iter_mut().zip(adjacent_map.iter()) {
        for (c, m) in line.iter_mut().zip(mask.iter()) {
            if !m {
                *c = ' ';
            }
        }
    }

    for line in schematic {
        let line: String = line.into_iter().collect();
        let line = line.trim();
        for num in line.split(" ") {
            if num.is_empty() {
                continue;
            }
            sum += u32::from_str_radix(num, 10).unwrap();
        }
    }

    Some(sum)
}

fn gear_neighbor_nums(schematic: &Vec<Vec<char>>, point: (usize, usize)) -> Vec<u32> {
    let mut nbor_nums = Vec::new();
    
    let mut adjacent_map: Vec<Vec<bool>> = Vec::new();
    let first_line = if point.0 == 0 {0} else {point.0 - 1};
    let last_line = if point.0 == schematic.len() - 1 {point.0} else {point.0 + 1};
    for i in first_line..=last_line {
        let map_line = vec![false; schematic[i].len()];
        adjacent_map.push(map_line);
    }
    let mut schematic_slice = schematic[first_line..=last_line].to_owned();

    print_schematic(&schematic_slice);

    let nbors = neighbors(&schematic_slice, (point.0 - first_line, point.1));
    for nb in nbors {
        mark_number(&schematic_slice, &mut adjacent_map, nb);
    }

    for (line, mask) in schematic_slice.iter_mut().zip(adjacent_map.iter()) {
        for (c, m) in line.iter_mut().zip(mask.iter()) {
            if !m {
                *c = ' ';
            }
        }
    }

    print_schematic(&schematic_slice);

    for line in schematic_slice {
        let line: String = line.into_iter().collect();
        let line = line.trim();
        for num in line.split(" ") {
            if num.is_empty() {
                continue;
            }
            nbor_nums.push(u32::from_str_radix(num, 10).unwrap());
        }
    }

    print_map(&adjacent_map);

    nbor_nums
}

fn find_adjacent_to_gear(schematic: &Vec<Vec<char>>, gear_ratios: &mut Vec<u32>) {
    for i in 0..schematic.len() {
        for j in 0..schematic[i].len() {
            let c = schematic[i][j];
            if c == '*' {
                // We have a gear
                let nbor_nums = gear_neighbor_nums(schematic, (i, j));
                if nbor_nums.len() == 2 {
                    gear_ratios.push(nbor_nums[0] * nbor_nums[1]);
                }
            }
        }
    }
    println!("")
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut gear_ratios = Vec::new();
    let schematic: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().filter(|c| !c.is_ascii_whitespace()).collect())
        .collect();

    print_schematic(&schematic);
    find_adjacent_to_gear(&schematic, &mut gear_ratios);

    let sum = gear_ratios.iter().sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, Some(467835));
    }
}
