use inpt::Inpt;

#[derive(Debug, Inpt, Clone, Copy)]
#[inpt(regex = r"(\d+)-(\d+),(\d+)-(\d+)")]
struct Ranges {
    start1: i32,
    end1: i32,
    start2: i32,
    end2: i32,
}

impl Ranges {
    fn part1(&self) -> bool {
        let r1 = self.start1..=self.end1;
        let r2 = self.start2..=self.end2;

        (r1.contains(&self.start2) && r1.contains(&self.end2))
            || (r2.contains(&self.start1) && r2.contains(&self.end1))
    }

    fn part2(&self) -> bool {
        let r1 = self.start1..=self.end1;
        let r2 = self.start2..=self.end2;

        r1.contains(&self.start2)
            || r1.contains(&self.end2)
            || r2.contains(&self.start1)
            || r2.contains(&self.end1)
    }
}

fn main() {
    let input = inpt::inpt::<Vec<Ranges>>(aoc::get_input!(2022, 4)).unwrap();

    let part1 = input.iter().filter(|r| r.part1()).count();
    println!("{}", part1);

    let part2 = input.iter().filter(|r| r.part2()).count();
    println!("{}", part2);
}
