advent_of_code::solution!(2);

fn check_nums(asc: bool, num: i32, last_num: i32) -> bool {
    return ((asc && (num > last_num)) || (!asc && (num < last_num)))
        && (((num - last_num).abs() >= 1) && (num - last_num).abs() <= 3);
}

fn check_line(nums: Vec<i32>) -> bool {
    let asc: bool = nums[1] > nums[0];
    for i in 1..nums.len() {
        let num = nums[i];
        let last_num = nums[i - 1];

        if check_nums(asc, num, last_num) {
        } else {
            return false;
        }
    }
    return true;
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut safe_lines: i32 = 0;

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| return s.parse::<i32>().expect("Couldn't parse!"))
            .collect();
        let asc: bool = nums[1] > nums[0];

        let mut valid: bool = true;

        for i in 1..nums.len() {
            let num = nums[i];
            let last_num = nums[i - 1];

            if check_nums(asc, num, last_num) {
            } else {
                valid = false;
                break;
            }
        }
        if valid {
            safe_lines += 1;
        }
    }

    return Some(safe_lines);
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut safe_lines: i32 = 0;

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| return s.parse::<i32>().expect("Couldn't parse!"))
            .collect();
        let asc: bool = nums[1] > nums[0];

        let mut valid: bool = true;

        for i in 1..nums.len() {
            let num = nums[i];

            let last_num = nums[i - 1];

            if check_nums(asc, num, last_num) {
            } else {
                valid = false;
                // println!("");
                // print!("Failed line {} at {}; Removed:", line, nums[i]);
                for j in 0..=i {
                    let mut new_list = nums.clone();
                    new_list.remove(j);
                    // print!(" {} ", nums[j]);
                    let line_valid = check_line(new_list);
                    if line_valid {
                        // print!(" Found valid!");
                        valid = true;
                    }
                }
                if valid {
                    break;
                }
            }
        }
        if valid {
            safe_lines += 1;
        }
    }

    return Some(safe_lines);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
