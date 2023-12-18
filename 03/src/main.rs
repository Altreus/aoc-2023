use std::env;
use std::fs::read_to_string;

#[derive(Debug, Default)]
struct Line {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>
}

#[derive(Debug, Clone)]
struct Number( PlainOrPart, LiteralNumber );

#[derive(Debug, Clone)]
enum PlainOrPart {
    PlainNumber,
    PartNumber,
}

#[derive(Debug, Clone, PartialEq)]
struct LiteralNumber {
    value: usize,
    bounds: [usize; 2]
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Symbol {
    Just(usize),
    MaybeGear(usize, Option<usize>),
    Gear(usize, usize, usize)
}

impl Line {
    fn new(s : String) -> Self {
        let mut line : Line = Default::default();
        let mut num : String = Default::default();

        for (i, c) in s.char_indices() {
            if c.is_ascii_digit() {
                num.push(c);
            }
            else if num.len() > 0 {
                line.numbers.push(
                    Number(PlainOrPart::PlainNumber, LiteralNumber {
                        value: num.parse::<usize>().unwrap(),
                        bounds: [ i - num.len(), i - 1 ],
                    })
                );
                num.clear();
            }

            if c.is_ascii_punctuation() && c != '.' {
                if c == '*' {
                    line.symbols.push(Symbol::MaybeGear(i, None));
                }
                else {
                    line.symbols.push(Symbol::Just(i));
                }
            }
        }

        if num.len() > 0 {
            line.numbers.push(
                Number(PlainOrPart::PlainNumber, LiteralNumber {
                    value: num.parse::<usize>().unwrap(),
                    // It the last num.len() chars of the string.
                    bounds: [ s.len() - num.len(), s.len() - 1 ]
                })
            );
        }

        line
    }

    fn upgrade_partnums(l1 : &Self, l2 : &Self) -> Self {
        // Find symbols in line2 and upgrade numbers from line1 accordingly.
        let mut new_line = Line {
            symbols: l1.symbols.clone(),
            ..Default::default()
        };

        // Remember symbols is just positions of symbols
        for num in &l1.numbers {
            if matches!(num.0, PlainOrPart::PartNumber) {
                new_line.numbers.push(num.clone());
            }
            else {
                let mut push_num = num.clone();

                for symbol in &l2.symbols {
                    if Line::num_adjacent_to_symbol(&num, &symbol) {
                        // this symbol is next to a number. If it might be a
                        // gear, upgrade it. If it's already a gear, nerf it
                        push_num =
                            Number(PlainOrPart::PartNumber, num.1.clone());
                        break;
                    }
                }

                new_line.numbers.push(push_num);
            }
        }

        new_line
    }

    fn upgrade_gears(l1 : &Line, l2: &Line) -> Line {
        let mut new_line = Line {
            numbers: l1.numbers.clone(),
            ..Default::default()
        };

        // Upgrade any MaybeGear in line 1 that is next to a number,
        // but downgrade any Gear that is next to yet another number.
        // Don't compare l1 to itself, because then we end up doing it twice.
        for symbol in &l1.symbols {
            // This symbol could go from MaybeGear(_, None) to
            // MaybeGear(_, Some), to Gear, to Just, all in 2 lines. So, figure
            // out what it is from all available numbers, *then* push it
            let mut push_s = symbol.clone();

            for num in &l2.numbers {
                if Line::num_adjacent_to_symbol(&num, &push_s) {
                    push_s = match push_s {
                        Symbol::Just(_) => push_s,
                        Symbol::MaybeGear(n, None) => Symbol::MaybeGear(n, Some(num.1.value)),
                        Symbol::MaybeGear(n, Some(v)) => Symbol::Gear(n, v, num.1.value),
                        Symbol::Gear(n, _, _) => Symbol::Just(n)
                    };
                }
            }

            new_line.symbols.push(push_s);
        }

        new_line
    }

    fn sum_of_part_numbers(&self) -> usize {
        self.numbers.iter().filter(|n| matches!(n.0, PlainOrPart::PartNumber))
            .map(|n| n.1.value)
            .sum::<usize>()
    }

    fn gear_ratios(&self) -> Vec<usize> {
        let mut v : Vec<usize> = vec![];

        for g in &self.symbols {
            match g {
                Symbol::Gear(_,a,b) => v.push(a * b),
                _ => ()
            }
        }

        return v;
    }

