use aoc::input_str;

#[derive(Debug, Clone)]
struct Computer {
    // the A, B, C registers
    regs: [u64; 3],
    // the instruction pointer
    ip: isize,
    // program memory
    program: Vec<u64>,
    // output buffer
    output: Vec<u64>,
}

impl Computer {
    fn new(a: u64, b: u64, c: u64, program: Vec<u64>) -> Self {
        Self {
            regs: [a, b, c],
            ip: 0,
            program,
            output: Vec::new(),
        }
    }

    fn load(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4 => self.regs[0],
            5 => self.regs[1],
            6 => self.regs[2],
            _ => panic!("Invalid operand"),
        }
    }

    fn run(&mut self) {
        while self.step() {}
    }

    fn step(&mut self) -> bool {
        // stop if the ip is out of bounds
        if self.ip as usize >= self.program.len() {
            return false;
        }

        let op = self.program[self.ip as usize + 1];

        match self.program[self.ip as usize] {
            0 => {
                self.regs[0] >>= self.load(op);
            }
            1 => {
                self.regs[1] ^= op;
            }
            2 => {
                let n = self.load(op);
                self.regs[1] = n % 8;
            }
            3 => {
                if self.regs[0] != 0 {
                    self.ip = op as isize - 2;
                }
            }
            4 => {
                self.regs[1] ^= self.regs[2];
            }
            5 => {
                self.output.push(self.load(op) % 8);
            }
            6 => {
                self.regs[1] = self.regs[0] >> self.load(op);
            }
            7 => {
                self.regs[2] = self.regs[0] >> self.load(op);
            }
            _ => {
                panic!("Invalid opcode");
            }
        };

        self.ip += 2;
        true
    }
}

fn main() {
    let input_str = input_str!(2024, 17);

    // Parse the input
    let lines: Vec<&str> = input_str.trim().lines().collect();

    // Parse register A
    let reg_a = lines[0]
        .strip_prefix("Register A: ")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    // Parse register B
    let reg_b = lines[1]
        .strip_prefix("Register B: ")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    // Parse register C
    let reg_c = lines[2]
        .strip_prefix("Register C: ")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    // Parse program
    let program: Vec<u64> = lines[4]
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let time = std::time::Instant::now();
    let mut computer = Computer::new(reg_a, reg_b, reg_c, program.clone());
    computer.run();

    println!(
        "Part 1: {}",
        computer
            .output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
    println!("Time: {:?}", time.elapsed());

    let time = std::time::Instant::now();
    let mut a = 0;
    for i in (0..program.len()).rev() {
        a <<= 3;

        loop {
            let mut computer = Computer::new(a, 0, 0, program.clone());
            computer.run();

            // check if we've got a partial solution
            if computer.output[..] == program[i..] {
                break;
            }

            a += 1;
        }
    }
    println!("Part 2: {}", a);
    println!("Time: {:?}", time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    // If register C contains 9, the program 2,6 would set register B to 1.
    #[test]
    fn test_1() {
        let mut computer = Computer::new(0, 0, 9, vec![2, 6]);
        while computer.step() {}
        assert_eq!(computer.regs[1], 1);
    }

    // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
    #[test]
    fn test_2() {
        let mut computer = Computer::new(10, 0, 0, vec![5, 0, 5, 1, 5, 4]);
        while computer.step() {}
        assert_eq!(computer.output, vec![0, 1, 2]);
    }

    // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
    #[test]
    fn test_3() {
        let mut computer = Computer::new(2024, 0, 0, vec![0, 1, 5, 4, 3, 0]);
        while computer.step() {}
        assert_eq!(computer.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer.regs[0], 0);
    }

    // If register B contains 29, the program 1,7 would set register B to 26.
    #[test]
    fn test_4() {
        let mut computer = Computer::new(0, 29, 0, vec![1, 7]);
        while computer.step() {}
        assert_eq!(computer.regs[1], 26);
    }

    // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
    #[test]
    fn test_5() {
        let mut computer = Computer::new(0, 2024, 43690, vec![4, 0]);
        while computer.step() {}
        assert_eq!(computer.regs[1], 44354);
    }

    #[test]
    fn test_example() {
        let mut computer = Computer::new(729, 0, 0, vec![0, 1, 5, 4, 3, 0]);
        while computer.step() {}
        assert_eq!(computer.output, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }
}
