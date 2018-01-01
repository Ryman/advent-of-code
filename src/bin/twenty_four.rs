/*
--- Day 24: Electromagnetic Moat ---

The CPU itself is a large, black building surrounded by a bottomless pit. Enormous metal tubes extend outward from the side of the building at regular intervals and descend down into the void. There's no way to cross, but you need to get inside.

No way, of course, other than building a bridge out of the magnetic components strewn about nearby.

Each component has two ports, one on each end. The ports come in all different types, and only matching types can be connected. You take an inventory of the components by their port types (your puzzle input). Each port is identified by the number of pins it uses; more pins mean a stronger connection for your bridge. A 3/7 component, for example, has a type-3 port on one side, and a type-7 port on the other.

Your side of the pit is metallic; a perfect surface to connect a magnetic, zero-pin port. Because of this, the first port you use must be of type 0. It doesn't matter what type of port you end with; your goal is just to make the bridge as strong as possible.

The strength of a bridge is the sum of the port types in each component. For example, if your bridge is made of components 0/3, 3/7, and 7/4, your bridge has a strength of 0+3 + 3+7 + 7+4 = 24.

For example, suppose you had the following components:

0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10

With them, you could make the following valid bridges:

    0/1
    0/1--10/1
    0/1--10/1--9/10
    0/2
    0/2--2/3
    0/2--2/3--3/4
    0/2--2/3--3/5
    0/2--2/2
    0/2--2/2--2/3
    0/2--2/2--2/3--3/4
    0/2--2/2--2/3--3/5

(Note how, as shown by 10/1, order of ports within a component doesn't matter. However, you may only use each port on a component once.)

Of these bridges, the strongest one is 0/1--10/1--9/10; it has a strength of 0+1 + 1+10 + 10+9 = 31.

What is the strength of the strongest bridge you can make with the components you have available?
*/

#[macro_use]
extern crate text_io;

use std::io::Read;
use std::fs::File;
use std::collections::{HashSet, HashMap};
use std::cmp;

fn main() {
    let mut input = File::open("inputs/twenty_four.txt").unwrap();
    let mut s = String::new();

    input.read_to_string(&mut s).unwrap();

    println!("a: {}", solve_a(&s));
    println!("b: {}", solve_b(&s));
}

fn setup(input: &str) -> (HashMap<usize, Vec<usize>>, HashSet<(usize, usize)>) {
    let components = input.lines().map(|line| {
        let (a, b): (usize, usize);
        scan!(line.bytes() => "{}/{}", a, b);
        (a, b)
    });

    let mut map = HashMap::new();

    for (a, b) in components {
        map.entry(a).or_insert(vec![]).push(b);
        map.entry(b).or_insert(vec![]).push(a);
    }

    (map, HashSet::new())
}

fn solve_a(input: &str) -> usize {
    let (map, mut used) = setup(input);

    max_weight(0, &map, &mut used)
}

fn max_weight(left: usize, map: &HashMap<usize, Vec<usize>>, used: &mut HashSet<(usize, usize)>) -> usize {
    let mut max = left;
    let components = &map[&left];

    for &right in components {
        let key = (cmp::min(left, right), cmp::max(left, right));
        if used.contains(&key) { continue }

        used.insert(key.clone());
        let subweight = max_weight(right, map, used);
        let weight = left + left + subweight;
        max = cmp::max(max, weight);
        used.remove(&key);
    }

    max
}

/*
-- Part Two ---

The bridge you've built isn't long enough; you can't jump the rest of the way.

In the example above, there are two longest bridges:

    0/2--2/2--2/3--3/4
    0/2--2/2--2/3--3/5

Of them, the one which uses the 3/5 component is stronger; its strength is 0+2 + 2+2 + 2+3 + 3+5 = 19.

What is the strength of the longest bridge you can make? If you can make multiple bridges of the longest length, pick the strongest one.

*/

fn solve_b(input: &str) -> usize {
    let (map, mut used) = setup(input);

    max_length_and_weight(0, 0, &map, &mut used).1
}

fn max_length_and_weight(
    left: usize,
    depth: usize,
    map: &HashMap<usize, Vec<usize>>,
    used: &mut HashSet<(usize, usize)>
) -> (usize, usize) {
    let mut max = (depth, left);
    let components = &map[&left];

    for &right in components {
        let key = (cmp::min(left, right), cmp::max(left, right));
        if used.contains(&key) { continue }

        used.insert(key.clone());

        let (depth, subweight) = max_length_and_weight(right, depth + 1, map, used);
        let weight = left + left + subweight;

        max = cmp::max(max, (depth, weight));
        used.remove(&key);
    }

    max
}

#[test]
fn test_a() {
    assert_eq!(
        solve_a(
"0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10"
        ),
        31
    );
}

#[test]
fn test_b() {
    assert_eq!(
        solve_b(
"0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10"
        ),
        19
    );
}