    fn num_adjacent_to_symbol(num : &Number, sym : &Symbol) -> bool {
        let pos = &symbol_pos(sym);
        let [mut start_idx, mut end_idx] = num.1.bounds;
        // Extend the range by 1 to capture diagonal adjacency
        if start_idx > 0 {
            start_idx -= 1;
        }
        end_idx += 1;

        return (start_idx..=end_idx).contains(pos);
    }
}

fn symbol_pos(s : &Symbol) -> usize {
    match s {
        Symbol::Just(n) => *n,
        Symbol::MaybeGear(n, _) => *n,
        Symbol::Gear(n, _, _) => *n
    }
}

fn main() {
    let argv : Vec<_> = env::args().collect();
    let binding = read_to_string(&argv[1])
        .unwrap();

    let mut line_iterator = binding.lines().peekable();
    let mut current_line : Option<Line> = None;
    let mut total_partnums : usize = 0;
    let mut total_gear_ratios : usize = 0;

    // Algo: If line 1 is not set, read it in
    // read line 2 from peeking next line
    // if we got a line 2, upgrade each line from each other
    // sum the part numbers on line 1 into total
    // if we got a line 2, move line 2 into line 1
    // let the loop restart naturally
    while let Some(input_line) = line_iterator.next() {
        // The first l1 doesn't need upgrading from the previous line cos
        // there isn't one. Future l1s will have been upgraded from the line
        // before them in the previous loop.
        let mut l1 = current_line.unwrap_or(Line::new(input_line.to_string()));
        let mut l2 : Line = Default::default();


        // It's this, or put l2 in an Option, which is faff.
        let mut next_line_exists = false;

        if let Some(next_line) = line_iterator.peek() {
            next_line_exists = true;

            // This is not the last line, so we can upgrade l1 from l2 and vice
            // versa. Now l1 is fully upgraded and we can sum its part numbers
            // Also upgrade l1 against itself; no need to do l2 because it'll
            // be l1 in a minute.
            l2 = Line::new(next_line.to_string());
            l1 = Line::upgrade_partnums(&l1, &l2);
            l1 = Line::upgrade_partnums(&l1, &l1);
            l2 = Line::upgrade_partnums(&l2, &l1);

            // We upgrade l1 against itself because we're about to use it and
            // discard it; it's not just inefficient to upgrade l2 against
            // itself, but wrong, because we'll do it again when it becomes l1,
            // and double-count some numbers.
            l1 = Line::upgrade_gears(&l1, &l2);
            l1 = Line::upgrade_gears(&l1, &l1);
            l2 = Line::upgrade_gears(&l2, &l1);
        }

        total_partnums += l1.sum_of_part_numbers();
        total_gear_ratios += l1.gear_ratios().iter().sum::<usize>();

        if next_line_exists {
            current_line = Some(l2);
        }
        else {
            current_line = None;
        }
    }

    println!("Part numbers: {}", total_partnums);
    println!("Gear ratios: {}", total_gear_ratios);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_numbers() {

        // will use char positions for digits.
        // 678 is intentionally at the end of the string
        let s = String::from("012...678");
        let l = Line::new(s);
        assert_eq!(l.numbers.len(), 2, "Found 2 numbers");
        assert_eq!(l.symbols.len(), 0, "Found 0 symbols");

        assert!(matches!(l.numbers[0].0, PlainOrPart::PlainNumber),
            "Numbers are plain by default");
        assert!(matches!(l.numbers[1].0, PlainOrPart::PlainNumber),
            "Numbers are plain by default");
        assert_eq!(l.numbers[0].1.value, 12, "First number is 12 (really 012)");
        assert_eq!(l.numbers[1].1.value, 678, "Second number is 678");
        assert_eq!(l.numbers[0].1.bounds, [ 0, 2 ], "012 bound is from 0 to 2");
        assert_eq!(l.numbers[1].1.bounds, [ 6, 8 ], "678 bound is from 6 to 8");
    }

    #[test]
    fn find_symbols() {
        let s = String::from("...+..!..-");
        let l = Line::new(s);

        assert_eq!(l.numbers.len(), 0, "No numbers");
        assert_eq!(l.symbols.len(), 3, "3 symbols");
        assert_eq!(l.symbols.iter().map(|s| symbol_pos(s)).collect::<Vec<_>>(),
           [3,6,9], "Correct indices");
    }

    #[test]
    fn upgrade_l1_numbers_l2_symbols() {
        let s1 = String::from("1876...68.....143");
        let s2 = String::from("....+.......&...*");
        let l1 = Line::new(s1);
        let l2 = Line::new(s2);

        let testline = Line::upgrade_partnums(&l1, &l2);

        assert!(matches!(testline.numbers[0].0, PlainOrPart::PartNumber),
            "First number now a PartNumber");
        assert!(matches!(testline.numbers[1].0, PlainOrPart::PlainNumber),
            "Second number still a PlainNumber");

        assert_eq!(l1.numbers[0].1, testline.numbers[0].1,
            "Actual number struct is the same");
    }

    #[test]
    fn upgrade_part_number_from_same_line() {
        let s1 = String::from("2233+..44.");
        let line = Line::new(s1);
        let testline = Line::upgrade_partnums(&line, &line);

        assert_eq!(testline.numbers.len(), 2, "Found 2 numbers");
        assert_eq!(testline.symbols.len(), 1, "Found 1 symbol");
        assert!(matches!(testline.numbers[0].0, PlainOrPart::PartNumber),
            "First number now a PartNumber");
        assert!(matches!(testline.numbers[1].0, PlainOrPart::PlainNumber),
            "Second number still a PlainNumber");
    }

    #[test]
    fn upgrade_multiple_symbols_for_same_number() {
        let s1 = String::from("..$1234....658.");
        let s2 = String::from(".....^.^.......");
        let l1 = Line::new(s1);
        let l2 = Line::new(s2);
        let testline = Line::upgrade_partnums(&l1, &l2);

        // The bug here was adding the number several times so this is the
        // most relevant test here.
        assert_eq!(testline.numbers.len(), 2, "Found 2 numbers");
        assert_eq!(testline.symbols.len(), 1, "Found 1 symbol");
        assert!(matches!(testline.numbers[0].0, PlainOrPart::PartNumber),
            "First number now a PartNumber");
        assert!(matches!(testline.numbers[1].0, PlainOrPart::PlainNumber),
            "Second number still a PlainNumber");

    }

    #[test]
    fn upgrade_gear_no_numbers() {
        let s1 = String::from(".+.....*......");
        let s2 = String::from("..1234....658.");
        let l1 = Line::new(s1);
        let l2 = Line::new(s2);

        assert_eq!(l1.symbols.len(), 2, "Found 2 symbols");
        assert!(matches!(l1.symbols[0], Symbol::Just(_)),
            "First symbol is Just a +");
        assert!(matches!(l1.symbols[1], Symbol::MaybeGear(_, None)),
            "Second symbol is a MaybeGear with no number info");

        let testline = Line::upgrade_gears(&l1, &l2);

        assert_eq!(testline.symbols.len(), 2, "Found 2 symbols");
        assert!(matches!(testline.symbols[0], Symbol::Just(_)),
            "First symbol is Just a +");
        assert!(matches!(testline.symbols[1], Symbol::MaybeGear(_, None)),
            "Second symbol is a * but with no number");
    }

    #[test]
    fn upgrade_gear_1_number() {
        let s1 = String::from(".+....*.......");
        let s2 = String::from("..1234....658.");
        let l1 = Line::new(s1);
        let l2 = Line::new(s2);

        assert_eq!(l1.symbols.len(), 2, "Found 2 symbols");
        assert!(matches!(l1.symbols[0], Symbol::Just(_)),
            "First symbol is Just a +");
        assert!(matches!(l1.symbols[1], Symbol::MaybeGear(_, None)),
            "Second symbol is a MaybeGear with no number info");

        let testline = Line::upgrade_gears(
            &Line::upgrade_partnums(&l1, &l2),
            &Line::upgrade_partnums(&l2, &l1));

        assert_eq!(testline.symbols.len(), 2, "Found 2 symbols");
        assert!(matches!(testline.symbols[0], Symbol::Just(_)),
            "First symbol is Just a +");

        let number = match testline.symbols[1] {
            Symbol::Just(_) => {
                assert!(false, "Second symbol is not a *");
                None
            }
            Symbol::MaybeGear(_, None) => {
                assert!(false, "Second symbol should have a number");
                None
            }
            Symbol::MaybeGear(_, Some(n)) => {
                assert!(true, "Second symbol has been given its number");
                Some(n)
            }
            Symbol::Gear(..) => {
                assert!(false, "Second symbol should not be a full Gear");
                None
            }
        };

        assert_eq!(number.unwrap(), 1234, "MaybeGear contains 1234");
    }

    #[test]
    fn upgrade_gear_2_numbers() {
        let s1 = String::from(".+....*.......");
        let s2 = String::from("..1234.658....");
        let l1 = Line::new(s1);
        let l2 = Line::new(s2);

        assert_eq!(l1.symbols.len(), 2, "Found 2 symbols");
        assert!(matches!(l1.symbols[0], Symbol::Just(_)),
            "First symbol is Just a +");
        assert!(matches!(l1.symbols[1], Symbol::MaybeGear(_, None)),
            "Second symbol is a MaybeGear with no number info");

        let testline = Line::upgrade_gears(
            &Line::upgrade_partnums(&l1, &l2),
            &Line::upgrade_partnums(&l2, &l1));

        assert_eq!(testline.symbols.len(), 2, "Found 2 symbols");
        assert!(matches!(testline.symbols[0], Symbol::Just(_)),
            "First symbol is Just a +");

        let (number1, number2) = match testline.symbols[1] {
            Symbol::Just(_) => {
                assert!(false, "Second symbol is not a *");
                (None, None)
            }
            Symbol::MaybeGear(_, None) => {
                assert!(false, "Second symbol should be a full Gear");
                (None, None)
            }
            Symbol::MaybeGear(..) => {
                assert!(false, "Second symbol should be a full Gear");
                (None, None)
            }
            Symbol::Gear(_,a,b) => {
                assert!(true, "Second symbol has been given two numbers");
                (Some(a), Some(b))
            }
        };

        assert_eq!(number1.unwrap(), 1234, "Gear contains 1234");
        assert_eq!(number2.unwrap(), 658,  "Gear contains 658");
    }

    #[test]
    fn upgrade_gear_3_numbers() {
        let s1 = String::from(".+....*94.....");
        let s2 = String::from("..1234.658....");
        let l1 = Line::new(s1);
        let l2 = Line::new(s2);

        assert_eq!(l1.symbols.len(), 2, "Found 2 symbols");
        assert!(matches!(l1.symbols[0], Symbol::Just(_)),
            "First symbol is Just a +");
        assert!(matches!(l1.symbols[1], Symbol::MaybeGear(_, None)),
            "Second symbol is a MaybeGear with no number info");

        let mut testline = Line::upgrade_gears(
            &Line::upgrade_partnums(&l1, &l2),
            &Line::upgrade_partnums(&l2, &l1));
        testline = Line::upgrade_gears(&testline, &testline);

        assert_eq!(testline.symbols.len(), 2, "Found 2 symbols");
        assert!(matches!(testline.symbols[0], Symbol::Just(_)),
            "First symbol is Just a +");

        match testline.symbols[1] {
            Symbol::Just(_) => {
                assert!(true, "Second symbol was downgraded to Just");
            }
            Symbol::MaybeGear(_, None) => {
                assert!(false, "Second symbol is not a gear after all");
            }
            Symbol::MaybeGear(..) => {
                assert!(false, "Second symbol is not a gear after all");
            }
            Symbol::Gear(..) => {
                assert!(false, "Second symbol is not a gear after all");
            }
        };
    }

    #[test]
    fn gear_ratios() {
        let s1 = String::from(".+....*94...-..2*...*");
        let s2 = String::from("..1234.658.....123..1");
        let l1 = Line::new(s1);
        let l2 = Line::new(s2);

        let mut testline = Line::upgrade_gears(
            &Line::upgrade_partnums(&l1, &l2),
            &Line::upgrade_partnums(&l2, &l1));
        testline = Line::upgrade_gears(&testline, &testline);

        assert_eq!(testline.gear_ratios(), [2*123]);
    }

    #[test]
    fn sum_of_part_numbers() {
        let s1 = String::from("1876...68...");
        let s2 = String::from("......*..!.-");
        let l1 = Line::new(s1);
        let l2 = Line::new(s2);

        let testline = Line::upgrade_partnums(&l1, &l2);

        assert_eq!(Line::sum_of_part_numbers(&testline), 68);
    }
}
