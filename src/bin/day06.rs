aoc2025::main!("../../assets/day06.txt");

fn part1(input: &str) -> u64 {
    let lines = input
        .lines()
        .map(|l| l.split_ascii_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut total_result = 0;
    for col in 0..lines[0].len() {
        let mut numbers = Vec::with_capacity(lines.len() - 1);
        for row in 0..lines.len() {
            let s = lines[row][col];

            // Operator is on last row
            if row == lines.len() - 1 {
                match s {
                    "*" => total_result += numbers.iter().product::<u64>(),
                    "+" => total_result += numbers.iter().sum::<u64>(),
                    _ => unreachable!(),
                }
                break;
            }

            let num = s.parse::<u64>().unwrap();
            numbers.push(num);
        }
    }

    total_result
}

fn part2(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<_>>();
    let operators = lines.last().unwrap();

    let mut total_result = 0;

    let mut ops = operators.char_indices();
    let mut cur = ops.next().unwrap();

    for o @ (i, op) in ops {
        let (start_i, cur_op) = cur;

        // Process previous number when next operator is found
        if op != ' ' || i == operators.len() - 1 {
            // Offset for the last number/space between two numbers
            let offset = if i == operators.len() - 1 { 1 } else { -1 };

            let mut total = 0;
            for i in start_i..(i as isize + offset) as usize {
                let num = lines[..lines.len() - 1]
                    .iter()
                    .filter_map(|l| (l.as_bytes()[i] as char).to_digit(10))
                    .fold(0u64, |mut num, d| {
                        num *= 10;
                        num + d as u64
                    });

                match cur_op {
                    '*' => total = if total == 0 { num } else { total * num },
                    '+' => total += num,
                    _ => unreachable!(),
                }
            }

            total_result += total;
            cur = o;
        }
    }

    total_result
}

aoc2025::test!(
    "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
",
    4277556,
    3263827
);
