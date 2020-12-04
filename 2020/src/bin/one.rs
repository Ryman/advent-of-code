/*
--- Day 1: Report Repair ---

After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.

The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.

To save your vacation, you need to get all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.

Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.

For example, suppose your expense report contained the following:

1721
979
366
299
675
1456

In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.

Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?
*/

use std::io::Read;
use std::fs::File;

fn main() {
    let mut input = File::open("inputs/one.txt").unwrap();
    let mut s = String::new();

    input.read_to_string(&mut s).unwrap();
    println!("a: {}", solve_a(&s));
    println!("b: {}", solve_b(&s));
}

fn solve_a(input: &str) -> u32 {
    let numbers: Vec<u32> = input.lines().flat_map(|s| s.trim().parse().ok()).collect();

    for (idx, a) in numbers.iter().enumerate() {
        for b in numbers.iter().skip(idx) {
            if a + b == 2020 {
                return a * b;
            }
        }
    }

    unreachable!()
}

/*
--- Part Two ---

The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.

Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.

In your expense report, what is the product of the three entries that sum to 2020?
*/

fn solve_b(input: &str) -> u32 {
    let numbers: Vec<u32> = input.lines().flat_map(|s| s.trim().parse().ok()).collect();

    for (idx, a) in numbers.iter().enumerate() {
        for (idx, b) in numbers.iter().skip(idx).enumerate() {
            for c in numbers.iter().skip(idx) {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }

    unreachable!()
}

#[test]
fn smoke_a() {
    assert_eq!(solve_a("1721
                        979
                        366
                        299
                        675
                        1456"), 514579);
}

#[test]
fn smoke_b() {
    assert_eq!(solve_b("1721
                        979
                        366
                        299
                        675
                        1456"), 241861950);
}
