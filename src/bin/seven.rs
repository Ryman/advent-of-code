/*
--- Day 7: Recursive Circus ---

Wandering further through the circuits of the computer, you come upon a tower of programs that have gotten themselves into a bit of trouble. A recursive algorithm has gotten out of hand, and now they're balanced precariously in a large tower.

One program at the bottom supports the entire tower. It's holding a large disc, and on the disc are balanced several more sub-towers. At the bottom of these sub-towers, standing on the bottom disc, are other programs, each holding their own disc, and so on. At the very tops of these sub-sub-sub-...-towers, many programs stand simply keeping the disc below them balanced but with no disc of their own.

You offer to help, but first you need to understand the structure of these towers. You ask each program to yell out their name, their weight, and (if they're holding a disc) the names of the programs immediately above them balancing on that disc. You write this information down (your puzzle input). Unfortunately, in their panic, they don't do this in an orderly fashion; by the time you're done, you're not sure which program gave which information.

For example, if your list is the following:

pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)

...then you would be able to recreate the structure of the towers that looks like this:

                gyxo
              /
         ugml - ebii
       /      \
      |         jptl
      |
      |         pbga
     /        /
tknk --- padx - havc
     \        \
      |         qoyq
      |
      |         ktlj
       \      /
         fwft - cntj
              \
                xhth

In this example, tknk is at the bottom of the tower (the bottom program), and is holding up ugml, padx, and fwft. Those programs are, in turn, holding up other programs; in this example, none of those programs are holding up any other programs, and are all the tops of their own towers. (The actual tower balancing in front of you is much larger.)

Before you're ready to help them, you need to make sure your information is correct. What is the name of the bottom program?

*/

use std::io::Read;
use std::fs::File;
use std::collections::{HashSet, HashMap};

fn main() {
    let mut input = File::open("inputs/seven.txt").unwrap();
    let mut s = String::new();

    input.read_to_string(&mut s).unwrap();

    println!("a: {}", solve_a(&s));
    println!("b: {}", solve_b(&s));
}

fn solve_a(input: &str) -> &str {
    let mut parents = vec![];
    let mut children = HashSet::new();

    for line in input.lines() {
        let mut split = line.split("->");
        let name_weight = split.next().unwrap();

        if let Some(list) = split.next() {
            // ugml (68)
            let parent = name_weight.split(' ').next().unwrap();
            parents.push(parent);

            // gyxo, ebii, jptl
            for child in list.trim().split(',').map(|s| s.trim()) {
                children.insert(child);
            }
        }
    }

    for parent in &parents {
        if !children.contains(parent) {
            return parent
        }
    }

    println!("{:?}", parents);
    println!("{:?}", children);
    unreachable!()
}

/*
--- Part Two ---

The programs explain the situation: they can't get down. Rather, they could get down, if they weren't expending all of their energy trying to keep the tower balanced. Apparently, one program has the wrong weight, and until it's fixed, they're stuck here.

For any program holding a disc, each program standing on that disc forms a sub-tower. Each of those sub-towers are supposed to be the same weight, or the disc itself isn't balanced. The weight of a tower is the sum of the weights of the programs in that tower.

In the example above, this means that for ugml's disc to be balanced, gyxo, ebii, and jptl must all have the same weight, and they do: 61.

However, for tknk to be balanced, each of the programs standing on its disc and all programs above it must each match. This means that the following sums must all be the same:

    ugml + (gyxo + ebii + jptl) = 68 + (61 + 61 + 61) = 251
    padx + (pbga + havc + qoyq) = 45 + (66 + 66 + 66) = 243
    fwft + (ktlj + cntj + xhth) = 72 + (57 + 57 + 57) = 243

As you can see, tknk's disc is unbalanced: ugml's stack is heavier than the other two. Even though the nodes above ugml are balanced, ugml itself is too heavy: it needs to be 8 units lighter for its stack to weigh 243 and keep the towers balanced. If this change were made, its weight would be 60.

Given that exactly one program is the wrong weight, what would its weight need to be to balance the entire tower?
*/

fn solve_b(input: &str) -> usize {
    let mut parents = HashMap::new();
    let mut weights = HashMap::new();
    let mut children = HashSet::new();

    for line in input.lines() {
        let mut split = line.split("->");

        // ugml (68)
        let name_weight = split.next().unwrap();
        let mut nw = name_weight.split(' ');
        let (parent, weight) = (
            nw.next().unwrap(),
            nw.next().unwrap()
                .trim_matches(&['(', ')'][..])
                .parse::<usize>().unwrap()
        );
        weights.insert(parent, weight);

        if let Some(list) = split.next() {
            let parent = parents.entry(parent).or_insert(vec![]);

            // gyxo, ebii, jptl
            for child in list.trim().split(',').map(|s| s.trim()) {
                children.insert(child);
                parent.push(child);
            }
        }
    }


    let mut current = None;
    for (parent, _) in &parents {
        if !children.contains(parent) {
            current = Some(parent);
        }
    }

    let mut current = current.unwrap();

    fn get_weight(parent: &str, parents: &HashMap<&str, Vec<&str>>, weights: &HashMap<&str, usize>) -> usize {
        let mut sum = 0;
        sum += weights[parent];
        for child in parents.get(parent).unwrap_or(&vec![]).iter() {
            sum += get_weight(child, parents, weights);
        }
        sum
    }

    loop {
        let mut child_weights = HashMap::new();
        for child in parents.get(current).unwrap_or(&vec![]).iter() {
            let cw =  get_weight(child, &parents, &weights);
            *child_weights.entry(cw).or_insert(0) += 1;

            println!("[{}] {} ({}) => {}", current, child, weights[child],cw);
        }

        let mut child_weights = child_weights.into_iter().collect::<Vec<(_, _)>>();
        child_weights.sort_by_key(|&(_, count)| -count);

        let (common, _) = child_weights[0];
        if child_weights.len() == 1 {
            panic!("{:?}", child_weights)
        }

        for child in &parents[current] {
            let cw =  get_weight(child, &parents, &weights);
            if common == cw { continue }

            // we want to check if all its children are balanced
            // if they are then we need to adjust the current value
            // if they are not then we recurse
            let mut subchild_weights = vec![];
            for subchild in parents.get(child).unwrap_or(&vec![]).iter() {
                let cw =  get_weight(subchild, &parents, &weights);
                subchild_weights.push(cw);
            }

            let culprit = subchild_weights.iter()
                .position(|&cw| cw != subchild_weights[0]);
            println!("subchildren: {:?}", subchild_weights);

            if let Some(_) = culprit {
                // If we have an unbalanced child then check them instead
                current = child;
                println!("recursing to {}", current);
            } else {
                let subweight = subchild_weights.get(0).unwrap_or(&0) * subchild_weights.len();

                let adjusted = common - subweight;

                println!("Should adjust {} to {} - {} = {}", cw, common, subweight, adjusted);
                return adjusted;
            }
        }

        println!("{:?}", child_weights);
    }
}

#[test]
fn test_a() {
    assert_eq!(solve_a("pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"), "tknk");
}

#[test]
fn test_b() {
    assert_eq!(solve_b("pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"), 60);
}