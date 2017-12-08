/*
--- Day 8: I Heard You Like Registers ---

You receive a signal directly from the CPU. Because of your recent assistance with jump instructions, it would like you to compute the result of a series of unusual register instructions.

Each instruction consists of several parts: the register to modify, whether to increase or decrease that register's value, the amount by which to increase or decrease it, and a condition. If the condition fails, skip the instruction without modifying the register. The registers all start at 0. The instructions look like this:

b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10

These instructions would be processed as follows:

    Because a starts at 0, it is not greater than 1, and so b is not modified.
    a is increased by 1 (to 1) because b is less than 5 (it is 0).
    c is decreased by -10 (to 10) because a is now greater than or equal to 1 (it is 1).
    c is increased by -20 (to -10) because c is equal to 10.

After this process, the largest value in any register is 1.

You might also encounter <= (less than or equal to) or != (not equal to). However, the CPU doesn't have the bandwidth to tell you what all the registers are named, and leaves that to you to determine.

What is the largest value in any register after completing the instructions in your puzzle input?
*/

use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use std::cmp;

fn main() {
    let mut input = File::open("inputs/eight.txt").unwrap();
    let mut s = String::new();

    input.read_to_string(&mut s).unwrap();

    println!("a: {}", solve_a(&s));
    println!("b: {}", solve_b(&s));
}

fn solve_a(input: &str) -> i64 {
    solve(input).0
}

fn solve(input: &str) -> (i64, i64) {
    let mut registers = HashMap::new();
    let mut max_ever = 0;

    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        assert_eq!(parts[3], "if");
        let cmp_reg = *registers.entry(parts[4]).or_insert(0);
        let cmp_op = parts[5];
        let cmp_val = parts[6].parse::<i64>().unwrap();
        let do_op = match cmp_op {
            "<" => cmp_reg < cmp_val,
            ">" => cmp_reg > cmp_val,
            "==" => cmp_reg == cmp_val,
            "!=" => cmp_reg != cmp_val,
            "<=" => cmp_reg <= cmp_val,
            ">=" => cmp_reg >= cmp_val,
            _ => panic!("Unhandled: {}", cmp_op),
        };

        if do_op {
            let reg = registers.entry(parts[0]).or_insert(0);
            let op = parts[1];
            let val = parts[2].parse::<i64>().unwrap();
            if op == "inc" {
                *reg += val
            } else {
                *reg -= val
            }

            max_ever = cmp::max(max_ever, *reg);
        }
    }

    return (
        *registers.values().into_iter().max().unwrap(),
        max_ever
    )
}

/*
--- Part Two ---

To be safe, the CPU also needs to know the highest value held in any register during this process so that it can decide how much memory to allocate to these operations. For example, in the above instructions, the highest value ever held was 10 (in register c after the third instruction was evaluated).
*/

fn solve_b(input: &str) -> i64 {
    solve(input).1
}

#[test]
fn test_a() {
    assert_eq!(solve_a("b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10"), 1);
}

#[test]
fn test_b() {
    assert_eq!(solve_b("b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10"), 10);
}