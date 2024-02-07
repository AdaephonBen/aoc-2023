use crate::common;

fn part_1_get_calibration_value(line: String) -> u32 {
    let mut is_first_digit_found: bool = false;
    let mut first_digit: u32 = 0;
    let mut last_digit: u32 = 0;
    for character in line.chars() {
        let digit = character.to_digit(10);
        let digit_value: u32;
        match digit {
            Some(inner) => digit_value = inner,
            None => continue,
        }
        if is_first_digit_found {
            last_digit = digit_value;
        } else {
            first_digit = digit_value;
            last_digit = digit_value;
            is_first_digit_found = true;
        }
    }
    let calib_value: u32 = first_digit * 10 + last_digit;
    return calib_value;
}

fn part_2_get_calibration_value(line: String) -> u32 {
    let mut is_first_digit_found: bool = false;
    let mut first_digit: u32 = 0;
    let mut first_digit_position: u32 = 0;
    let mut last_digit_position: u32 = 0;
    let mut last_digit: u32 = 0;
    for (index, character) in line.chars().enumerate() {
        let digit = character.to_digit(10);
        let digit_value: u32;
        match digit {
            Some(inner) => digit_value = inner,
            None => continue,
        }
        if is_first_digit_found {
            last_digit = digit_value;
            last_digit_position = index as u32;
        } else {
            first_digit = digit_value;
            last_digit = digit_value;
            last_digit_position = index as u32;
            first_digit_position = index as u32;
            is_first_digit_found = true;
        }
    }
    let mut lowest_first_digit_position: u32 = first_digit_position;
    let mut highest_last_digit_position: u32 = last_digit_position;
    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for (index_digit, alpha_digit) in digits.iter().enumerate() {
        let first_digit_position_alpha_option = line.find(alpha_digit);
        let last_digit_position_alpha_option = line.rfind(alpha_digit);
        let first_digit_position_alpha: u32;
        let last_digit_position_alpha: u32;
        match first_digit_position_alpha_option {
            Some(digit) => first_digit_position_alpha = digit as u32,
            None => continue,
        }
        match last_digit_position_alpha_option {
            Some(digit) => last_digit_position_alpha = digit as u32,
            None => continue,
        }
        if first_digit_position_alpha < lowest_first_digit_position {
            lowest_first_digit_position = first_digit_position_alpha;
            first_digit = (index_digit + 1) as u32;
        }
        if last_digit_position_alpha > highest_last_digit_position {
            highest_last_digit_position = last_digit_position_alpha;
            last_digit = (index_digit + 1) as u32;
        }
    }
    let calib_value: u32 = first_digit * 10 + last_digit;
    return calib_value;
}

pub fn part_1_trebuchet() {
    let lines: Vec<String> = common::read_file_into_vector(String::from("../input/day_1_input.txt"));
    let mut sum: u32 = 0;
    for line in lines {
        sum += part_1_get_calibration_value(line);
    } 
    println!("The total is {}", sum);
}

pub fn part_2_trebuchet() {
    let lines: Vec<String> = common::read_file_into_vector(String::from("../input/day_1_input.txt"));
    let mut sum: u32 = 0;
    for line in lines {
        sum += part_2_get_calibration_value(line);
    } 
    println!("The total is {}", sum);
}
