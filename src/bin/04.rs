advent_of_code::solution!(4);

fn check_diag(input: &str, match_string: &str, y: i32, x: i32, rev: bool) -> bool {
    let mut is_valid = true;
    let lines: Vec<&str> = input.lines().collect();

    for i in 0..match_string.len() {
        if y as usize + i as usize >= lines.len() {
            is_valid = false;
            break;
        }

        let check_line = lines.get(y as usize + i as usize).unwrap();
        let new_x = if rev == true {
            x as i32 - i as i32
        } else {
            x as i32 + i as i32
        };

        if new_x >= check_line.chars().collect::<Vec<char>>().len() as i32 || new_x < 0 {
            is_valid = false;
            break;
        }
        if check_line
            .chars()
            .collect::<Vec<char>>()
            .get(new_x as usize)
            != match_string.chars().collect::<Vec<char>>().get(i)
        {
            is_valid = false;
            break;
        }
    }
    is_valid
}

fn check_vert(input: &str, match_string: &str, y: i32, x: i32) -> bool {
    let mut is_valid = true;
    let lines: Vec<&str> = input.lines().collect();

    for i in 0..match_string.len() {
        if y as usize + i as usize >= lines.len() {
            is_valid = false;
            break;
        }

        let check_line = lines.get(y as usize + i as usize).unwrap();
        if check_line.chars().collect::<Vec<char>>().get(x as usize)
            != match_string.chars().collect::<Vec<char>>().get(i)
        {
            is_valid = false;
            break;
        }
    }
    is_valid
}

pub fn part_one(input: &str) -> Option<i32> {
    let match_string = "XMAS";
    let reversed_match_string: String = match_string.chars().rev().collect();

    let mut running_total = 0;

    let lines: Vec<&str> = input.lines().collect();
    for y in 0..lines.len() as i32 {
        let line = lines[y as usize];
        // println!("{}", line);
        running_total += line.matches(match_string).count() as i32;
        running_total += line.matches(&reversed_match_string).count() as i32;

        for x in 0..line.chars().count() {
            let character_list: Vec<char> = line.chars().collect();
            let character = character_list[x];
            if character.to_string() == "X" || character.to_string() == "S" {
                // print!("Found thing");
                if check_diag(input, match_string, y, x as i32, false) {
                    running_total += 1
                }
                if check_diag(input, match_string, y, x as i32, true) {
                    running_total += 1
                }
                if check_diag(input, reversed_match_string.as_str(), y, x as i32, false) {
                    running_total += 1
                }
                if check_diag(input, reversed_match_string.as_str(), y, x as i32, true) {
                    running_total += 1
                }
                if check_vert(input, match_string, y, x as i32) {
                    running_total += 1
                }
                if check_vert(input, reversed_match_string.as_str(), y, x as i32) {
                    running_total += 1
                }
            }
        }
    }
    Some(running_total)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut running_total = 0;

    let lines: Vec<&str> = input.lines().collect();

    fn check(input: &str, y: i32, x: i32) -> bool {
        let lines: Vec<&str> = input.lines().collect();
        if y + 1 < lines.len() as i32 && y - 1 >= 0 && x - 1 >= 0 {
            let top_line = lines[y as usize - 1];
            let bottom_line = lines[y as usize - 1];
            if x + 1 >= top_line.len() as i32 || x + 1 > bottom_line.len() as i32 {
                return false;
            }

            let top_left_char = lines[y as usize - 1 as usize]
                .chars()
                .collect::<Vec<char>>()[x as usize - 1 as usize]
                .to_string();
            let top_right_char = lines[y as usize - 1 as usize]
                .chars()
                .collect::<Vec<char>>()[x as usize + 1 as usize]
                .to_string();

            let bottom_right_char = lines[y as usize + 1 as usize]
                .chars()
                .collect::<Vec<char>>()[x as usize + 1 as usize]
                .to_string();
            let bottom_left_char = lines[y as usize + 1 as usize]
                .chars()
                .collect::<Vec<char>>()[x as usize - 1 as usize]
                .to_string();

            if (top_left_char == "M" || top_left_char == "S")
                && (bottom_right_char == "M" || bottom_right_char == "S")
                && top_left_char != bottom_right_char
                && (top_right_char == "M" || top_right_char == "S")
                && (bottom_left_char == "M" || bottom_left_char == "S")
                && top_right_char != bottom_left_char
            {
                return true;
            } else {
                return false;
            }
        } else {
            false
        }
    }

    for y in 0..lines.len() as i32 {
        let line = lines[y as usize];
        // println!("{}", line);

        for x in 0..line.chars().count() {
            let character_list: Vec<char> = line.chars().collect();
            let character = character_list[x];
            if character.to_string() == "A" {
                if check(input, y, x as i32) {
                    running_total += 1;
                }
            }
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
