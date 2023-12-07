use std::env;
use std::fs::read_to_string;
use regex::Regex;

#[derive(Debug)]
struct Hand {
    red: Option<i64>,
    green: Option<i64>,
    blue: Option<i64>
}

fn main() {
    let argv : Vec<_> = env::args().collect();
    let proto_hand = Hand {
        red: Some(12),
        green: Some(13),
        blue: Some(14)
    };

    let total = read_to_string(&argv[1])
        .unwrap()
        .lines()
        .filter(|l| is_game_possible(l, &proto_hand))
        .map(|l| get_game_id(l))
        .sum::<i64>();

    println!("{}", total);
}

fn is_game_possible(line: &str, proto: &Hand) -> bool {
    let hands = line.split(";");

    // Is any hand bigger than the prototype hand? That means it's not possible
    return !hands.map(|h| str_to_hand(h)).any(|h| is_any_field_bigger(&h, proto));
}

fn str_to_hand(string: &str) -> Hand {
    let re_red = Regex::new(r"(\d+) red").unwrap();
    let re_green = Regex::new(r"(\d+) green").unwrap();
    let re_blue = Regex::new(r"(\d+) blue").unwrap();

    Hand {
        red: one_int_from_str(string, re_red),
        green: one_int_from_str(string, re_green),
        blue: one_int_from_str(string, re_blue)
    }
}

fn is_any_field_bigger(lhs: &Hand, rhs: &Hand) -> bool {
    if lhs.red != None && rhs.red != None {
        if lhs.red.unwrap() > rhs.red.unwrap() {
            return true;
        }
    }
    if lhs.green != None && rhs.green != None {
        if lhs.green.unwrap() > rhs.green.unwrap() {
            return true;
        }
    }
    if lhs.blue != None && rhs.blue != None {
        if lhs.blue.unwrap() > rhs.blue.unwrap() {
            return true;
        }
    }

    return false;
}

fn one_int_from_str(string: &str, re: Regex) -> Option<i64> {
    if let Some(res) = re.captures(string) {
        let (_, [num]) = res.extract();
        return Some(num.parse::<i64>().unwrap());
    }
    return None;

}

fn get_game_id(line: &str) -> i64 {
    let re = Regex::new(r"Game (\d+)").unwrap();
    return one_int_from_str(line, re).unwrap();

}
