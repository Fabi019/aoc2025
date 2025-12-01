aoc2025::main!("../../assets/day01.txt");

fn part1(input: &str) -> u32 {
    let amounts = input.lines().map(|l| {
        let dist: i32 = l[1..].parse().unwrap();
        dist * if &l[0..1] == "L" { -1 } else { 1 }
    });

    amounts
        .fold((50, 0), |(mut current, zeros), amount| {
            current += amount;
            current %= 100;

            if current < 0 {
                current += 100;
            }

            (current, if current == 0 { zeros + 1 } else { zeros })
        })
        .1
}

fn part2(input: &str) -> i32 {
    let amounts = input.lines().map(|l| {
        let dist: i32 = l[1..].parse().unwrap();
        dist * if &l[0..1] == "L" { -1 } else { 1 }
    });

    let mut start = 50;
    let mut zeros = 0;

    for amount in amounts {
        let was_zero = start == 0;
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
            zeros += start / 100;
            start %= 100;
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
