use std::collections::VecDeque;
use std::error::Error;

#[derive(Copy, Clone)]
enum Instruction {
    Noop,
    Addx(i64),
}

struct Cpu {
    instructions: VecDeque<Instruction>,
    cycle: i64,
    register: i64,
    tick_queue: VecDeque<i64>,
}

impl Cpu {
    fn new(instructions: &[Instruction]) -> Cpu {
        Cpu {
            instructions: instructions.iter().copied().collect(),
            cycle: 1,
            register: 1,
            tick_queue: VecDeque::new(),
        }
    }

    fn tick(&mut self) {
        self.cycle += 1;

        if self.tick_queue.is_empty() {
            if let Some(instruction) = self.instructions.pop_front() {
                match instruction {
                    Instruction::Noop => self.tick_queue.push_back(0),
                    Instruction::Addx(x) => {
                        self.tick_queue.push_back(0);
                        self.tick_queue.push_back(x);
                    }
                }
            }
        }

        if let Some(delta) = self.tick_queue.pop_front() {
            self.register += delta;
        }
    }

    fn signal_strength(&self) -> i64 {
        self.register * self.cycle
    }

    fn advance_to(&mut self, cycle: i64) {
        while self.cycle < cycle {
            self.tick();
        }
    }

    fn sprite_visible(&self) -> bool {
        let crt = (self.cycle - 1) % 40;
        let spr = self.register;

        crt >= spr - 1 && crt <= spr + 1
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let (ins, val) = l.split_once(' ').unwrap_or((l, ""));
            match ins {
                "noop" => Instruction::Noop,
                "addx" => {
                    let val = val
                        .parse()
                        .unwrap_or_else(|_| panic!("Could not parse number {}", val));
                    Instruction::Addx(val)
                }
                _ => panic!("Could not parse {l}"),
            }
        })
        .collect()
}

fn part_one(instructions: &[Instruction]) -> i64 {
    let mut cpu = Cpu::new(instructions);
    vec![20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|cycle| {
            cpu.advance_to(cycle);
            cpu.signal_strength()
        })
        .sum()
}

fn part_two(instructions: &[Instruction]) -> String {
    let mut cpu = Cpu::new(instructions);
    let mut out: String = String::new();

    for _ in 0..6 {
        for _ in 0..40 {
            if cpu.sprite_visible() {
                out.push('#');
            } else {
                out.push('.');
            }
            cpu.tick();
        }
        out.push('\n');
    }

    out
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/day10.txt");
    let instructions = parse(input);

    let p1 = part_one(instructions.as_slice());
    println!("Part One: {p1}");

    let p2 = part_two(instructions.as_slice());
    println!("Part Two:\n{p2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_instructions() -> Vec<Instruction> {
        let input = include_str!("../input/day10_test.txt");
        parse(input)
    }

    #[test]
    fn small_program_example() {
        let mut cpu = Cpu::new(
            vec![
                Instruction::Noop,
                Instruction::Addx(3),
                Instruction::Addx(-5),
            ]
            .as_slice(),
        );

        assert_eq!(cpu.register, 1);

        //noop
        cpu.tick();
        assert_eq!(cpu.register, 1);

        //addx 3
        cpu.tick();
        assert_eq!(cpu.register, 1);
        cpu.tick();
        assert_eq!(cpu.register, 4);

        //addx -5
        cpu.tick();
        assert_eq!(cpu.register, 4);
        cpu.tick();
        assert_eq!(cpu.register, -1);
    }

    #[test]
    fn advance_to_test() {
        let mut cpu = Cpu::new(test_instructions().as_slice());

        cpu.advance_to(20);
        assert_eq!(cpu.signal_strength(), 420);

        cpu.advance_to(60);
        assert_eq!(cpu.signal_strength(), 1140);

        cpu.advance_to(100);
        assert_eq!(cpu.signal_strength(), 1800);

        cpu.advance_to(140);
        assert_eq!(cpu.signal_strength(), 2940);

        cpu.advance_to(180);
        assert_eq!(cpu.signal_strength(), 2880);

        cpu.advance_to(220);
        assert_eq!(cpu.signal_strength(), 3960);
        assert_eq!(cpu.cycle, 220);
    }

    #[test]
    fn part_one_test() {
        let result = part_one(test_instructions().as_slice());
        assert_eq!(result, 13140);
    }

    #[test]
    fn part_two_test() {
        let result = part_two(test_instructions().as_slice());
        let expected = include_str!("../input/day10_test2.txt");

        assert_eq!(result, expected);
    }
}
