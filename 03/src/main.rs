use std::env;
use std::fs::read_to_string;

#[derive(Debug, Default)]
struct Line {
    numbers: Vec<Number>,
    symbols: Vec<usize>
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
                line.symbols.push(i);
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

    fn upgrade(l1 : &Self, l2 : &Self) -> Self {
        // Find symbols in line2 and upgrade numbers from line1 accordingly.
        let mut new_line = Line {
            symbols: l1.symbols.clone(),
            .. Default::default()
        };

        if l2.symbols.len() == 0 {
            new_line.numbers = l1.numbers.clone();
            return new_line;
        }

        // Remember symbols is just positions of symbols
        for num in &l1.numbers {
            if matches!(num.0, PlainOrPart::PartNumber) {
                new_line.numbers.push(num.clone());
            }
            else {

                let mut push_num = num.clone();
                for symbol in &l2.symbols {
                    let [mut start_idx, mut end_idx] = num.1.bounds;
                    // Extend the range by 1 to capture diagonal adjacency
                    if start_idx > 0 {
                        start_idx -= 1;
                    }
                    end_idx += 1;

                    if (start_idx..=end_idx).contains(&symbol) {
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

    fn sum_of_part_numbers(l : &Self) -> usize {
        l.numbers.iter().filter(|n| matches!(n.0, PlainOrPart::PartNumber))
            .map(|n| n.1.value)
            .sum::<usize>()
    }
}

fn main() {
    let argv : Vec<_> = env::args().collect();
    let binding = read_to_string(&argv[1])
        .unwrap();

    let mut line_iterator = binding.lines().peekable();
    let mut current_line : Option<Line> = None;
    let mut total : usize = 0;

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
            // Also upgrade the lines with themselves - this will catch symbols
            // next to numbers on the same line
            l2 = Line::new(next_line.to_string());
            l1 = Line::upgrade(&l1, &l2);
            l1 = Line::upgrade(&l1, &l1);
            l2 = Line::upgrade(&l2, &l1);
            l2 = Line::upgrade(&l2, &l2);
        }

        total += Line::sum_of_part_numbers(&l1);

        if next_line_exists {
            current_line = Some(l2);
        }
        else {
            current_line = None;
        }
    }

    println!("{}", total);
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
        assert_eq!(l.symbols, [3,6,9], "Correct indices");
    }

    #[test]
    fn upgrade() {
        let s1 = String::from("1876...68.");
        let s2 = String::from("....+.....");
        let l1 = Line::new(s1);
        let l2 = Line::new(s2);

        let mut testline = Line::upgrade(&l1, &l2);

        assert!(matches!(testline.numbers[0].0, PlainOrPart::PartNumber),
            "First number now a PartNumber");
        assert!(matches!(testline.numbers[1].0, PlainOrPart::PlainNumber),
            "Second number still a PlainNumber");

        assert_eq!(l1.numbers[0].1, testline.numbers[0].1,
            "Actual number struct is the same");

        let s3 = String::from("2233+..44.");
        let s4 = String::from("......*.+.");
        let l3 = Line::new(s3);
        let l4 = Line::new(s4);
        testline = Line::upgrade(&l3, &l4);
        testline = Line::upgrade(&testline, &testline);

        assert_eq!(testline.numbers.len(), 2, "Found 2 numbers");
        assert_eq!(testline.symbols.len(), 1, "Found 1 symbol");
        assert!(matches!(testline.numbers[0].0, PlainOrPart::PartNumber),
            "First number now a PartNumber");
        assert!(matches!(testline.numbers[1].0, PlainOrPart::PartNumber),
            "Second number still a PlainNumber");

    }

    #[test]
    fn sum_of_part_numbers() {
        let s1 = String::from("1876...68...");
        let s2 = String::from("......*..!.-");
        let l1 = Line::new(s1);
        let l2 = Line::new(s2);

        let testline = Line::upgrade(&l1, &l2);

        assert_eq!(Line::sum_of_part_numbers(&testline), 68);
    }
}
