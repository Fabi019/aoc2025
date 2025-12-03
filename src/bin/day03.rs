aoc2025::main!("../../assets/day03.txt");

fn part1(input: &str) -> u32 {
    let banks = input.lines().map(|l| {
        l.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>()
    });

    banks.fold(0, |acc, bank| {
        let (mut cur_max, mut index) = (0, 0);
        for (i, c) in bank[0..bank.len() - 1].iter().enumerate() {
            if *c > cur_max {
                cur_max = *c;
                index = i;
            }
        }
        let second = &bank[index + 1..bank.len()].iter().max();
        acc + cur_max * 10 + *second.unwrap()
    })
}

fn part2(input: &str) -> u64 {
    let banks = input.lines().map(|l| {
        l.chars()
            .map(|c: char| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<_>>()
    });

    banks.fold(0, |acc, bank| {
        let (mut sum, mut offset) = (0, 0);
        for i in (0..12).rev() {
            let (mut cur_max, mut index_max) = (0, 0);
            for (i, &c) in bank[offset..(bank.len() - i)].iter().enumerate() {
                if c > cur_max {
                    cur_max = c;
                    index_max = i;
                }
            }
            offset += index_max + 1;
            sum = sum * 10 + cur_max;
        }
        acc + sum
    })
}

aoc2025::test!(
    "\
987654321111111
811111111111119
234234234234278
818181911112111
",
    357,
    3121910778619
);
