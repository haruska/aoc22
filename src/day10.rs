use std::collections::VecDeque;
use std::error::Error;

enum Instruction {
    Noop,
    Addx(i32),
}

struct Cpu {
    instructions: VecDeque<Instruction>,
    cycle: usize,
    register: i32,
    tick_queue: VecDeque<i32>,
}

impl Cpu {
    fn new(instructions: Vec<Instruction>) -> Cpu {
        Cpu {
            instructions: instructions.into_iter().collect(),
            cycle: 0,
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
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_program_example() {
        let mut cpu = Cpu::new(vec![
            Instruction::Noop,
            Instruction::Addx(3),
            Instruction::Addx(-5),
        ]);

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
}
