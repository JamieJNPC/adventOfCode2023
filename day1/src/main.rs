use std::fs;

use regex::Regex;

fn parse_digits(line: String) -> String {
    let mut new_line = line;
    let patterns = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for (i,pattern) in patterns.iter().enumerate() {
        let re = Regex::new(pattern).unwrap();
        let owned_pattern = pattern.to_owned().to_owned();
        new_line = re.replace_all(&new_line, owned_pattern.clone() + &(i+1).to_string() + &owned_pattern).to_string();
    }
    return new_line;
}

fn main() {
    let contents: String = fs::read_to_string("./input.txt").unwrap();
    let mut vec: Vec<String> = Vec::new();
    let re = Regex::new(r"[A-Za-z]").unwrap();
    for line in contents.split('\n') {
        let number_string = re.replace_all(&parse_digits(line.to_string()), "").to_string();
        let mut number_chars = number_string.chars();
        let mut string: String = String::new();
        let first_digit: char = number_chars.next().unwrap();
        string.push(first_digit);
        match number_chars.last() {
            Some(x) => string.push(x),
            None => string.push(first_digit)
        }
        vec.push(string);
    }
    let mut count = 0;
    for string in vec {
        count += string.parse::<i32>().unwrap();
    }
    println!("{}", count);
}