
aoc2025::main!("../../assets/day06.txt");

fn part1(input: &str) -> u64 {
    let lines = input.lines().map(|l| l.split_ascii_whitespace().collect::<Vec<_>>()).collect::<Vec<_>>();
    
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
                    _ => panic!("Unknown operator"),
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

    let mut start_idx = 0;
    let mut current_op = ' ';
    for (i, op) in operators.char_indices() {
        if op != ' ' && current_op == ' ' {
            current_op = op;
            start_idx = i;
        }

        if op != ' ' && start_idx != i || i == operators.len() - 1 {
            println!("Processing from {} to {} with operator {}", start_idx, i, current_op);

            // Fix for the last number
            let offset = if i == operators.len() - 1 { 1 } else { -1 };

            let mut nums = Vec::new();
            for i in (start_idx..(i as isize + offset) as usize).rev() {
                let mut num = String::new();
                for n in 0..lines.len() - 1 {
                    num += &lines[n].chars().nth(i).unwrap().to_string();
                }
                println!("Extracted number at {i}: {}", num);
                let num = num.trim().parse::<u64>().unwrap();
                nums.push(num);
            }

            match current_op {
                '*' => total_result += nums.iter().product::<u64>(),
                '+' => total_result += nums.iter().sum::<u64>(),
                _ => panic!("Unknown operator"),
            }

            start_idx = i;
            current_op = op;
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
", 4277556, 3263827
);
