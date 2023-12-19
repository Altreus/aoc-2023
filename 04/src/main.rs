use std::env;
use std::fs::read_to_string;
use regex::Regex;

#[derive(Debug)]
struct Card {
    num: usize,
    winning: Vec<usize>,
    have: Vec<usize>
}

impl Card {
    fn from(s : &str) -> Self {
        let (cnum, nums) = s.split_once(':').unwrap();
        let num = one_int_from_str(cnum, Regex::new(r"Card\s+(\d+)").unwrap()).unwrap();
        let (wnums, hnums) = nums.split_once('|').unwrap();
        let winning = wnums.split(' ')
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();
        let have = hnums.split(' ')
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();

        Card {
            num: num,
            winning: winning,
            have: have
        }
    }

    fn have_wins (&self) -> Vec<usize> {
        self.have.iter().filter_map(|n| if self.winning.contains(n) {Some(*n)} else { None }) .collect()
    }
}

fn main() {
    let argv : Vec<_> = env::args().collect();
    let binding = read_to_string(&argv[1])
        .unwrap();

    let mut total_score : usize = 0;

    for input_line in binding.lines() {
        let c = Card::from(input_line);
        let have_wins = c.have_wins();
        if have_wins.len() != 0 {
            total_score += usize::pow(2, (c.have_wins().len() - 1).try_into().unwrap());
        }
    }

    println!("{}", total_score);
}

fn one_int_from_str(string: &str, re: Regex) -> Option<usize> {
    if let Some(res) = re.captures(string) {
        let (_, [num]) = res.extract();
        return Some(num.parse::<usize>().unwrap());
    }
    return None;

}
