/*
--- Day 25: The Halting Problem ---

Following the twisty passageways deeper and deeper into the CPU, you finally reach the core of the computer. Here, in the expansive central chamber, you find a grand apparatus that fills the entire room, suspended nanometers above your head.

You had always imagined CPUs to be noisy, chaotic places, bustling with activity. Instead, the room is quiet, motionless, and dark.

Suddenly, you and the CPU's garbage collector startle each other. "It's not often we get many visitors here!", he says. You inquire about the stopped machinery.

"It stopped milliseconds ago; not sure why. I'm a garbage collector, not a doctor." You ask what the machine is for.

"Programs these days, don't know their origins. That's the Turing machine! It's what makes the whole computer work." You try to explain that Turing machines are merely models of computation, but he cuts you off. "No, see, that's just what they want you to think. Ultimately, inside every CPU, there's a Turing machine driving the whole thing! Too bad this one's broken. We're doomed!"

You ask how you can help. "Well, unfortunately, the only way to get the computer running again would be to create a whole new Turing machine from scratch, but there's no way you can-" He notices the look on your face, gives you a curious glance, shrugs, and goes back to sweeping the floor.

You find the Turing machine blueprints (your puzzle input) on a tablet in a nearby pile of debris. Looking back up at the broken Turing machine above, you can start to identify its parts:

    A tape which contains 0 repeated infinitely to the left and right.
    A cursor, which can move left or right along the tape and read or write values at its current position.
    A set of states, each containing rules about what to do based on the current value under the cursor.

Each slot on the tape has two possible values: 0 (the starting value for all slots) and 1. Based on whether the cursor is pointing at a 0 or a 1, the current state says what value to write at the current position of the cursor, whether to move the cursor left or right one slot, and which state to use next.

For example, suppose you found the following blueprint:

Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.

Running it until the number of steps required to take the listed diagnostic checksum would result in the following tape configurations (with the cursor marked in square brackets):

... 0  0  0 [0] 0  0 ... (before any steps; about to run state A)
... 0  0  0  1 [0] 0 ... (after 1 step;     about to run state B)
... 0  0  0 [1] 1  0 ... (after 2 steps;    about to run state A)
... 0  0 [0] 0  1  0 ... (after 3 steps;    about to run state B)
... 0 [0] 1  0  1  0 ... (after 4 steps;    about to run state A)
... 0  1 [1] 0  1  0 ... (after 5 steps;    about to run state B)
... 0  1  1 [0] 1  0 ... (after 6 steps;    about to run state A)

The CPU can confirm that the Turing machine is working by taking a diagnostic checksum after a specific number of steps (given in the blueprint). Once the specified number of steps have been executed, the Turing machine should pause; once it does, count the number of times 1 appears on the tape. In the above example, the diagnostic checksum is 3.

Recreate the Turing machine and save the computer! What is the diagnostic checksum it produces once it's working again?

*/

#[macro_use]
extern crate text_io;

use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let mut input = File::open("inputs/twenty_five.txt").unwrap();
    let mut s = String::new();

    input.read_to_string(&mut s).unwrap();

    println!("a: {}", solve_a(&s, 6));
    println!("b: {}", solve_b(&s));
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    false_transition: Transition,
    true_transition: Transition,
}

#[derive(Debug, PartialEq, Eq)]
struct Transition {
    write_value: usize,
    move_left: bool,
    next_state: char,
}

#[allow(unused_imports)]
fn parse_blueprint(input: &str, num_states: usize) -> (char, usize, HashMap<char, State>) {
    let mut input = input.bytes();
    let starting_state: char = read!("Begin in state {}.\n", &mut input);
    let diagnostic_steps: usize = read!("Perform a diagnostic checksum after {} steps.\n", &mut input);

    let parse_transition = |mut input: &mut std::str::Bytes| {
        let current_value: usize = read!("  If the current value is {}:\n", input);
        let write_value = read!("    - Write the value {}.\n", input);
        let direction: String = read!("    - Move one slot to the {}.\n", input);
        let next_state = read!("    - Continue with state {}.\n", input);

        let move_left = direction == "left";

        (current_value, Transition { write_value, move_left, next_state })
    };

    let mut states = HashMap::new();

    for _ in 0..num_states {
        let state_name: char = read!("\nIn state {}:\n", &mut input);
        let (false_value, false_transition) = parse_transition(&mut input);
        assert_eq!(false_value, 0);

        let (true_value, true_transition) = parse_transition(&mut input);
        assert_eq!(true_value, 1);

        let state = State { false_transition, true_transition };
        states.insert(state_name, state);
    }

    (starting_state, diagnostic_steps, states)
}

fn solve_a(input: &str, num_states: usize) -> usize {
    let (starting_state, diagnostic_steps, states) = parse_blueprint(input, num_states);

    let mut x = 0;
    let mut tape = HashMap::new();
    let mut current_state = &states[&starting_state];

    for _ in 0..diagnostic_steps {
        let current_slot = tape.entry(x).or_insert(false);
        let transition = if *current_slot {
            &current_state.true_transition
        } else {
            &current_state.false_transition
        };

        *current_slot = transition.write_value == 1;
        if transition.move_left { x -= 1 } else { x += 1 }
        current_state = &states[&transition.next_state];
    }

    tape.values().filter(|&&v| v).count()
}

/*
--- Part Two ---

The Turing machine, and soon the entire computer, springs back to life. A console glows dimly nearby, awaiting your command.

> reboot printer
Error: That command requires priority 50. You currently have priority 0.
You must deposit 50 stars to increase your priority to the required level.

The console flickers for a moment, and then prints another message:

Star accepted.
You must deposit 49 stars to increase your priority to the required level.

The garbage collector winks at you, then continues sweeping.

-----

You deposit all fifty stars and reboot the printer. Suddenly, everything seems a lot less pixelated than before.

"--raise your priority level enough to send the reboot command and... hey look, it's printing! I'll bring it to Santa. Thanks!" She runs off.

Congratulations! You've finished every puzzle in Advent of Code 2017! I hope you had as much fun solving them as I had making them for you. I'd love to hear about your adventure; you can get in touch with me via contact info on my website or through Twitter.

If you'd like to see more things like this in the future, please consider supporting Advent of Code and sharing it with others.

To hear about future projects, you can follow me on Twitter.

I've highlighted the easter eggs in each puzzle, just in case you missed any. Hover your mouse over them, and the easter egg will appear.
*/

fn solve_b(_: &str) -> &'static str {
    "FINISHED!"
}

#[test]
fn test_a() {
    assert_eq!(
        solve_a(
"Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
",
        2
        ),
        3
    );
}