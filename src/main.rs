use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let total = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| find_calibration_values(l))
        .sum::<i64>();

    println!("{}", total);
}

fn find_calibration_values(line: &str) -> i64 {
    let re_1d = Regex::new(r"(\d)").unwrap();
    let re_2d = Regex::new(r"(\d).*(\d)").unwrap();

    if let Some(res) = re_2d.captures(line) {
        let ( _, [num1, num2] ) = res.extract();


        let cal = num1.to_owned() + num2;
        return cal.parse::<i64>().unwrap();
    }

    if let Some(res) = re_1d.captures(line) {
        let (_, [num]) = res.extract();
        let cal = num.to_owned() + num;
        return cal.parse::<i64>().unwrap();
    }

    return 0;
}
