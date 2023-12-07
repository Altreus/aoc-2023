use std::env;
use std::fs::read_to_string;
use regex::Regex;

#[derive(Debug)]
struct Hand {
    rgb: [ Option<i64>; 3 ]
}

#[derive(Debug)]
struct Game {
    hands: Vec<Hand>,
    id: i64
}

fn main() {
    let argv : Vec<_> = env::args().collect();
    let proto_hand = Hand {
        rgb: [ Some(12), Some(13), Some(14) ]
    };

    // The compiler told me to make this binding variable
    let binding = read_to_string(&argv[1])
        .unwrap();
    let games = binding
        .lines()
        .map(|l| str_to_game(l));

    let total = games.clone()
        .filter(|g| is_game_possible(g, &proto_hand))
        .map(|g| g.id)
        .sum::<i64>();

    let powersum = games.map(|g| max_of_each_colour(&g.hands))
        .map(|h| power(&h))
        .sum::<i64>();

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
        rgb: [
            one_int_from_str(string, re_red),
            one_int_from_str(string, re_green),
            one_int_from_str(string, re_blue)
        ]
    }
}

fn is_any_field_bigger(lhs: &Hand, rhs: &Hand) -> bool {
    for n in 0..=2 {
        if lhs.rgb[n].unwrap_or(0) > rhs.rgb[n].unwrap_or(0) {
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

fn max_of_each_colour(hands: &Vec<Hand>) -> Hand {
    let mut hand = Hand {
        rgb: [None, None, None]
    };

    for h in hands {
        for n in 0..=2 {
            hand.rgb[n] = match (hand.rgb[n], h.rgb[n]) {
                (x,        None)     => x,
                (None,     Some(x))  => Some(x),
                (Some(x1), Some(x2)) => Some(std::cmp::max(x1,x2))
            };
        }
    }

    return hand;
}

fn power(hand: &Hand) -> i64 {
    return hand.rgb.iter().map(|x| x.unwrap_or(1)).fold(1, |acc,x| acc * x);
}
