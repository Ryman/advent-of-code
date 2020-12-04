/*
--- Day 9: Stream Processing ---

A large stream blocks your path. According to the locals, it's not safe to cross the stream at the moment because it's full of garbage. You look down at the stream; rather than water, you discover that it's a stream of characters.

You sit for a while and record part of the stream (your puzzle input). The characters represent groups - sequences that begin with { and end with }. Within a group, there are zero or more other things, separated by commas: either another group or garbage. Since groups can contain other groups, a } only closes the most-recently-opened unclosed group - that is, they are nestable. Your puzzle input represents a single, large group which itself contains many smaller ones.

Sometimes, instead of a group, you will find garbage. Garbage begins with < and ends with >. Between those angle brackets, almost any character can appear, including { and }. Within garbage, < has no special meaning.

In a futile attempt to clean up the garbage, some program has canceled some of the characters within it using !: inside garbage, any character that comes after ! should be ignored, including <, >, and even another !.

You don't see any characters that deviate from these rules. Outside garbage, you only find well-formed groups, and garbage always terminates according to the rules above.

Here are some self-contained pieces of garbage:

    <>, empty garbage.
    <random characters>, garbage containing random characters.
    <<<<>, because the extra < are ignored.
    <{!>}>, because the first > is canceled.
    <!!>, because the second ! is canceled, allowing the > to terminate the garbage.
    <!!!>>, because the second ! and the first > are canceled.
    <{o"i!a,<{i<a>, which ends at the first >.

Here are some examples of whole streams and the number of groups they contain:

    {}, 1 group.
    {{{}}}, 3 groups.
    {{},{}}, also 3 groups.
    {{{},{},{{}}}}, 6 groups.
    {<{},{},{{}}>}, 1 group (which itself contains garbage).
    {<a>,<a>,<a>,<a>}, 1 group.
    {{<a>},{<a>},{<a>},{<a>}}, 5 groups.
    {{<!>},{<!>},{<!>},{<a>}}, 2 groups (since all but the last > are canceled).

Your goal is to find the total score for all groups in your input. Each group is assigned a score which is one more than the score of the group that immediately contains it. (The outermost group gets a score of 1.)

    {}, score of 1.
    {{{}}}, score of 1 + 2 + 3 = 6.
    {{},{}}, score of 1 + 2 + 2 = 5.
    {{{},{},{{}}}}, score of 1 + 2 + 3 + 3 + 3 + 4 = 16.
    {<a>,<a>,<a>,<a>}, score of 1.
    {{<ab>},{<ab>},{<ab>},{<ab>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
    {{<!!>},{<!!>},{<!!>},{<!!>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
    {{<a!>},{<a!>},{<a!>},{<ab>}}, score of 1 + 2 = 3.

What is the total score for all groups in your input?

*/

use std::io::Read;
use std::fs::File;

fn main() {
    let mut input = File::open("inputs/nine.txt").unwrap();
    let mut s = String::new();

    input.read_to_string(&mut s).unwrap();

    println!("a: {}", solve_a(&s));
    println!("b: {}", solve_b(&s));
}

fn solve_a(input: &str) -> u64 {
    solve(input).0
}

fn solve(input: &str) -> (u64, u64) {
    let mut in_garbage = false;
    let mut ignore_next = false;
    let mut depth = 0;
    let mut score = 0;
    let mut removed_garbage = 0;

    for c in input.chars() {
        if in_garbage {
            if ignore_next {
                ignore_next = false;
            } else {
                match c {
                    '>' => in_garbage = false,
                    '!' => ignore_next = true,
                    _ => removed_garbage += 1,
                }
            }
        } else {
            match c {
                '<' => in_garbage = true,
                '{' => depth += 1,
                '}' if depth > 0 => {
                    score += depth;
                    depth -= 1;
                }
                _ => {}
            }
        }
    }

    (score, removed_garbage)
}

/*
--- Part Two ---

Now, you're ready to remove the garbage.

To prove you've removed it, you need to count all of the characters within the garbage. The leading and trailing < and > don't count, nor do any canceled characters or the ! doing the canceling.

    <>, 0 characters.
    <random characters>, 17 characters.
    <<<<>, 3 characters.
    <{!>}>, 2 characters.
    <!!>, 0 characters.
    <!!!>>, 0 characters.
    <{o"i!a,<{i<a>, 10 characters.

How many non-canceled characters are within the garbage in your puzzle input?
*/

fn solve_b(input: &str) -> u64 {
    solve(input).1
}

#[test]
fn test_a() {
    assert_eq!(solve_a("{}"), 1);
    assert_eq!(solve_a("{{{}}}"), 6);
    assert_eq!(solve_a("{{},{}}"), 5);
    assert_eq!(solve_a("{{{},{},{{}}}}"), 16);
    assert_eq!(solve_a("{<{},{},{{}}>}"), 1);
    assert_eq!(solve_a("{<a>,<a>,<a>,<a>}"), 1);
    assert_eq!(solve_a("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
    assert_eq!(solve_a("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
    assert_eq!(solve_a("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
}

#[test]
fn test_b() {
    assert_eq!(solve_b("<>"), 0);
    assert_eq!(solve_b("<random characters>"), 17);
    assert_eq!(solve_b("<<<<>"), 3);
    assert_eq!(solve_b("<{!>}>"), 2);
    assert_eq!(solve_b("<!!>"), 0);
    assert_eq!(solve_b("<!!!>>"), 0);
    assert_eq!(solve_b(r#"<{o"i!a,<{i<a>"#), 10);
}