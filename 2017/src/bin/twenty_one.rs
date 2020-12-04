/*
--- Day 21: Fractal Art ---

You find a program trying to generate some art. It uses a strange process that involves repeatedly enhancing the detail of an image through a set of rules.

The image consists of a two-dimensional square grid of pixels that are either on (#) or off (.). The program always begins with this pattern:

.#.
..#
###

Because the pattern is both 3 pixels wide and 3 pixels tall, it is said to have a size of 3.

Then, the program repeats the following process:

    If the size is evenly divisible by 2, break the pixels up into 2x2 squares, and convert each 2x2 square into a 3x3 square by following the corresponding enhancement rule.
    Otherwise, the size is evenly divisible by 3; break the pixels up into 3x3 squares, and convert each 3x3 square into a 4x4 square by following the corresponding enhancement rule.

Because each square of pixels is replaced by a larger one, the image gains pixels and so its size increases.

The artist's book of enhancement rules is nearby (your puzzle input); however, it seems to be missing rules. The artist explains that sometimes, one must rotate or flip the input pattern to find a match. (Never rotate or flip the output pattern, though.) Each pattern is written concisely: rows are listed as single units, ordered top-down, and separated by slashes. For example, the following rules correspond to the adjacent patterns:

../.#  =  ..
          .#

                .#.
.#./..#/###  =  ..#
                ###

                        #..#
#..#/..../#..#/.##.  =  ....
                        #..#
                        .##.

When searching for a rule to use, rotate and flip the pattern as necessary. For example, all of the following patterns match the same rule:

.#.   .#.   #..   ###
..#   #..   #.#   ..#
###   ###   ##.   .#.

Suppose the book contained the following two rules:

../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#

As before, the program begins with this pattern:

.#.
..#
###

The size of the grid (3) is not divisible by 2, but it is divisible by 3. It divides evenly into a single square; the square matches the second rule, which produces:

#..#
....
....
#..#

The size of this enhanced grid (4) is evenly divisible by 2, so that rule is used. It divides evenly into four squares:

#.|.#
..|..
--+--
..|..
#.|.#

Each of these squares matches the same rule (../.# => ##./#../...), three of which require some flipping and rotation to line up with the rule. The output for the rule is the same in all four cases:

##.|##.
#..|#..
...|...
---+---
##.|##.
#..|#..
...|...

Finally, the squares are joined into a new grid:

##.##.
#..#..
......
##.##.
#..#..
......

Thus, after 2 iterations, the grid contains 12 pixels that are on.

How many pixels stay on after 5 iterations?

*/

#[macro_use]
extern crate text_io;

use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use std::mem;

fn main() {
    let mut input = File::open("inputs/twenty_one.txt").unwrap();
    let mut s = String::new();

    input.read_to_string(&mut s).unwrap();

    println!("a: {}", solve(&s, 5));
    println!("b: {}", solve(&s, 18));
}

fn parse_rules(input: &str) -> HashMap<Vec<Vec<char>>, Vec<Vec<char>>> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let before: String;
        let after: String;

        // .#./..#/### => #..#/..../..../#..#
        scan!(line.bytes() => "{} => {}", before, after);

        let mut before = parse_grid(&before);
        let after = parse_grid(&after);
        let size = before.len();

        // Rotate clockwise 4 times
        for _ in 0..4 {
            let mut rotation: Vec<Vec<_>> = before.clone();

            // Rotations
            for i in 0..size {
                // Take top from the left
                rotation[0][i] = before[i][0];
            }

            for i in 0..size {
                // Take left from bottom
                rotation[size - i - 1][0] = before[size - 1][size - i - 1];
                // Take bottom from right
                rotation[size - 1][size - i - 1] = before[i][size - 1];
                // Take right from top
                rotation[i][size - 1] = before[0][i];
            }

            let mut vertical_flip = rotation.clone();
            let mut horizontal_flip = rotation.clone();
            // Flips for all rotations (not worried about duplicates)
            for i in 0..size {
                // Take top from bottom
                vertical_flip[0][i] = rotation[size - 1][i];
                // Take bottom from top
                vertical_flip[size - 1][i] = rotation[0][i];
                // Take left from right
                horizontal_flip[i][0] = rotation[i][size - 1];
                // Take right from left
                horizontal_flip[i][size - 1] = rotation[i][0];
            }

            // Insert into the map
            map.insert(rotation.clone(), after.clone());
            map.insert(vertical_flip, after.clone());
            map.insert(horizontal_flip, after.clone());

            before = rotation;
        }
    }

    map
}

fn parse_grid(raw: &str) -> Vec<Vec<char>> {
    raw.split('/').map(|row| row.chars().collect()).collect()
}

fn solve(input: &str, iterations: usize) -> usize {
    let rules = parse_rules(input);
    let mut grid = parse_grid(".#./..#/###");

    for _ in 0..iterations {
        let size = grid.len();
        // Important: Try rule 2 before rule 3.
        let split_size = if size % 2 == 0 { 2 } else { 3 };
        let split_count = size / split_size;

        let new_size = split_count * (split_size + 1);
        let old_grid = mem::replace(&mut grid, vec![vec!['x'; new_size]; new_size]);

        for sq_y in 0..split_count {
            for sq_x in 0..split_count {

                let mut current_block = vec![vec!['x'; split_size]; split_size];
                for y in 0..split_size {
                    for x in 0..split_size {
                        let real_y = (sq_y * split_size) + y;
                        let real_x = (sq_x * split_size) + x;

                        current_block[y][x] = old_grid[real_y][real_x];
                    }
                }

                let new_block = &rules[&current_block];
                for y in 0..split_size + 1 {
                    for x in 0..split_size + 1 {
                        let real_y = (sq_y * (split_size + 1)) + y;
                        let real_x = (sq_x * (split_size + 1)) + x;

                        grid[real_y][real_x] = new_block[y][x];
                    }
                }
            }
        }
    }

    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| c == '#')
        .count()
}

/*
--- Part Two ---

How many pixels stay on after 18 iterations?
*/

#[test]
fn test_a() {
    assert_eq!(
        solve(
"../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#",
        2),
        12
    );
}