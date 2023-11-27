use std::{collections::HashMap, io::Write};

use aoc::input_str;

#[derive(Debug, Clone)]
enum Monkey {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Num(i64),

    Eq(String, String),
    Unknown,
}

impl Monkey {
    fn into_left_right(self) -> Option<(String, String)> {
        match self {
            Monkey::Add(left, right) => Some((left, right)),
            Monkey::Sub(left, right) => Some((left, right)),
            Monkey::Mul(left, right) => Some((left, right)),
            Monkey::Div(left, right) => Some((left, right)),
            _ => None,
        }
    }

    fn undo(&self, monkeys: &MonkeyMath, y: &mut i64) -> Option<String> {
        let (left, right) = match self {
            Monkey::Add(l, r) => Some((l, r)),
            Monkey::Sub(l, r) => Some((l, r)),
            Monkey::Mul(l, r) => Some((l, r)),
            Monkey::Div(l, r) => Some((l, r)),
            Monkey::Eq(l, r) => Some((l, r)),
            _ => None,
        }?;

        let mut num = None;
        let mut eq = None;
        if let Some(Monkey::Num(n)) = monkeys.get(left) {
            num = Some(n);
            eq = Some(right);
        } else if let Some(Monkey::Num(n)) = monkeys.get(right) {
            num = Some(n);
            eq = Some(left);
        }
        let num = num?;
        let eq = eq?;

        match self {
            Monkey::Add(_, _) => *y -= num,
            Monkey::Sub(_, _) => *y += num,
            Monkey::Mul(_, _) => {
                *y /= num;
            }
            Monkey::Div(_, _) => *y *= num,
            Monkey::Eq(_, _) => *y = -num,
            _ => {}
        }

        Some(eq.clone())
    }
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Monkey::Add(left, right) => write!(f, "({} + {})", left, right),
            Monkey::Sub(left, right) => write!(f, "({} - {})", left, right),
            Monkey::Mul(left, right) => write!(f, "({} * {})", left, right),
            Monkey::Div(left, right) => write!(f, "({} / {})", left, right),
            Monkey::Num(num) => write!(f, "{}", num),
            Monkey::Eq(left, right) => write!(f, "{} = {}", left, right),
            Monkey::Unknown => write!(f, "???"),
        }
    }
}

struct MonkeyMath {
    monkeys: HashMap<String, Monkey>,
}

impl MonkeyMath {
    fn from(input: &str) -> Self {
        let mut monkeys = HashMap::new();

        for line in input.lines() {
            let mut parts = line.split(": ");
            let name = parts.next().unwrap();
            let expr = parts.next().unwrap();

            let monkey = if let Ok(num) = expr.parse::<i64>() {
                Monkey::Num(num)
            } else {
                let mut parts = expr.split(' ');
                let left = parts.next().unwrap();
                let op = parts.next().unwrap();
                let right = parts.next().unwrap();

                match op {
                    "+" => Monkey::Add(left.to_string(), right.to_string()),
                    "-" => Monkey::Sub(left.to_string(), right.to_string()),
                    "*" => Monkey::Mul(left.to_string(), right.to_string()),
                    "/" => Monkey::Div(left.to_string(), right.to_string()),
                    _ => panic!("bad op"),
                }
            };

            monkeys.insert(name.to_string(), monkey);
        }

        Self { monkeys }
    }

    fn get(&self, name: &str) -> Option<&Monkey> {
        self.monkeys.get(name)
    }

    fn replace(&mut self, name: &str, monkey: Monkey) {
        self.monkeys.insert(name.to_string(), monkey);
    }

    fn eval(&self, name: &str) -> Option<i64> {
        let monkey = self.monkeys.get(name).unwrap();

        match monkey {
            Monkey::Num(num) => Some(*num),
            Monkey::Add(left, right) => Some(self.eval(left)? + self.eval(right)?),
            Monkey::Sub(left, right) => Some(self.eval(left)? - self.eval(right)?),
            Monkey::Mul(left, right) => Some(self.eval(left)? * self.eval(right)?),
            Monkey::Div(left, right) => Some(self.eval(left)? / self.eval(right)?),
            _ => None,
        }
    }

    // Reduce the expression as much as possible
    fn simplify(&mut self, name: &str) -> Option<i64> {
        let monkey = self.monkeys.get(name).unwrap().clone();

        let value = match monkey {
            Monkey::Add(left, right) => {
                let left = self.simplify(&left);
                let right = self.simplify(&right);

                if let (Some(l), Some(r)) = (left, right) {
                    Some(l + r)
                } else {
                    None
                }
            }
            Monkey::Sub(left, right) => {
                let left = self.simplify(&left);
                let right = self.simplify(&right);

                if let (Some(l), Some(r)) = (left, right) {
                    Some(l - r)
                } else {
                    None
                }
            }
            Monkey::Mul(left, right) => {
                let left = self.simplify(&left);
                let right = self.simplify(&right);

                if let (Some(l), Some(r)) = (left, right) {
                    Some(l * r)
                } else {
                    None
                }
            }
            Monkey::Div(left, right) => {
                let left = self.simplify(&left);
                let right = self.simplify(&right);

                if let (Some(l), Some(r)) = (left, right) {
                    Some(l / r)
                } else {
                    None
                }
            }
            Monkey::Eq(left, right) => {
                let _ = self.simplify(&left);
                let _ = self.simplify(&right);
                None
            }
            Monkey::Num(num) => Some(num),
            Monkey::Unknown => None,
        };

        if let Some(value) = value {
            if let Some((l, r)) = self
                .monkeys
                .insert(name.to_string(), Monkey::Num(value))
                .and_then(|m| m.into_left_right())
            {
                self.monkeys.remove(&l);
                self.monkeys.remove(&r);
            }
        }

        value
    }

