use std::env;
use std::collections::VecDeque;
use std::fs::read_to_string;

#[derive(Debug)]
struct Card {
    winning: Vec<usize>,
    have: Vec<usize>
}

impl Card {
    fn from(s : &str) -> Self {
        let (_, nums) = s.split_once(':').unwrap();
        let (wnums, hnums) = nums.split_once('|').unwrap();
        let winning = wnums.split(' ')
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();
        let have = hnums.split(' ')
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();

        Card {
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

    let mut pt1_score : usize = 0;
    let mut pt2_score : usize = 0;
    let mut multipliers : VecDeque<usize> = VecDeque::new();

    for input_line in binding.lines() {
        let c = Card::from(input_line);
        let num_wins = c.have_wins().len();
        let current_mult = multipliers.pop_front().unwrap_or(1);
        pt2_score += current_mult;


        if num_wins != 0 {
            if multipliers.len() < num_wins {
                multipliers.resize(num_wins, 1);
            }

            // Part 1: for every win on the card, this card is worth double;
            // that's just 2^wins.
            pt1_score += usize::pow(2, (c.have_wins().len() - 1).try_into().unwrap());

            // Part 2: for every win on the card, you get 1 more of each of the
            // next N cards. So if you already have X copies of this card, then
            // all X of them will have N wins. So the next N cards will get X
            // more copies. The total is how many cards you end up with.
            for n in 0..num_wins {
                multipliers[n] += current_mult;
            }
        }
    }

    println!("Part 1 score: {}", pt1_score);
    println!("Part 2 score: {}", pt2_score);
}
