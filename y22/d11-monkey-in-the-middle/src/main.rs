use aoc::{get_mut::GetMany, IterJunk};

#[derive(Debug, Clone, Copy)]
enum Op {
    Add(i64),
    Mul(i64),
    Square,
}

impl Op {
    fn apply<'a>(&self, n: &'a mut i64) -> &'a mut i64 {
        match self {
            Op::Add(x) => *n += x,
            Op::Mul(x) => *n *= x,
            Op::Square => *n *= *n,
        }
        n
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    pub items: Vec<i64>,
    pub operation: Op,
    pub divisible: i64,
    pub send: (usize, usize),
    pub inspections: usize,
}

fn run_round(monkeys: &mut [Monkey], worried: Option<i64>) {
    for index in 0..monkeys.len() {
        let (a, b) = monkeys[index].send;
        let (monkey, monkey_a, monkey_b) = monkeys.get_mut_3(index, a, b);

        // Update items worry values
        monkey.items.iter_mut().for_each(|i| {
            monkey.operation.apply(i);
            match worried {
                Some(lcm) => *i %= lcm,
                None => *i /= 3,
            }
        });

        // Send items to other monkeys, based on `divisible` check
        for item in monkey.items.drain(..) {
            monkey.inspections += 1;
            if item % monkey.divisible == 0 {
                monkey_a.items.push(item);
            } else {
                monkey_b.items.push(item);
            }
        }
    }
}

// I didn't parse the input, I just hardcoded it because that was easier using
// chat-gpt
fn get_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![89, 84, 88, 78, 70],
            operation: Op::Mul(5),
            divisible: 7,
            send: (6, 7),
            inspections: 0,
        },
        Monkey {
            items: vec![76, 62, 61, 54, 69, 60, 85],
            operation: Op::Add(1),
            divisible: 17,
            send: (0, 6),
            inspections: 0,
        },
        Monkey {
            items: vec![83, 89, 53],
            operation: Op::Add(8),
            divisible: 11,
            send: (5, 3),
            inspections: 0,
        },
        Monkey {
            items: vec![95, 94, 85, 57],
            operation: Op::Add(4),
            divisible: 13,
            send: (0, 1),
            inspections: 0,
        },
        Monkey {
            items: vec![82, 98],
            operation: Op::Add(7),
            divisible: 19,
            send: (5, 2),
            inspections: 0,
        },
        Monkey {
            items: vec![69],
            operation: Op::Add(2),
            divisible: 2,
            send: (1, 3),
            inspections: 0,
        },
        Monkey {
            items: vec![82, 70, 58, 87, 59, 99, 92, 65],
            operation: Op::Mul(11),
            divisible: 5,
            send: (7, 4),
            inspections: 0,
        },
        Monkey {
            items: vec![91, 53, 96, 98, 68, 82],
            operation: Op::Square,
            divisible: 3,
            send: (4, 2),
            inspections: 0,
        },
    ]
}

fn main() {
    println!("{}", part1(get_monkeys()));
    println!("{}", part2(get_monkeys()));
}

fn part1(mut monkeys: Vec<Monkey>) -> usize {
    for _ in 0..20 {
        run_round(&mut monkeys, None);
    }

    monkeys
        .iter()
        .map(|m| m.inspections)
        .k_largest(2)
        .product::<usize>()
}

fn part2(mut monkeys: Vec<Monkey>) -> usize {
    // Optimization: we only care that the numbers are divisible by `monkey.divisible`
    // In order to keep numbers small for part 2, we can simplify numbers by the lcm of all divisors
    let lcm = monkeys.iter().map(|m| m.divisible).product::<i64>();

    for _ in 0..10000 {
        run_round(&mut monkeys, Some(lcm));
    }

    monkeys
        .iter()
        .map(|m| m.inspections)
        .k_largest(2)
        .product::<usize>()
}

#[cfg(test)]
mod test {
    use crate::{get_monkeys, part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_monkeys()), 55930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_monkeys()), 14636993466);
    }
}
