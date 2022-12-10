use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Instruction<'a> {
    Noop,
    Add(&'a str, i128),
}

pub struct VirtualMachine<'a> {
    _registers: HashMap<&'a str, i128>,
    _cycle: usize,
}

impl<'a> VirtualMachine<'a> {
    pub fn new() -> VirtualMachine<'a> {
        VirtualMachine {
            _registers: HashMap::new(),
            _cycle: 0,
        }
    }

    fn parse_unary(command_name: &'a str, raw_command: &'a str, args: &'a str) -> (&'a str, i128) {
        let (_, register) = raw_command.split_once(command_name).expect("Could not get register from command");
        (register, args.parse().expect("Could not parse arg"))
    }

    pub fn parse_instruction(line: &'a str) -> Instruction<'a> {
        if line == "noop" {
            return Instruction::Noop;
        }

        let (command, args) = line.split_once(" ").expect("Could not split line, are there args?");

        if command.starts_with("add") {
            let (register, amount) = VirtualMachine::parse_unary("add", command, args);
            return Instruction::Add(register, amount);
        }

        panic!("Could not parse instruction {}", line);
    }

    pub fn get_register_value(&self, name: &str) -> Option<i128> {
        self._registers.get(name).map(|v| *v)
    }

    fn get_or_insert_register_ref(&mut self, name: &'a str) -> &mut i128 {
        self._registers.entry(name).or_insert(1)
    }

    pub fn get_cycle(&self) -> usize {
        return self._cycle;
    }

    pub fn execute_instruction<F>(&mut self, instruction: Instruction<'a>, mut on_cycle: F)
        where F: FnMut(&Self) -> () {
        let cycle_count = match instruction {
            Instruction::Noop => 1,
            Instruction::Add(_, _) => 2
        };

        for _ in 0..cycle_count {
            self._cycle += 1;
            on_cycle(&self);
        }

        match instruction {
            Instruction::Noop => (),
            Instruction::Add(register, amount) => *self.get_or_insert_register_ref(register) += amount
        }
    }
}