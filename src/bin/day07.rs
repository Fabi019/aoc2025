use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

aoc2025::main!("../../assets/day07.txt");

fn part1(input: &str) -> u32 {
    let lines = input.lines().map(|l| l.as_bytes());

    let mut splits = 0;
    let mut beams = HashSet::new();
    for (y, l) in lines.enumerate() {
        for (x, b) in l.iter().enumerate() {
            match b {
                b'S' => {
                    beams.insert((x, y + 1));
                }
                b'^' if beams.contains(&(x, y)) => {
                    beams.insert((x - 1, y + 1));
                    beams.insert((x + 1, y + 1));
                    splits += 1;
                }
                b'.' if beams.contains(&(x, y)) => {
                    beams.insert((x, y + 1));
                }
                _ => {}
            }
        }
    }

    splits
}

fn part2(input: &str) -> u64 {
    let lines = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let start_x = lines[0].iter().position(|&b| b == b'S').unwrap();

    let mut beams = HashSet::new();
    CACHE.with(|cache| cache.borrow_mut().clear());
    1 + recurse_beam(start_x, 0, &lines, &mut beams)
}

thread_local! {
    static CACHE: RefCell<HashMap<(usize, usize), u64>> = RefCell::new(HashMap::new());
}

fn recurse_beam(
    x: usize,
    y: usize,
    input: &Vec<&[u8]>,
    beams: &mut HashSet<(usize, usize)>,
) -> u64 {
    if y >= input.len() {
        return 0;
    }

    if let Some(e) = CACHE.with(|cache| cache.borrow().get(&(x, y)).cloned()) {
        return e;
    }

    let total = match input[y][x] {
        b'S' | b'.' => {
            beams.insert((x, y + 1));
            recurse_beam(x, y + 1, input, beams)
        }
        b'^' => {
            beams.insert((x - 1, y + 1));
            beams.insert((x + 1, y + 1));
            1 + recurse_beam(x - 1, y + 1, input, beams) + recurse_beam(x + 1, y + 1, input, beams)
        }
        _ => unreachable!(),
    };

    CACHE.with(|cache| {
        cache.borrow_mut().insert((x, y), total);
    });

    total
}

aoc2025::test!(
    "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
",
    21,
    40
);
