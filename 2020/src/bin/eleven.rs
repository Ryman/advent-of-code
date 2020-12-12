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
    println!("b: {}", solve_b(&s));
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

fn simulate_a(state: &PlaneState) -> PlaneState {
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
        let next = simulate_a(&current);

        if next == current {
            return current.iter().map(|row| {
                row.iter().filter(|&&seat| seat == TileType::Seat(true)).count()
            }).sum()
        }

        current = next;
    }
}

/*
--- Part Two ---

As soon as people start to arrive, you realize your mistake. People don't just care about adjacent seats - they care about the first seat they can see in each of those eight directions!

Now, instead of considering just the eight immediately adjacent seats, consider the first seat in each of those eight directions. For example, the empty seat below would see eight occupied seats:

.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....

The leftmost empty seat below would only see one empty seat, but cannot see any of the occupied ones:

.............
.L.L.#.#.#.#.
.............

The empty seat below would see no occupied seats:

.##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.

Also, people seem to be more tolerant than you expected: it now takes five or more visible occupied seats for an occupied seat to become empty (rather than four or more from the previous rules). The other rules still apply: empty seats that see no occupied seats become occupied, seats matching no rule don't change, and floor never changes.

Given the same starting layout as above, these new rules cause the seating area to shift around as follows:

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

#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#

#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#

#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#

#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#

#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#

Again, at this point, people stop shifting around and the seating area reaches equilibrium. Once this occurs, you count 26 occupied seats.

Given the new visibility method and the rule change for occupied seats becoming empty, once equilibrium is reached, how many seats end up occupied?
*/

fn simulate_b(state: &PlaneState) -> PlaneState {
    let mut next = state.clone();

    fn find_next_seat(map: &PlaneState, (mut x, mut y): (isize, isize), (dx, dy): (isize, isize)) -> Option<TileType> {
        loop {
            x += dx;
            y += dy;

            match map.get(x as usize).and_then(|row| row.get(y as usize)) {
                Some(seat @ TileType::Seat(_)) => return Some(*seat),
                None => return None,
                Some(_) => continue,
            }
        }
    }

    for x in 0..state.len() as isize {
        for y in 0..state[x as usize].len() as isize{
            let occupied = vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1)
            ].into_iter()
            .map(|(dx, dy)| find_next_seat(state, (x, y), (dx, dy)))
            .filter(|state| *state == Some(TileType::Seat(true)))
            .count();

            next[x as usize][y as usize] = match state[x as usize][y as usize] {
                TileType::Seat(false) if occupied == 0 => TileType::Seat(true),
                TileType::Seat(true) if occupied >= 5 => TileType::Seat(false),
                current => current
            };
        }
    }

    next
}

fn solve_b(input: &str) -> usize {
    let mut current = layout(input);

    loop {
        let next = simulate_b(&current);

        if next == current {
            return current.iter().map(|row| {
                row.iter().filter(|&&seat| seat == TileType::Seat(true)).count()
            }).sum()
        }

        current = next;
    }
}

#[test]
fn test_simulate_a() {
    println!("one");
    assert_eq!(
        simulate_a(
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
        simulate_a(
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
        simulate_a(
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
        simulate_a(
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

#[test]
fn test_simulate_b() {
    println!("one");
    assert_eq!(
        simulate_b(
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
        simulate_b(
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
            "#.LL.LL.L#
            #LLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLL#
            #.LLLLLL.L
            #.LLLLL.L#"
        )
    );

    println!("three");
    assert_eq!(
        simulate_b(
            &layout(
                "#.LL.LL.L#
                #LLLLLL.LL
                L.L.L..L..
                LLLL.LL.LL
                L.LL.LL.LL
                L.LLLLL.LL
                ..L.L.....
                LLLLLLLLL#
                #.LLLLLL.L
                #.LLLLL.L#"
            )
        ),
        layout(
            "#.L#.##.L#
            #L#####.LL
            L.#.#..#..
            ##L#.##.##
            #.##.#L.##
            #.#####.#L
            ..#.#.....
            LLL####LL#
            #.L#####.L
            #.L####.L#"
        )
    );

    println!("four");
    assert_eq!(
        simulate_b(
            &layout(
                "#.L#.##.L#
                #L#####.LL
                L.#.#..#..
                ##L#.##.##
                #.##.#L.##
                #.#####.#L
                ..#.#.....
                LLL####LL#
                #.L#####.L
                #.L####.L#"
            )
        ),
        layout(
            "#.L#.L#.L#
            #LLLLLL.LL
            L.L.L..#..
            ##LL.LL.L#
            L.LL.LL.L#
            #.LLLLL.LL
            ..L.L.....
            LLLLLLLLL#
            #.LLLLL#.L
            #.L#LL#.L#"
        )
    );
}

#[test]
fn smoke_b() {
    assert_eq!(solve_b("L.LL.LL.LL
                        LLLLLLL.LL
                        L.L.L..L..
                        LLLL.LL.LL
                        L.LL.LL.LL
                        L.LLLLL.LL
                        ..L.L.....
                        LLLLLLLLLL
                        L.LLLLLL.L
                        L.LLLLL.LL"), 26);
}