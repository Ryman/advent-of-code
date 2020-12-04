/*
--- Day 23: Coprocessor Conflagration ---

You decide to head directly to the CPU and fix the printer from there. As you get close, you find an experimental coprocessor doing so much work that the local programs are afraid it will halt and catch fire. This would cause serious issues for the rest of the computer, so you head in and see what you can do.

The code it's running seems to be a variant of the kind you saw recently on that tablet. The general functionality seems very similar, but some of the instructions are different:

    set X Y sets register X to the value of Y.
    sub X Y decreases register X by the value of Y.
    mul X Y sets register X to the result of multiplying the value contained in register X by the value of Y.
    jnz X Y jumps with an offset of the value of Y, but only if the value of X is not zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)

    Only the instructions listed above are used. The eight registers here, named a through h, all start at 0.

The coprocessor is currently set to some kind of debug mode, which allows for testing, but prevents it from doing any meaningful work.

If you run the program (your puzzle input), how many times is the mul instruction invoked?
*/

use std::io::Read;
use std::fs::File;

fn main() {
    let mut input = File::open("inputs/twenty_three.txt").unwrap();
    let mut s = String::new();

    input.read_to_string(&mut s).unwrap();

    println!("a: {}", solve(&s));
    println!("b: {}", optimized());
}

fn solve(input: &str) -> usize {
    let mut program_counter = 0isize;
    let mut registers = vec![0; 26];
    let mut multiplies = 0;

    let reg_key = |key: &str| { (key.as_bytes()[0] - b'a') as usize };

    macro_rules! val {
        ($key:ident) => {{
            if !$key.chars().all(|c| c.is_alphabetic()) {
                $key.parse::<isize>().unwrap()
            } else {
                registers[reg_key($key)]
            }
        }}
    }

    let program = input.lines().map(|s| {
        let mut split = s.split(" ");
        (split.next().unwrap(), split.next().unwrap(), split.next())
    }).collect::<Vec<_>>();

    while program_counter >= 0 && (program_counter as usize )< program.len() {
        let (op, x, y) = program[program_counter as usize].clone();
        let y = y.map(|s| val!(s));
        match (op, x, y) {
            // set X Y sets register X to the value of Y.
            ("set", x, y) => registers[reg_key(x)] = y.unwrap(),
            // sub X Y decreases register X by the value of Y.
            ("sub", x, y) => registers[reg_key(x)] -= y.unwrap(),
            // mul X Y sets register X to the result of multiplying the value contained in register X by the value of Y.
            ("mul", x, y) => {
                multiplies += 1;
                registers[reg_key(x)] *= y.unwrap()
            },
            // jnz X Y jumps with an offset of the value of Y, but only if the value of X is not zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)
            ("jnz", x, y) => if val!(x) != 0 {
                program_counter += y.unwrap();
                continue
            },
            _ => unreachable!()
        }

        program_counter += 1;
    }

    multiplies
}

/*
    --- Part Two ---

Now, it's time to fix the problem.

The debug mode switch is wired directly to register a. You flip the switch, which makes register a now start at 1 when the program is executed.

Immediately, the coprocessor begins to overheat. Whoever wrote this program obviously didn't choose a very efficient implementation. You'll need to optimize the program if it has any hope of completing before Santa needs that printer working.

The coprocessor's ultimate goal is to determine the final value left in register h once the program completes. Technically, if it had that... it wouldn't even need to run the program.

After setting register a to 1, if the program were to run to completion, what value would be left in register h?

*/

#[allow(dead_code)]
fn translated() -> isize {
    let mut b = 81;
    let mut f;
    let mut g;
    let mut h = 0;

    b *= 100;
    b -= -100000;
    let c = b - -17000;
    loop {
        f = 1;
        // d = 2;
        for d in 2..b+1 {
            // println!("{}", d);
            // loop {
                for e in 2..b+1 {
                // e = 2;
                // loop {
                    g = d;
                    g *= e;
                    g -= b;
                    if g == 0 {
                        f = 0;
                    }
                    // e -= -1;
                    // g = e;
                    // g -= b;
                    // if g == 0 {
                        // break
                    // }
                }
                // d -= -1;
                // g = d;
                // g -= b;
                // if g == 0 {
                //     break
                // }
            // }
        }
        if f == 0 {
            h -= -1;
        }
        g = b;
        g -= c;
        if g == 0 {
            return h;
        }
        println!("{:?}", (b, c, f, g, h));
        b -= -17;
    }
}

fn optimized() -> isize {
    let mut h = 0;

    let mut b = 81 * 100 + 100_000;
    let c = b + 17000;

    loop {
        // Check if prime!
        let f = (2..b+1).any(|d| {
            let x = b / d;
            b % x == 0 && x < b && x > 1
            // (2..b+1).any(move |e| {
            //     d * e == b
            // })
        });

        if f == true {
            h += 1;
        }

        // println!("{:?}", (b, c, f, h));

        if b == c {
            return h
        }
        b += 17;
    }
}

