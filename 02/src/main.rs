use std::env;
use std::fs::read_to_string;
use regex::Regex;

#[derive(Debug)]
struct Hand {
    red: Option<i64>,
    green: Option<i64>,
    blue: Option<i64>
}

#[derive(Debug)]
struct Game {
    hands: Vec<Hand>,
    id: i64
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
        .map(|l| str_to_game(l))
        .filter(|g| is_game_possible(g, &proto_hand))
        .map(|g| g.id)
        .sum::<i64>();

    let powersum = 0;

    println!("Part 1: {}", total);

    println!("Part 2: {}", powersum);
}

fn is_game_possible(game: &Game, proto: &Hand) -> bool {
    // Is any hand bigger than the prototype hand? That means it's not possible
    return !game.hands.iter()
        .any(|h| is_any_field_bigger(&h, proto));
}

fn str_to_game(string: &str) -> Game {
    // it doesn't really matter that the first split has the Game N: part
    let hands = string.split(";").map(|h| str_to_hand(h));
    let id = get_game_id(string);

    return Game {
        hands: hands.collect::<Vec<Hand>>(),
        id: id
    }
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

//fn max_of_each_colour(hands: &Vec<Hand>) -> Hand {
//}
