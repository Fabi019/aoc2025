aoc2025::main!("../../assets/day01.txt");

fn part1(input: &str) -> u32 {
    let lines = input.lines();

    let mut start = 50;
    let mut zeros = 0;

    for line in lines {
        let dist: i32 = line[1..].parse().unwrap();
        let dir = if &line[0..1] == "L" { -1 } else { 1 };

        start += dist * dir;
        start %= 100;

        if start < 0 {
            start += 100;
        }

        if start == 0 {
            zeros += 1;
        }
    }
    zeros
}

fn part2(input: &str) -> u32 {
    let lines = input.lines();

    let mut start = 50;
    let mut zeros = 0;

    for line in lines {
        let dist: i32 = line[1..].parse().unwrap();
        let dir = if &line[0..1] == "L" { -1 } else { 1 };

        let was_zero = start == 0;
        let amount = dist * dir;
        start += amount;

        if start == 0 {
            zeros += 1;
        } else if amount < 0 {
            if start < 0 && was_zero {
                zeros -= 1; // don't count first transition
            }

            while start < 0 {
                start += 100;
                zeros += 1;
            }

            if start == 0 {
                zeros += 1;
            }
        } else {
            while start >= 100 {
                start -= 100;
                zeros += 1;
            }
        }
    }
    zeros
}

aoc2025::test!(
    "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
",
    3,
    6
);
