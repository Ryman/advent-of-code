/*
--- Day 22: Sporifica Virus ---

Diagnostics indicate that the local grid computing cluster has been contaminated with the Sporifica Virus. The grid computing cluster is a seemingly-infinite two-dimensional grid of compute nodes. Each node is either clean or infected by the virus.

To prevent overloading the nodes (which would render them useless to the virus) or detection by system administrators, exactly one virus carrier moves through the network, infecting or cleaning nodes as it moves. The virus carrier is always located on a single node in the network (the current node) and keeps track of the direction it is facing.

To avoid detection, the virus carrier works in bursts; in each burst, it wakes up, does some work, and goes back to sleep. The following steps are all executed in order one time each burst:

    If the current node is infected, it turns to its right. Otherwise, it turns to its left. (Turning is done in-place; the current node does not change.)
    If the current node is clean, it becomes infected. Otherwise, it becomes cleaned. (This is done after the node is considered for the purposes of changing direction.)
    The virus carrier moves forward one node in the direction it is facing.

Diagnostics have also provided a map of the node infection status (your puzzle input). Clean nodes are shown as .; infected nodes are shown as #. This map only shows the center of the grid; there are many more nodes beyond those shown, but none of them are currently infected.

The virus carrier begins in the middle of the map facing up.

For example, suppose you are given a map like this:

..#
#..
...

Then, the middle of the infinite grid looks like this, with the virus carrier's position marked with [ ]:

. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . # . . .
. . . #[.]. . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .

The virus carrier is on a clean node, so it turns left, infects the node, and moves left:

. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . # . . .
. . .[#]# . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .

The virus carrier is on an infected node, so it turns right, cleans the node, and moves up:

. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . .[.]. # . . .
. . . . # . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .

Four times in a row, the virus carrier finds a clean, infects it, turns left, and moves forward, ending in the same place and still facing up:

. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . #[#]. # . . .
. . # # # . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .

Now on the same node as before, it sees an infection, which causes it to turn right, clean the node, and move forward:

. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . # .[.]# . . .
. . # # # . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .

After the above actions, a total of 7 bursts of activity had taken place. Of them, 5 bursts of activity caused an infection.

After a total of 70, the grid looks like this, with the virus carrier facing up:

. . . . . # # . .
. . . . # . . # .
. . . # . . . . #
. . # . #[.]. . #
. . # . # . . # .
. . . . . # # . .
. . . . . . . . .
. . . . . . . . .

By this time, 41 bursts of activity caused an infection (though most of those nodes have since been cleaned).

After a total of 10000 bursts of activity, 5587 bursts will have caused an infection.

Given your actual map, after 10000 bursts of activity, how many bursts cause a node to become infected? (Do not count nodes that begin infected.)
*/

use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

const INFECTED: usize = 2;

fn main() {
    let mut input = File::open("inputs/twenty_two.txt").unwrap();
    let mut s = String::new();

    input.read_to_string(&mut s).unwrap();

    println!("a: {}", solve_a(&s));
    println!("b: {}", solve_b(&s));
}

fn parse_input(input: &str) -> HashMap<(isize, isize), usize> {
    let mut map = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        let width = line.len() as isize;
        let offset = width / 2;

        for (x, c) in line.chars().enumerate() {
            let coord = (x as isize - offset, y as isize - offset);
            let infected = if c == '#' { INFECTED } else { 0 };
            map.insert(coord, infected);
        }
    }

    map
}

fn solve_a(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut x = 0;
    let mut y = 0;
    let mut direction = 0;
    let mut infections = 0;

    for _ in 0..10_000 {
        let current_node = grid.entry((x, y)).or_insert(0);

        // If the current node is infected, it turns to its right. Otherwise, it turns to its left. (Turning is done in-place; the current node does not change.)
        // If the current node is clean, it becomes infected. Otherwise, it becomes cleaned. (This is done after the node is considered for the purposes of changing direction.)
        if *current_node == INFECTED {
            direction = (direction + 1) % 4;
            *current_node = 0;
        } else {
            direction = (direction - 1 + 4) % 4;
            infections += 1;
            *current_node = INFECTED;
        }

        // The virus carrier moves forward one node in the direction it is facing.
        match direction {
            // up
            0 => y -= 1,
            // right
            1 => x += 1,
            // down
            2 => y += 1,
            // left
            3 => x -= 1,
            _ => unreachable!()
        }
    }

    infections
}

