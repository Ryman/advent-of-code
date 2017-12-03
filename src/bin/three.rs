/*
You come across an experimental new kind of memory stored on an infinite two-dimensional grid.

Each square on the grid is allocated in a spiral pattern starting at a location marked 1 and then counting up while spiraling outward. For example, the first few squares are allocated like this:

17  16  15  14  13
18   5   4   3  12
19   6   1   2  11
20   7   8   9  10
21  22  23---> ...

While this is very space-efficient (no squares are skipped), requested data must be carried back to square 1 (the location of the only access port for this memory system) by programs that can only move up, down, left, or right. They always take the shortest path: the Manhattan Distance between the location of the data and square 1.

For example:

    Data from square 1 is carried 0 steps, since it's at the access port.
    Data from square 12 is carried 3 steps, such as: down, left, left.
    Data from square 23 is carried only 2 steps: up twice.
    Data from square 1024 must be carried 31 steps.

How many steps are required to carry the data from the square identified in your puzzle input all the way to the access port?
*/
use std::io::Read;
use std::fs::File;
use std::cmp;

fn main() {
    let mut input = File::open("inputs/three.txt").unwrap();
    let mut s = String::new();

    input.read_to_string(&mut s).unwrap();
    let input = s.parse().unwrap();
    println!("a: {}", solve_a(input));
    println!("b: {}", solve_b(input));
}

fn solve_a(target: u32) -> u32 {
    let mut n = (target as f64).sqrt().ceil() as i32;

    // always generate an odd sized spiral
    if n % 2 == 0 { n += 1 }

    let mut current = n * n;
    let mut depth = 0;
    // let mut x = n - 1;
    // let mut y = n - 1;

    macro_rules! check {
        ($y:ident, $x:ident) => {{
            if current == target as i32 {
                let mid = n / 2;
                println!("Found solution at {:?} mid = {:?}", ($x, $y), (mid, mid));
                let dx = (mid - $x).abs();
                let dy = (mid - $y).abs();
                return (dx + dy) as u32;
            }
            current -= 1;
        }}
    }

    loop {
        // bottom row
        let bottom_row = n - depth - 1;
        for x in (depth..n - depth).rev() {
            // print!("br ({}, {}) ", bottom_row, x);
            check!(bottom_row, x)
        }

        // left column
        let left_column = depth;
        for y in (depth..n - depth - 1).rev() {
            // print!("lc ({}, {}) ", y, left_column);
            check!(y, left_column)
        }

        // top row
        let top_row = depth;
        for x in depth + 1..n - depth {
            // print!("tr ({}, {}) ", top_row, x);
            check!(top_row, x)
        }

        // right column
        let right_column = n - depth - 1;
        for y in depth + 1..n - depth - 1 {
            // print!("rc ({}, {}) ", y, right_column);
            check!(y, right_column)
        }

        println!("");
        depth += 1;
    }
}

/*
--- Part Two ---

As a stress test on the system, the programs here clear the grid and then store the value 1 in square 1. Then, in the same allocation order as shown above, they store the sum of the values in all adjacent squares, including diagonals.

So, the first few squares' values are chosen as follows:

    Square 1 starts with the value 1.
    Square 2 has only one adjacent filled square (with value 1), so it also stores 1.
    Square 3 has both of the above squares as neighbors and stores the sum of their values, 2.
    Square 4 has all three of the aforementioned squares as neighbors and stores the sum of their values, 4.
    Square 5 only has the first and fourth squares as neighbors, so it gets the value 5.

Once a square is written, its value does not change. Therefore, the first few squares would receive the following values:

147  142  133  122   59
304    5    4    2   57
330   10    1    1   54
351   11   23   25   26
362  747  806--->   ...

What is the first value written that is larger than your puzzle input?
*/

fn solve_b(target: u32) -> u32 {
    let mut n = (target as f64).sqrt().ceil() as i32;

    // always generate an odd sized spiral
    if n % 2 == 0 {
        n += 1;
    }
    n += 4; // ensure we always have enough neighbours to skip bounds checks

    let mut matrix = vec![vec![0; n as usize]; n as usize];

    let mut depth = n / 2;
    macro_rules! m {
        ($y:ident, $x:ident) => (
            // get neighbours
            let (x, y) = ($x as usize, $y as usize);
            let mut sum = 0;
            for dy in 0..3 {
                for dx in 0..3 {
                    sum += matrix[y + dy - 1][x + dx - 1];
                }
            }

            if sum > target { return sum }

            matrix[$y as usize][$x as usize] = cmp::max(sum, 1);
        )
    }
    loop {
        // right column
        let right_column = n - depth - 1;
        for y in (depth + 1..n - depth - 1).rev() {
            // print!("rc ({}, {}) = {} ", y, right_column, current);
            m!(y, right_column);
        }

        // top row
        let top_row = depth;
        for x in (depth + 1..n - depth).rev() {
            // print!("tr ({}, {}) = {} ", top_row, x, current);
            m!(top_row, x);
        }

        // left column
        let left_column = depth;
        for y in depth..n - depth - 1 {
            // print!("lc ({}, {}) = {} ", y, left_column, current);
            m!(y, left_column);
        }

        // bottom row
        let bottom_row = n - depth - 1;
        for x in depth..n - depth {
            // print!("br ({}, {}) = {} ", bottom_row, x, current);
            m!(bottom_row, x);
        }

        // println!("");
        depth -= 1;
    }
}

#[test]
fn test_a() {
    assert_eq!(solve_a(1), 0);
    assert_eq!(solve_a(12), 3);
    assert_eq!(solve_a(23), 2);
    assert_eq!(solve_a(1024), 31);
}

#[test]
fn test_b() {
    // 147  142  133  122   59
    // 304    5    4    2   57
    // 330   10    1    1   54
    // 351   11   23   25   26
    // 362  747  806--->   ...
    assert_eq!(solve_b(1), 2);
    assert_eq!(solve_b(5), 10);
    assert_eq!(solve_b(330), 351);
    assert_eq!(solve_b(740), 747);
}