use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

aoc2025::main!("../../assets/day07.txt");

fn part1(input: &str) -> u32 {
    let mut lines = input.lines().map(|l| l.as_bytes());

    let mut beams = HashSet::new();
    let start_x = lines.next().and_then(|l| l.iter().position(|b| b == &b'S'));
    beams.insert((start_x.unwrap(), 0));

    lines.fold(0, |mut acc, l| {
        let mut new_beams = HashSet::new();
        for &(x, y) in &beams {
            match l[x] {
                b'S' | b'.' => {
                    new_beams.insert((x, y + 1));
                }
                b'^' => {
                    new_beams.insert((x - 1, y + 1));
                    new_beams.insert((x + 1, y + 1));
                    acc += 1;
                }
                _ => unreachable!(),
            }
        }
        beams = new_beams;
        acc
    })
}

fn part2(input: &str) -> u64 {
    let lines = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let start_x = lines[0].iter().position(|&b| b == b'S').unwrap();

    CACHE.with(|cache| cache.borrow_mut().clear());
    1 + recurse_beam(start_x, 0, &lines)
}

thread_local! {
    static CACHE: RefCell<HashMap<(usize, usize), u64>> = RefCell::new(HashMap::new());
}

fn recurse_beam(x: usize, y: usize, input: &Vec<&[u8]>) -> u64 {
    if y >= input.len() {
        return 0;
    }

    if let Some(e) = CACHE.with(|cache| cache.borrow().get(&(x, y)).cloned()) {
        return e;
    }

    let total = match input[y][x] {
        b'S' | b'.' => recurse_beam(x, y + 1, input),
        b'^' => 1 + recurse_beam(x - 1, y + 1, input) + recurse_beam(x + 1, y + 1, input),
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