/*
--- Part Two ---

As you go to remove the virus from the infected nodes, it evolves to resist your attempt.

Now, before it infects a clean node, it will weaken it to disable your defenses. If it encounters an infected node, it will instead flag the node to be cleaned in the future. So:

    Clean nodes become weakened.
    Weakened nodes become infected.
    Infected nodes become flagged.
    Flagged nodes become clean.

Every node is always in exactly one of the above states.

The virus carrier still functions in a similar way, but now uses the following logic during its bursts of action:

    Decide which way to turn based on the current node:
        If it is clean, it turns left.
        If it is weakened, it does not turn, and will continue moving in the same direction.
        If it is infected, it turns right.
        If it is flagged, it reverses direction, and will go back the way it came.
    Modify the state of the current node, as described above.
    The virus carrier moves forward one node in the direction it is facing.

Start with the same map (still using . for clean and # for infected) and still with the virus carrier starting in the middle and facing up.

Using the same initial state as the previous example, and drawing weakened as W and flagged as F, the middle of the infinite grid looks like this, with the virus carrier's position again marked with [ ]:

. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . # . . .
. . . #[.]. . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .

This is the same as before, since no initial nodes are weakened or flagged. The virus carrier is on a clean node, so it still turns left, instead weakens the node, and moves left:

. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . # . . .
. . .[#]W . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .

The virus carrier is on an infected node, so it still turns right, instead flags the node, and moves up:

. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . .[.]. # . . .
. . . F W . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .

This process repeats three more times, ending on the previously-flagged node and facing right:

. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . W W . # . . .
. . W[F]W . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .

Finding a flagged node, it reverses direction and cleans the node:

. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . W W . # . . .
. .[W]. W . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .

The weakened node becomes infected, and it continues in the same direction:

. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . W W . # . . .
.[.]# . W . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .

Of the first 100 bursts, 26 will result in infection. Unfortunately, another feature of this evolved virus is speed; of the first 10000000 bursts, 2511944 will result in infection.

Given your actual map, after 10000000 bursts of activity, how many bursts cause a node to become infected? (Do not count nodes that begin infected.)
*/

fn solve_b(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut x = 0;
    let mut y = 0;
    let mut direction = 0;
    let mut infections = 0;

    for _ in 0..10_000_000 {
        let current_node = grid.entry((x, y)).or_insert(0);

        // Decide which way to turn based on the current node:
        match *current_node {
            //     If it is clean, it turns left.
            0 => direction = (direction - 1 + 4) % 4,
            //     If it is weakened, it does not turn, and will continue moving in the same direction.
            1 => infections += 1,
            //     If it is infected, it turns right.
            2 => direction = (direction + 1) % 4,
            //     If it is flagged, it reverses direction, and will go back the way it came.
            3 => direction = (direction + 2) % 4,
            _ => unreachable!(),
        }

        // Modify the state of the current node, as described above.
        //     Clean nodes become weakened.
        //     Weakened nodes become infected.
        //     Infected nodes become flagged.
        //     Flagged nodes become clean.
        *current_node = (*current_node + 1) % 4;

        // The virus carrier moves forward one node in the direction it is facing.
        match direction {
            // up
            0 => y -= 1,
            // right
            1 => x += 1,
            // down
            2 => y += 1,
            // left
            3 => x -= 1,
            _ => unreachable!()
        }
    }

    infections
}

#[test]
fn test_a() {
    assert_eq!(
        solve_a(
"..#
#..
..."),
        5587
    );
}

#[test]
fn test_b() {
    assert_eq!(
        solve_b(
"..#
#..
..."),
        2511944
    );
}