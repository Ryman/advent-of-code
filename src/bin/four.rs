/*
--- Day 4: High-Entropy Passphrases ---

A new system policy has been put in place that requires all accounts to use a passphrase instead of simply a password. A passphrase consists of a series of words (lowercase letters) separated by spaces.

To ensure security, a valid passphrase must contain no duplicate words.

For example:

    aa bb cc dd ee is valid.
    aa bb cc dd aa is not valid - the word aa appears more than once.
    aa bb cc dd aaa is valid - aa and aaa count as different words.

The system's full passphrase list is available as your puzzle input. How many passphrases are valid?
*/
use std::io::Read;
use std::fs::File;
use std::collections::HashSet;

fn main() {
    let mut input = File::open("inputs/four.txt").unwrap();
    let mut s = String::new();

    input.read_to_string(&mut s).unwrap();

    println!("a: {}", solve_a(&s));
    println!("b: {}", solve_b(&s));
}

fn solve_a(input: &str) -> usize {
    input.lines().filter(|s| valid_a(s)).count()
}

fn solve_b(input: &str) -> usize {
    input.lines().filter(|s| valid_b(s)).count()
}

fn valid_a(input: &str) -> bool {
    let mut seen = HashSet::new();
    for word in input.split(' ') {
        if seen.contains(&word) {
            return false
        }
        seen.insert(word);
    }

    true
}

/*
--- Part Two ---

For added security, yet another system policy has been put in place. Now, a valid passphrase must contain no two words that are anagrams of each other - that is, a passphrase is invalid if any word's letters can be rearranged to form any other word in the passphrase.

For example:

    abcde fghij is a valid passphrase.
    abcde xyz ecdab is not valid - the letters from the third word can be rearranged to form the first word.
    a ab abc abd abf abj is a valid passphrase, because all letters need to be used when forming another word.
    iiii oiii ooii oooi oooo is valid.
    oiii ioii iioi iiio is not valid - any of these words can be rearranged to form any other word.

Under this new system policy, how many passphrases are valid?
*/

fn valid_b(input: &str) -> bool {
    let mut seen = HashSet::new();
    for word in input.split(' ') {
        let mut word = word.chars().collect::<Vec<_>>();
        word.sort();

        if seen.contains(&word) {
            return false
        }

        seen.insert(word);
    }

    true
}

#[test]
fn test_a() {
    assert_eq!(valid_a("aa bb cc dd ee"), true);
    assert_eq!(valid_a("aa bb cc dd aa"), false);
    assert_eq!(valid_a("aa bb cc dd aaa"), true);
}

#[test]
fn test_b() {
    assert_eq!(valid_b("abcde fghij"), true);
    assert_eq!(valid_b("abcde xyz ecdab"), false);
    assert_eq!(valid_b("a ab abc abd abf abj"), true);
    assert_eq!(valid_b("iiii oiii ooii oooi oooo"), true);
    assert_eq!(valid_b("oiii ioii iioi iiio"), false);
}