    // Returns the dot notation of the expression, for plotting on graphviz
    fn dot(&self) -> String {
        let mut dot = "digraph {\n".to_string();

        for (name, monkey) in &self.monkeys {
            match monkey {
                Monkey::Add(left, right) => {
                    dot.push_str(&format!("\t{} -> {};\n", name, left));
                    dot.push_str(&format!("\t{} -> {};\n", name, right));
                    dot.push_str(&format!("\t{} [label=\"+\"];\n", name));
                }
                Monkey::Sub(left, right) => {
                    dot.push_str(&format!("\t{} -> {};\n", name, left));
                    dot.push_str(&format!("\t{} -> {};\n", name, right));
                    dot.push_str(&format!("\t{} [label=\"-\"];\n", name));
                }
                Monkey::Mul(left, right) => {
                    dot.push_str(&format!("\t{} -> {};\n", name, left));
                    dot.push_str(&format!("\t{} -> {};\n", name, right));
                    dot.push_str(&format!("\t{} [label=\"*\"];\n", name));
                }
                Monkey::Div(left, right) => {
                    dot.push_str(&format!("\t{} -> {};\n", name, left));
                    dot.push_str(&format!("\t{} -> {};\n", name, right));
                    dot.push_str(&format!("\t{} [label=\"/\"];\n", name));
                }
                Monkey::Num(num) => {
                    dot.push_str(&format!("\t{} [label=\"{}\", shape=box];\n", name, num));
                }
                Monkey::Eq(left, right) => {
                    dot.push_str(&format!("\t{} -> {};\n", name, left));
                    dot.push_str(&format!("\t{} -> {};\n", name, right));
                    dot.push_str(&format!("\t{} [label=\"=\"];\n", name));
                }
                Monkey::Unknown => {}
            }
        }

        dot.push_str("}\n");

        dot
    }

    // Solves part 2 like an equation
    fn solve(&self) -> i64 {
        let mut y = 0;
        let mut op = "root".to_string();

        while let Some(monkey) = self.get(&op) {
            if let Some(next) = monkey.undo(self, &mut y) {
                op = next;
            } else {
                break;
            }
        }

        y
    }

    // returns true if the expression is Equal and the left and right sides are equal
    fn equal(&self, name: &str) -> bool {
        if let Some(Monkey::Eq(l, r)) = self.get(name) {
            let left = self.eval(l);
            let right = self.eval(r);

            if let (Some(l), Some(r)) = (left, right) {
                l == r
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl std::fmt::Display for MonkeyMath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, monkey) in &self.monkeys {
            writeln!(f, "{}: {}", name, monkey)?;
        }
        Ok(())
    }
}

fn part1(input: &str) -> i64 {
    let monkeys = MonkeyMath::from(input);
    monkeys.eval("root").unwrap()
}

fn part2(input: &str) -> i64 {
    let mut monkeys = MonkeyMath::from(input);
    monkeys.replace("humn", Monkey::Unknown);

    let root = monkeys.get("root").unwrap().clone();
    let (left, right) = root.into_left_right().unwrap();
    monkeys.replace("root", Monkey::Eq(left, right));

    let _ = monkeys.simplify("root");

    // Save the dot notation to a file
    let mut file = std::fs::File::create("graph.dot").unwrap();
    file.write_all(monkeys.dot().as_bytes()).unwrap();

    let n = monkeys.solve();

    // Check +/- 1000 for the right answer
    for i in n - 1000..n + 1000 {
        monkeys.replace("humn", Monkey::Num(i));
        if monkeys.equal("root") {
            return i;
        }
    }

    panic!("no answer found");
}

fn main() {
    let input = input_str!(2022, 21);
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "root: pppw + sjmn\ndbpl: 5\ncczh: sllz + lgvd\nzczc: 2\nptdq: humn - dvpt\ndvpt: 3\nlfqf: 4\nhumn: 5\nljgn: 2\nsjmn: drzm * dbpl\nsllz: 4\npppw: cczh / lfqf\nlgvd: ljgn * ptdq\ndrzm: hmdt - zczc\nhmdt: 32";

        assert_eq!(part1(input), 152);
        assert_eq!(part2(input), 301);
    }

    #[test]
    fn test_part1() {
        let input = input_str!(2022, 21);
        assert_eq!(part1(&input), 152479825094094);
    }

    #[test]
    fn test_part2() {
        let input = input_str!(2022, 21);
        assert_eq!(part2(&input), 3360561285172);
    }
}
