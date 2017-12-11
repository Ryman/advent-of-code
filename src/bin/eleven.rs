/*
--- Day 11: Hex Ed ---

Crossing the bridge, you've barely reached the other side of the stream when a program comes up to you, clearly in distress. "It's my child process," she says, "he's gotten lost in an infinite grid!"

Fortunately for her, you have plenty of experience with infinite grids.

Unfortunately for you, it's a hex grid.

The hexagons ("hexes") in this grid are aligned such that adjacent hexes can be found to the north, northeast, southeast, south, southwest, and northwest:

  \ n  /
nw +--+ ne
  /    \
-+      +-
  \    /
sw +--+ se
  / s  \

You have the path the child process took. Starting where he started, you need to determine the fewest number of steps required to reach him. (A "step" means to move from the hex you are in to any adjacent hex.)

For example:

    ne,ne,ne is 3 steps away.
    ne,ne,sw,sw is 0 steps away (back where you started).
    ne,ne,s,s is 2 steps away (se,se).
    se,sw,se,sw,sw is 3 steps away (s,s,sw).
*/

use std::io::Read;
use std::fs::File;
use std::cmp;

fn main() {
    let mut input = File::open("inputs/eleven.txt").unwrap();
    let mut s = String::new();

    input.read_to_string(&mut s).unwrap();

    println!("a: {}", solve_a(&s));
    println!("b: {}", solve_b(&s));
}

fn solve_a(input: &str) -> i64 {
    solve(input).0
}

fn solve(input: &str) -> (i64, i64) {
    let (mut x, mut y) = (0i64, 0i64);
    let mut furthest = 0;
    let mut steps = 0;

    for step in input.split(',') {
        match step {
            "ne" => {
                x += 1;
                y -= 1;
            }
            "se" => {
                x += 1;
            }
            "s" => {
                y += 1;
            }
            "sw" => {
                x -= 1;
                y += 1;
            }
            "nw" => {
                x -= 1;
            }
            "n" => {
                y -= 1;
            }
            _ => unreachable!()
        }

        steps = cmp::max(x.abs(), y.abs());
        furthest = cmp::max(furthest, steps);
    }

    (steps, furthest)
}

/*
--- Part Two ---

How many steps away is the furthest he ever got from his starting position?
*/

fn solve_b(input: &str) -> i64 {
    solve(input).1
}

#[test]
fn test_a() {
    assert_eq!(solve_a("ne,ne,ne"), 3);
    assert_eq!(solve_a("ne,ne,sw,sw"), 0);
    assert_eq!(solve_a("ne,ne,s,s"), 2);
    assert_eq!(solve_a("se,sw,se,sw,sw"), 3);
}

#[test]
fn test_b() {
    assert_eq!(solve_b("ne,ne,ne"), 3);
    assert_eq!(solve_b("ne,ne,sw,sw"), 2);
    assert_eq!(solve_b("ne,ne,s,s"), 2);
    assert_eq!(solve_b("se,sw,se,sw,sw"), 3);
}