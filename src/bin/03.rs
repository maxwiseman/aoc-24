advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<i32> {
    let mul_regex = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();
    let matches = mul_regex.captures_iter(input);

    let mut running_total = 0;

    for item in matches {
        // print!("Matched: {}, {}", &item[1], &item[2]);
        // println!(
        //     " -- {}",
        //     (&item[1].parse::<i32>().unwrap() * &item[2].parse::<i32>().unwrap()).to_string()
        // );
        running_total += &item[1].parse::<i32>().unwrap() * &item[2].parse::<i32>().unwrap()
    }

    Some(running_total)
}

pub fn part_two(input: &str) -> Option<i32> {
    let all_regex = Regex::new(r"mul\((\d*),(\d*)\)|don\'t\(\)|do\(\)").unwrap();
    let mul_regex = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don\'t\(\)").unwrap();
    let matches = all_regex.captures_iter(input);

    let mut running_total = 0;
    let mut should_do = true;

    for item in matches {
        // if let Some(mul_match) = mul_regex.captures(&item[0]) {
        if mul_regex.captures(&item[0]).is_some() {
            // println!("Matched: {}, {}", &mul_match[1], &mul_match[2]);
            if should_do {
                running_total += &item[1].parse::<i32>().unwrap() * &item[2].parse::<i32>().unwrap()
            }
        }
        if do_regex.captures(&item[0]).is_some() {
            // println!("Matched do");
            should_do = true;
        }
        if dont_regex.captures(&item[0]).is_some() {
            // println!("Matched do");
            should_do = false;
        }
    }

    Some(running_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
