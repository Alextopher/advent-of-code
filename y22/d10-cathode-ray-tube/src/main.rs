use aoc::*;

enum Instruction {
    Noop(),
    AddX(i32),
}

fn part1(input: &str) -> i32 {
    let lines = get_lines(input);

    // each line is either "noop" or it's "addx num" where num can be negative
    let mut instructions = vec![];

    for line in lines {
        let mut parts = line.split_whitespace();
        let instruction = parts.next().unwrap();

        if instruction == "noop" {
            instructions.push(Instruction::Noop());
        } else {
            let num = parts.next().unwrap().parse::<i32>().unwrap();
            instructions.push(Instruction::Noop());
            instructions.push(Instruction::AddX(num));
        }
    }

    // 20th, 60th, 100th, 140th, 180th, and 220th cycle
    let mut cycle = 1;
    let mut acc = 1;
    let mut index = 0;

    let mut score = 0;

    loop {
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            score += acc * cycle;
        }

        if index == instructions.len() {
            break;
        }

        let instruction = &instructions[index];

        match instruction {
            Instruction::Noop() => {
                index += 1;
            }
            Instruction::AddX(num) => {
                acc += num;
                index += 1;
            }
        }

        cycle += 1;
    }

    score
}

fn part2(input: &str) {
    let lines = get_lines(input);

    // each line is either "noop" or it's "addx num" where num can be negative
    let mut instructions = vec![];

    for line in lines {
        let mut parts = line.split_whitespace();
        let instruction = parts.next().unwrap();

        if instruction == "noop" {
            instructions.push(Instruction::Noop());
        } else {
            let num = parts.next().unwrap().parse::<i32>().unwrap();
            instructions.push(Instruction::Noop());
            instructions.push(Instruction::AddX(num));
        }
    }

    let mut acc = 1;
    let mut cycle = 0;
    let mut index = 0;

    loop {
        let instruction = &instructions[index % instructions.len()];

        let width: i32 = cycle % 40;
        let height: i32 = cycle / 40;

        if width == 0 {
            println!();
        }

        if height == 6 {
            break;
        }

        // if cycle is within 1 of acc print "#" else print "."
        if (acc - width).abs() <= 1 {
            print!("#");
        } else {
            print!(" ");
        }

        if let Instruction::AddX(num) = instruction {
            acc += num;
        }

        cycle += 1;
        index += 1;
    }
}

fn main() {
    let input = input_str!(2022, 10);
    println!("{}", part1(input));
    part2(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example.txt")), 13140);
        assert_eq!(part1(input_str!(2022, 10)), 14360);
    }
}
