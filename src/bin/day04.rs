use std::collections::HashSet;

aoc2025::main!("../../assets/day04.txt");

const DIRS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
];

fn part1(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut positions = HashSet::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let c = grid[y][x];
            if c != '@' {
                continue;
            }

            let mut rolls = 0;
            for (dx, dy) in DIRS {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx < 0 || ny < 0 || nx >= grid[0].len() as isize || ny >= grid.len() as isize {
                    continue;
                }

                if grid[ny as usize][nx as usize] == '@' {
                    rolls += 1;
                }
            }

            if rolls < 4 {
                positions.insert((x, y));
            }
        }
    }

    positions.len() as u32
}

fn part2(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut removed = HashSet::new();

    loop {
        let mut positions = HashSet::new();
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                let c = grid[y][x];
                if c != '@' || removed.contains(&(x, y)) {
                    continue;
                }

                let mut rolls = 0;
                for (dx, dy) in DIRS {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;

                    if nx < 0 || ny < 0 || nx >= grid[0].len() as isize || ny >= grid.len() as isize
                    {
                        continue;
                    }

                    if grid[ny as usize][nx as usize] == '@'
                        && !removed.contains(&(nx as usize, ny as usize))
                    {
                        rolls += 1;
                    }
                }

                if rolls < 4 {
                    positions.insert((x, y));
                }
            }
        }

        if positions.is_empty() {
            break;
        }

        for pos in positions {
            removed.insert(pos);
        }
    }

    removed.len() as u32
}

aoc2025::test!(
    "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
",
    13,
    43
);
