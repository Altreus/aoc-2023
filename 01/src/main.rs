use std::env;
use std::fs::read_to_string;
use regex::Regex;

const NUMS : [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const RENUMS : &str = "(one|two|three|four|five|six|seven|eight|nine|\\d)";

fn main() {
    let argv : Vec<_> = env::args().collect();
    let total = read_to_string(&argv[1])
        .unwrap()
        .lines()
        .map(|l| find_calibration_values(l))
        .sum::<i64>();

    println!("{}", total);
}

fn str_to_i64(string: &str) -> i64 {
    if let Some(idx) = NUMS.iter().position(|x| x == &string) {
        return (idx + 1) as i64;
    }

    return string.parse::<i64>().unwrap();
}

fn find_calibration_values(line: &str) -> i64 {
    let re_num1 = Regex::new(RENUMS).unwrap();
    let re_num2 = Regex::new(format!(".*{}", RENUMS).as_str()).unwrap();

    let (digit1, digit2) : (i64, i64);

    let res = re_num1.captures(line).unwrap();
    let ( _, [num1] ) = res.extract();

    digit1 = str_to_i64(num1);

    if let Some(res) = re_num2.captures(line) {
        let (_, [num]) = res.extract();
        digit2 = str_to_i64(num);
    }
    else {
        digit2 = digit1;
    }

    return digit1 * 10 + digit2;
}
