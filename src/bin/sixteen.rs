/*
--- Day 16: Permutation Promenade ---

You come upon a very unusual sight; a group of programs here appear to be dancing.

There are sixteen programs in total, named a through p. They start by standing in a line: a stands in position 0, b stands in position 1, and so on until p, which stands in position 15.

The programs' dance consists of a sequence of dance moves:

    Spin, written sX, makes X programs move from the end to the front, but maintain their order otherwise. (For example, s3 on abcde produces cdeab).
    Exchange, written xA/B, makes the programs at positions A and B swap places.
    Partner, written pA/B, makes the programs named A and B swap places.

For example, with only five programs standing in a line (abcde), they could do the following dance:

    s1, a spin of size 1: eabcd.
    x3/4, swapping the last two programs: eabdc.
    pe/b, swapping programs e and b: baedc.

After finishing their dance, the programs end up in order baedc.

You watch the dance for a while and record their dance moves (your puzzle input). In what order are the programs standing after their dance?
*/

use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let mut input = File::open("inputs/sixteen.txt").unwrap();
    let mut s = String::new();

    input.read_to_string(&mut s).unwrap();

    println!("a: {}", solve_a(16, &s));
    println!("b: {}", solve_b(16, &s, 1000000000));
}

fn solve_a(count: usize, input: &str) -> String {
    let chars = (0..count).map(|i| (i as u8 + b'a') as char).collect::<Vec<_>>();
    solve(chars, input).into_iter().collect()
}

fn solve(mut chars: Vec<char>, input: &str) -> Vec<char> {
    for command in input.split(',') {
        match &command[0..1] {
            "s" => {
                let pivot = command[1..].parse::<usize>().unwrap();
                let next = {
                    let len = chars.len();
                    let (before, after) = chars.split_at(len - pivot);
                    let mut after = after.to_vec();
                    after.extend(before);
                    after
                };
                chars = next;
            }
            "x" => {
                let mut split = command[1..].split('/').map(|s| s.parse().unwrap());
                let left = split.next().unwrap();
                let right = split.next().unwrap();

                chars.swap(left, right);
            }
            "p" => {
                let mut split = command[1..].split('/');
                let left = split.next().unwrap();
                let right = split.next().unwrap();

                let left_idx = chars.iter().position(|&c| left.starts_with(c)).unwrap();
                let right_idx = chars.iter().position(|&c| right.starts_with(c)).unwrap();

                chars.swap(left_idx, right_idx);
            }
            _ => unreachable!()
        }
    }

    chars
}

/*
--- Part Two ---

Now that you're starting to get a feel for the dance moves, you turn your attention to the dance as a whole.

Keeping the positions they ended up in from their previous dance, the programs perform it again and again: including the first dance, a total of one billion (1000000000) times.

In the example above, their second dance would begin with the order baedc, and use the same dance moves:

    s1, a spin of size 1: cbaed.
    x3/4, swapping the last two programs: cbade.
    pe/b, swapping programs e and b: ceadb.

In what order are the programs standing after their billion dances?

*/

fn solve_b(count: usize, input: &str, reps: usize) -> String {
    let mut idx = 0;
    let mut chars = (0..count).map(|i| (i as u8 + b'a') as char).collect::<Vec<_>>();
    let mut map = HashMap::<_, usize>::new();
    let mut positions = vec![];

    loop {
        let result = solve(chars, input);
        chars = result.clone();

        if map.contains_key(&result) {
            assert_eq!(map[&result], 0);
            break
        }

        map.insert(result.clone(), idx);
        positions.push(result);
        idx += 1;
    }

    positions[(reps - 1) % idx].iter().collect()
}

#[test]
fn test_a() {
    assert_eq!(solve_a(5, "s1,x3/4,pe/b"), "baedc");
}

#[test]
fn test_b() {
    assert_eq!(solve_b(5, "s1,x3/4,pe/b", 2), "ceadb");
}