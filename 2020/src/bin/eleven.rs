/*
--- Day 11: Seating System ---

Your plane lands with plenty of time to spare. The final leg of your journey is a ferry that goes directly to the tropical island where you can finally start your vacation. As you reach the waiting area to board the ferry, you realize you're so early, nobody else has even arrived yet!

By modeling the process people use to choose (or abandon) their seat in the waiting area, you're pretty sure you can predict the best place to sit. You make a quick map of the seat layout (your puzzle input).

The seat layout fits neatly on a grid. Each position is either floor (.), an empty seat (L), or an occupied seat (#). For example, the initial seat layout might look like this:

L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL

Now, you just need to model the people who will be arriving shortly. Fortunately, people are entirely predictable and always follow a simple set of rules. All decisions are based on the number of occupied seats adjacent to a given seat (one of the eight positions immediately up, down, left, right, or diagonal from the seat). The following rules are applied to every seat simultaneously:

    If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
    If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
    Otherwise, the seat's state does not change.

Floor (.) never changes; seats don't move, and nobody sits on the floor.

After one round of these rules, every seat in the example layout becomes occupied:

#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##

After a second round, the seats with four or more occupied adjacent seats become empty again:

#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##

This process continues for three more rounds:

#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##

#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##

#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##

At this point, something interesting happens: the chaos stabilizes and further applications of these rules cause no seats to change state! Once people stop moving around, you count 37 occupied seats.

Simulate your seating area by applying the seating rules repeatedly until no seats change state. How many seats end up occupied?
*/
use std::io::Read;
use std::fs::File;

fn main() {
    let mut input = File::open("inputs/eleven.txt").unwrap();
    let mut s = String::new();

    input.read_to_string(&mut s).unwrap();
    println!("a: {}", solve_a(&s));
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum TileType {
    Floor,
    Seat(bool)
}

type PlaneState = Vec<Vec<TileType>>;

fn layout(input: &str) -> PlaneState {
    input.lines().map(|s| {
        s.trim().chars().map(|c| {
            match c {
                '.' => TileType::Floor,
                'L' => TileType::Seat(false),
                '#' => TileType::Seat(true),
                _ => unreachable!("Encountered unknown character {}", c),
            }
        }).collect()
    }).collect()
}

fn simulate(state: &PlaneState) -> PlaneState {
    let mut next = state.clone();

    for x in 0..state.len() as isize {
        for y in 0..state[x as usize].len() as isize{
            let occupied = vec![
                (x - 1, y - 1),
                (x - 1, y),
                (x - 1, y + 1),
                (x, y - 1),
                (x, y + 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x + 1, y + 1)
            ].into_iter().map(|(x, y)| {
                state.get(x as usize).and_then(|row| row.get(y as usize))
            }).filter(|state| *state == Some(&TileType::Seat(true)))
              .count();

            next[x as usize][y as usize] = match state[x as usize][y as usize] {
                TileType::Seat(false) if occupied == 0 => TileType::Seat(true),
                TileType::Seat(true) if occupied >= 4 => TileType::Seat(false),
                current => current
            };
        }
    }

    next
}

fn solve_a(input: &str) -> usize {
    let mut current = layout(input);

    loop {
        let next = simulate(&current);

        if next == current {
            return current.iter().map(|row| {
                row.iter().filter(|&&seat| seat == TileType::Seat(true)).count()
            }).sum()
        }

        current = next;
    }
}

#[test]
fn test_simulate() {
    println!("one");
    assert_eq!(
        simulate(
            &layout(
                "L.LL.LL.LL
                LLLLLLL.LL
                L.L.L..L..
                LLLL.LL.LL
                L.LL.LL.LL
                L.LLLLL.LL
                ..L.L.....
                LLLLLLLLLL
                L.LLLLLL.L
                L.LLLLL.LL"
            )
        ),
        layout(
            "#.##.##.##
            #######.##
            #.#.#..#..
            ####.##.##
            #.##.##.##
            #.#####.##
            ..#.#.....
            ##########
            #.######.#
            #.#####.##"
        )
    );

    println!("two");
    assert_eq!(
        simulate(
            &layout(
                "#.##.##.##
                #######.##
                #.#.#..#..
                ####.##.##
                #.##.##.##
                #.#####.##
                ..#.#.....
                ##########
                #.######.#
                #.#####.##"
            )
        ),
        layout(
            "#.LL.L#.##
            #LLLLLL.L#
            L.L.L..L..
            #LLL.LL.L#
            #.LL.LL.LL
            #.LLLL#.##
            ..L.L.....
            #LLLLLLLL#
            #.LLLLLL.L
            #.#LLLL.##"
        )
    );

    println!("three");
    assert_eq!(
        simulate(
            &layout(
                "#.LL.L#.##
                #LLLLLL.L#
                L.L.L..L..
                #LLL.LL.L#
                #.LL.LL.LL
                #.LLLL#.##
                ..L.L.....
                #LLLLLLLL#
                #.LLLLLL.L
                #.#LLLL.##"
            )
        ),
        layout(
            "#.##.L#.##
            #L###LL.L#
            L.#.#..#..
            #L##.##.L#
            #.##.LL.LL
            #.###L#.##
            ..#.#.....
            #L######L#
            #.LL###L.L
            #.#L###.##"
        )
    );

    println!("four");
    assert_eq!(
        simulate(
            &layout(
                "#.##.L#.##
                #L###LL.L#
                L.#.#..#..
                #L##.##.L#
                #.##.LL.LL
                #.###L#.##
                ..#.#.....
                #L######L#
                #.LL###L.L
                #.#L###.##"
            )
        ),
        layout(
            "#.#L.L#.##
            #LLL#LL.L#
            L.L.L..#..
            #LLL.##.L#
            #.LL.LL.LL
            #.LL#L#.##
            ..L.L.....
            #L#LLLL#L#
            #.LLLLLL.L
            #.#L#L#.##"
        )
    );
}

#[test]
fn smoke_a() {
    assert_eq!(solve_a("L.LL.LL.LL
                        LLLLLLL.LL
                        L.L.L..L..
                        LLLL.LL.LL
                        L.LL.LL.LL
                        L.LLLLL.LL
                        ..L.L.....
                        LLLLLLLLLL
                        L.LLLLLL.L
                        L.LLLLL.LL"), 37);
}
