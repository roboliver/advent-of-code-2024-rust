use std::fs;

pub fn read_input(day: u8) -> Result<String, String> {
    if day == 0 || day > 25 {
        return Err(String::from("day must be between 1 and 25"));
    }
    fs::read_to_string(format!("input/day_{:02}.txt", day))
        .map_err(|e| String::from(format!("could not open file: {}", e.to_string())))
}