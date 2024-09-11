use crate::common::{instructions::Instruction, Word};

pub struct VirtualMachine {
    memory: [Word; 128],
    accumulator: Word,
    memory_address: Word,
}

enum AddressMode {
    IMMEDIATE,
    DIRECT,
    INDIRECT,
}

struct Operands {
    first: Option<Word>,
    second: Option<Word>,
}

pub struct Payload {
    operands: Operands,
    address_mode: AddressMode,
}

impl VirtualMachine {
    pub fn execute(&mut self, instr: Instruction, payload: Payload) {
        match instr {
            Instruction::ADD => self.add(payload),
            _ => panic!("not implemented"),
        }
    }

    fn add(&mut self, payload: Payload) {
        let first_operand = payload.operands.first;
        let second_operand = payload.operands.second;

        if let Some(_) = second_operand {
            panic!("second operand must be None")
        }

        match first_operand {
            Some(value) => match payload.address_mode {
                AddressMode::IMMEDIATE => self.accumulator += value,
                AddressMode::DIRECT => self.accumulator += self.memory[value as usize],
                AddressMode::INDIRECT => panic!("first operand must be None"),
            },
            None => match payload.address_mode {
                AddressMode::INDIRECT => {
                    self.accumulator += self.memory[self.memory_address as usize]
                }
                _ => panic!("missing first operand"),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // To use in unit tests
    fn setup_vm(acc: Word) -> VirtualMachine {
        VirtualMachine {
            memory: [0; 128],
            accumulator: acc,
            memory_address: 39,
        }
    }

    #[test]
    #[should_panic(expected = "second operand must be None")]
    fn add_some_second_operand() {
        let mut vm = setup_vm(18);
        let payload = Payload {
            address_mode: AddressMode::IMMEDIATE,
            operands: Operands {
                first: Some(19),
                second: Some(28),
            },
        };

        vm.execute(Instruction::ADD, payload)
    }

    #[test]
    #[should_panic(expected = "first operand must be None")]
    fn add_none_first_operand() {
        let mut vm = setup_vm(20);
        let payload = Payload {
            address_mode: AddressMode::INDIRECT,
            operands: Operands {
                first: Some(20),
                second: None,
            },
        };

        vm.execute(Instruction::ADD, payload);
    }

    #[test]
    #[should_panic(expected = "missing first operand")]
    fn add_missing_first_operand() {
        let mut vm = setup_vm(20);
        let payload = Payload {
            address_mode: AddressMode::DIRECT,
            operands: Operands {
                first: None,
                second: None,
            }
        };

        vm.execute(Instruction::ADD, payload)
    }

    #[test]
    fn add_immediate() {
        let mut vm = setup_vm(16);
        let payload = Payload {
            address_mode: AddressMode::IMMEDIATE,
            operands: Operands {
                first: Some(10),
                second: None,
            },
        };

        vm.execute(Instruction::ADD, payload);

        assert_eq!(vm.accumulator, 26)
    }

    #[test]
    fn add_direct() {
        let mut vm = setup_vm(47);
        vm.memory[28] = 55;
        let payload = Payload {
            address_mode: AddressMode::DIRECT,
            operands: Operands {
                first: Some(28),
                second: None,
            },
        };

        vm.execute(Instruction::ADD, payload);

        assert_eq!(vm.accumulator, 102)
    }

    #[test]
    fn add_indirect() {
        let mut vm = setup_vm(47);
        vm.memory[77] = 96;
        vm.memory_address = 77;
        let payload = Payload {
            address_mode: AddressMode::INDIRECT,
            operands: Operands {
                first: None,
                second: None,
            },
        };

        vm.execute(Instruction::ADD, payload);

        assert_eq!(vm.accumulator, 143)
    }
}
