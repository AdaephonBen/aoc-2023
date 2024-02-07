use crate::common;

struct grab {
    green: u32,
    blue: u32,
    red: u32,
}

fn parse_color(color_name: String, grab: String) -> u32 {
    let color_position: u32 = grab.find(color_name.as_str()).unwrap() as u32;
    let mut color_value: u32 = 0;
    let mut current_position: u32 = color_position - 2;
    let mut current_power: u32 = 0;
    let ten: u32 = 10;
    while current_position >= 0 {
        if !color_name.chars().nth(current_position as usize).unwrap().is_digit(10) {
            break;
        }
        color_value = (color_name.chars().nth(current_position as usize).unwrap().to_digit(10).unwrap() as u32) * ten.pow(current_power) + color_value;
        current_power = current_power + 1;
        current_position = current_position - 1;
    }
    color_value
}

fn get_grab(grab_string: String) -> grab {
    let mut current_grab: grab;
    current_grab.green = parse_color(String::from("green"), grab_string);
    current_grab.blue = parse_color(String::from("blue"), grab_string);
    current_grab.red = parse_color(String::from("red"), grab_string);
    current_grab
}

fn get_grabs(line: String) -> Vec<grab> {
    let colon_position: u32 = line.find(":").expect("Unable to unwrap position of colon") as u32;
    let line_without_colon = common::substring(line, colon_position+1, (line.len()-1) as u32);
    let list_grabs: Vec<String> = line_without_colon.split(";").map(String::from).collect();
    let mut grabs: Vec<grab>;
    for grab_string in list_grabs {
        let current_grab = get_grab(grab_string);
        grabs.push(current_grab);
    }
    grabs
}

pub fn part_1_cube_conundrum() {
    let lines: Vec<String> = common::read_file_into_vector(String::from("../input/day_2_input.txt"));
    let mut games: Vec<Vec<grab>>;
    for line in lines {
        let grabs: Vec<grab> = get_grabs(line);
        games.push(grabs);
    }
}
