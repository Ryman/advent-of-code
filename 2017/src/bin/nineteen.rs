/*
--- Day 19: A Series of Tubes ---

Somehow, a network packet got lost and ended up here. It's trying to follow a routing diagram (your puzzle input), but it's confused about where to go.

Its starting point is just off the top of the diagram. Lines (drawn with |, -, and +) show the path it needs to take, starting by going down onto the only line connected to the top of the diagram. It needs to follow this path until it reaches the end (located somewhere within the diagram) and stop there.

Sometimes, the lines cross over each other; in these cases, it needs to continue going the same direction, and only turn left or right when there's no other option. In addition, someone has left letters on the line; these also don't change its direction, but it can use them to keep track of where it's been. For example:

     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+

Given this diagram, the packet needs to take the following path:

    Starting at the only line touching the top of the diagram, it must go down, pass through A, and continue onward to the first +.
    Travel right, up, and right, passing through B in the process.
    Continue down (collecting C), right, and up (collecting D).
    Finally, go all the way left through E and stopping at F.

Following the path to the end, the letters it sees on its path are ABCDEF.

The little packet looks up at you, hoping you can help it find the way. What letters will it see (in the order it would see them) if it follows the path? (The routing diagram is very wide; make sure you view it without line wrapping.)
*/

use std::io::Read;
use std::fs::File;
use std::cmp;

fn main() {
    let mut input = File::open("inputs/nineteen.txt").unwrap();
    let mut s = String::new();

    input.read_to_string(&mut s).unwrap();

    println!("a: {}", solve_a(&s));
    println!("b: {}", solve_b(&s));
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid = vec![];
    let mut max_len = 0;
    for line in input.lines() {
        let chars = line.chars().collect::<Vec<_>>();
        max_len = cmp::max(max_len, chars.len());
        grid.push(chars);
    }

    for row in &mut grid {
        let len = row.len();
        if len < max_len {
            row.extend((0..max_len - len).map(|_| ' '));
        }
    }

    grid
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn solve_a(input: &str) -> String {
    solve(input).0
}

fn solve(input: &str) -> (String, usize) {
    use Direction::*;

    let grid = parse_grid(input);
    let (n, m) = (grid.len(), grid[0].len());
    let mut y = 0;
    let mut x = grid[0].iter().position(|&c| c == '|').unwrap();
    let mut seen = String::new();
    let mut direction = Down;
    let mut steps = 0;

    loop {
        steps += 1;

        match direction {
            Up | Down => {
                if direction == Down {
                    if y + 1 == n { break }
                    y += 1;
                } else {
                    if  y == 0 { break }
                    y -= 1;
                }

                match grid[y][x] {
                    '|' | '-' => {},
                    '+' => {
                        match (grid[y].get(x - 1), grid[y].get(x + 1)) {
                            (Some(&c), Some(&' ')) | (Some(&c), None) => {
                                direction = Direction::Left;
                                x -= 1;
                                if c != '-' { seen.push(c) }
                            },
                            (Some(&' '), Some(&c)) | (None, Some(&c)) => {
                                direction = Direction::Right;
                                x += 1;
                                if c != '-' { seen.push(c) }
                            },
                            _ => unreachable!()
                        }

                        steps += 1;
                    }
                    ' ' => break,
                    c => seen.push(c),
                }
            }
            Left | Right => {
                if direction == Left {
                    if x == 0 { break }
                    x -= 1;
                } else {
                    if x + 1 == m { break }
                    x += 1;
                }

                match grid[y][x] {
                    '|' | '-' => {},
                    '+' => {
                        match (grid.get(y - 1).map(|r| r[x]), grid.get(y + 1).map(|r| r[x])) {
                            (Some(c), Some(' ')) | (Some(c), None) => {
                                direction = Direction::Up;
                                y -= 1;
                                if c != '|' { seen.push(c) }
                            },
                            (Some(' '), Some(c)) | (None, Some(c)) => {
                                direction = Direction::Down;
                                y += 1;
                                if c != '|' { seen.push(c) }
                            },
                            _ => unreachable!()
                        }

                        steps += 1;
                    }
                    ' ' => break,
                    c => seen.push(c),
                }
            }
        }
    }

    (seen, steps)
}

/*
--- Part Two ---

The packet is curious how many steps it needs to go.

For example, using the same routing diagram from the example above...

     |
     |  +--+
     A  |  C
 F---|--|-E---+
     |  |  |  D
     +B-+  +--+

...the packet would go:

    6 steps down (including the first line at the top of the diagram).
    3 steps right.
    4 steps up.
    3 steps right.
    4 steps down.
    3 steps right.
    2 steps up.
    13 steps left (including the F it stops on).

This would result in a total of 38 steps.

How many steps does the packet need to go?

*/

fn solve_b(input: &str) -> usize {
    solve(input).1
}

#[test]
fn test_a() {
    assert_eq!(solve_a(
"     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+ "), "ABCDEF");
}

#[test]
fn test_b() {
    assert_eq!(solve_b(
"     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+ "), 38);
}