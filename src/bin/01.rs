advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    let mut total_diff: i32 = 0;

    for line in input.lines() {
        let line_numbers: Vec<&str> = line.split_whitespace().collect();
        left_list.push(line_numbers[0].parse().expect("Invalid number!"));
        right_list.push(line_numbers[1].parse().expect("Invalid number!"));
    }

    left_list.sort();
    right_list.sort();

    if left_list.len() == right_list.len() {
        println!("Comparing!");
        for n in 0..left_list.len() {
            let diff = (left_list[n] - right_list[n]).abs();
            total_diff += diff;
        }
    } else {
        println!("Cannot compare! Lists are not the same length!");
    }

    println!("Answer: {}", total_diff);
    return Some(total_diff);
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    let mut total_similarity: i32 = 0;

    for line in input.lines() {
        let line_numbers: Vec<&str> = line.split_whitespace().collect();
        left_list.push(line_numbers[0].parse().expect("Invalid number!"));
        right_list.push(line_numbers[1].parse().expect("Invalid number!"));
    }

    left_list.sort();
    right_list.sort();

    if left_list.len() == right_list.len() {
        println!("Calculating!");
        for left_num in left_list {
            let mut matches: i32 = 0;

            for right_num in &right_list {
                if right_num > &left_num {
                    break;
                } else {
                    if right_num == &left_num {
                        matches += 1;
                    }
                }
            }

            let similarity = matches * left_num;
            total_similarity += similarity;
        }
    } else {
        println!("Cannot calculate! Lists are not the same length!");
    }

    println!("Similarity: {}", total_similarity);
    return Some(total_similarity);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
