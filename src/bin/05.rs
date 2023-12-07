advent_of_code::solution!(5);

pub fn transform(input: u64, ranges: &Vec<Vec<(u64, u64, u64)>>) -> u64 {
    let mut input = input;
    let mut output: Option<u64> = None;
    // print!("{}", input);
    for map in ranges {
        for (dst, src, len) in map {
            if input >= *src && input < *src + *len {
                output = Some(dst + (input - src));
            }
        }
        if output == None {
            output = Some(input);
        }
        // print!(" -> {}", output.unwrap());
        input = output.unwrap();
    }
    // println!("");
    return output.unwrap();
}

pub fn list_to_vec_num(list: &str) -> Vec<u64> {
    let vec_num: Vec<u64> = list
        .split(" ")
        .filter(|c| !c.trim().is_empty())
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .collect();
    vec_num
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut it = input.lines();
    let seeds = it.next().unwrap().split(":").skip(1).next().unwrap();
    let seeds = list_to_vec_num(seeds);
    // caprintln!("Seeds: {:?}", seeds);

    let mut ranges: Vec<Vec<(u64, u64, u64)>> = Vec::new();

    for block in input
        .lines()
        .map(|x| x.trim().to_string() + "\n")
        .collect::<String>()
        .split("\n\n")
        .skip(1)
    {
        let mut vec_triples: Vec<(u64, u64, u64)> = Vec::new();
        for line in block.trim().lines().skip(1) {
            // println!("{}", line);
            let vec_num = list_to_vec_num(line);
            // println!("{:?}", vec_num);
            vec_triples.push((vec_num[0], vec_num[1], vec_num[2]));
        }
        ranges.push(vec_triples);
    }

    let mut lowest_location = u64::MAX;
    for seed in seeds {
        let location = transform(seed, &ranges);
        lowest_location = lowest_location.min(location);
    }

    Some(lowest_location)
}

pub fn cut_range(preserve_range: Vec<(u64, u64)>, trans_range: (u64, u64)) -> Vec<(u64, u64)> {
    let t_start = trans_range.0;
    let t_end = trans_range.1;

    let mut new_preserve_range: Vec<(u64, u64)> = Vec::new();

    for (p_start, p_len) in preserve_range {
        let p_end = p_start + p_len;
        if p_end < t_start || t_end < p_start {
            // Nothing cut
            new_preserve_range.push((p_start, p_len));
        } else if t_start > p_start && t_end < t_start {
            // Cut inside of preserve range
            new_preserve_range.push((p_start, t_start - p_start));
            new_preserve_range.push((t_end, p_end - t_end));
        } else if t_start < p_start && p_start < t_end && t_end < p_end {
            // Cut left
            new_preserve_range.push((t_end, p_end - t_end));
        } else {
            // Cut right
            new_preserve_range.push((p_start, t_start - p_start));
        }
    }
    new_preserve_range
}

pub fn transform_ranges(input_range: (u64, u64), ranges: &Vec<Vec<(u64, u64, u64)>>) -> Vec<u64> {
    let mut inputs: Vec<(u64, u64)> = Vec::new();
    let mut outputs: Vec<(u64, u64)> = Vec::new();
    inputs.push(input_range);
    for map in ranges {
        print!("NEXT STEP");
        for (input_start, input_len) in &inputs {
            println!("Input ({input_start}, {input_len})");
        }
        for (input_start, input_len) in inputs {
            if input_len == 0 {
                continue
            }
            println!("({}, {}) --v", input_start, input_len);
            let mut preserved_input: Vec<(u64, u64)> = Vec::new();
            preserved_input.push((input_start, input_len));
            for &(dst_start, src_start, range_len) in map {
                println!(
                    "IN RANGE ({}, {}) to dst {}",
                    src_start, range_len, dst_start
                );
                // We only need to change the input covered by range.
                let trans_start = src_start.max(input_start);
                let trans_end = (src_start + range_len).min(input_start + input_len);
                if trans_start <= trans_end {
                    let post_trans_start = dst_start + trans_start - src_start;
                    let post_trans_len = trans_end - trans_start;
                    outputs.push((post_trans_start, post_trans_len));
                    println!("   ({}, {}) (inrange)", post_trans_start, post_trans_len);
                    // Save what part of input was transformed
                    preserved_input = cut_range(preserved_input, (trans_start, trans_end));
                }
            }
            outputs.append(&mut preserved_input);
        }
        for (input_start, input_len) in &outputs {
            println!("Output ({input_start}, {input_len})");
        }
        inputs = outputs.clone();
        outputs.clear();
    }

    return inputs.iter().map(|(start, len)| *start).collect();
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut it = input.lines();
    let seeds = it.next().unwrap().split(":").skip(1).next().unwrap();
    let seeds = list_to_vec_num(seeds);
    let seeds_sizes: Vec<&[u64]> = seeds.chunks(2).collect();
    // caprintln!("Seeds: {:?}", seeds);

    let mut ranges: Vec<Vec<(u64, u64, u64)>> = Vec::new();

    for block in input
        .lines()
        .map(|x| x.trim().to_string() + "\n")
        .collect::<String>()
        .split("\n\n")
        .skip(1)
    {
        let mut vec_triples: Vec<(u64, u64, u64)> = Vec::new();
        for line in block.trim().lines().skip(1) {
            // println!("{}", line);
            let vec_num = list_to_vec_num(line);
            // println!("{:?}", vec_num);
            vec_triples.push((vec_num[0], vec_num[1], vec_num[2]));
        }
        ranges.push(vec_triples);
    }

    let mut lowest_location = u64::MAX;
    for seed_size in seeds_sizes {
        let s = (seed_size[0], seed_size[1]);
        let location_ranges = transform_ranges(s, &ranges);
        for loc in location_ranges {
            lowest_location = lowest_location.min(loc);
        }
        println!("Lowest location: {}", lowest_location);
    }

    Some(lowest_location)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, Some(46));
    }
}
