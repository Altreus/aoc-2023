use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let mut total : i64 = 0;

    let re_1d = Regex::new(r"(\d)").unwrap();
    let re_2d = Regex::new(r"(\d).*(\d)").unwrap();
    for line in read_to_string("input.txt").unwrap().lines() {
        let cap = re_2d.captures(line);
        match cap {
            None => {
                let cap2 = re_1d.captures(line);
                match cap2 {
                    None => { continue; }
                    Some(res) => {
                        let (_, [num]) = res.extract();
                        let cal = num.to_owned() + num;
                        total += cal.parse::<i64>().unwrap();
                    }
                }
            }
            Some(res) => {
                let ( _, [num1, num2] ) = res.extract();


                let cal = num1.to_owned() + num2;
                total += cal.parse::<i64>().unwrap();
            }
        }
    }

    println!("{}", total);
}
