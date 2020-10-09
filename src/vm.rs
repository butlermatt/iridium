use crate::instructions::Opcode;
use crate::assembler::PIE_HEADER_PREFIX;

pub struct VM {
    pub registers: [i32; 32],
    pc: usize,
    pub program: Vec<u8>,
    heap: Vec<u8>,
    remainder: u32,
    equal_flag: bool,
    ro_data: Vec<u8>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: [0; 32],
            pc: 65,
            program: vec![],
            heap: vec![],
            remainder: 0,
            equal_flag: false,
            ro_data: vec![],
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.program.push(byte);
    }

    pub fn add_bytes(&mut self, mut bytes: Vec<u8>) {
        self.program.append(&mut bytes);
    }

    fn execute_instruction(&mut self) -> bool {
        // If our program counter has exceeded the length of the program itself,
        // something has gong awry
        if self.pc >= self.program.len() {
            return true;
        }

        let op = self.decode_opcode();

        match op {
            Opcode::HLT => {
                return true;
            },
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize; // we cast to usize so we can use it as an index into the array
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32; // Our registers are i32s so we need to cast it. We'll cover that later.
            },
            Opcode::INC => {
                let register = self.next_8_bits() as usize;
                self.registers[register] += 1;
            },
            Opcode::DEC => {
                let register = self.next_8_bits() as usize;
                self.registers[register] -= 1;
            },
            Opcode::ADD | Opcode::SUB | Opcode::MUL | Opcode::DIV => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = match op {
                    Opcode::ADD => reg1 + reg2,
                    Opcode::SUB => reg1 - reg2,
                    Opcode::MUL => reg1 * reg2,
                    Opcode::DIV => {
                        self.remainder = (reg1 % reg2) as u32;
                        reg1 / reg2
                    },
                    _ => { -100 } // Impossible to reach
                };
            },
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            },
            Opcode::JMPF => {
                let amount = self.registers[self.next_8_bits() as usize];
                self.pc += amount as usize;
            },
            Opcode::JMPB => {
               let amount = self.registers[self.next_8_bits() as usize];
                self.pc -= amount as usize;
            },
            Opcode::EQ | Opcode::NEQ | Opcode::GT | Opcode::LT | Opcode::GTE | Opcode::LTE => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = match op {
                    Opcode::EQ => { reg1 == reg2 },
                    Opcode::NEQ => { reg1 != reg2 },
                    Opcode::GT => { reg1 > reg2 },
                    Opcode::LT => { reg1 < reg2 },
                    Opcode::GTE => { reg1 >= reg2 },
                    Opcode::LTE => { reg1 <= reg2 },
                    _ => { false } // Can't reach this point
                };

                self.next_8_bits(); // Eat empty byte?
            },
            Opcode::JMPE => {
                let register = self.next_8_bits() as usize;
                let target = self.registers[register];
                if self.equal_flag {
                    self.pc = target as usize;
                }
            },
            Opcode::ALOC => {
                let register = self.next_8_bits() as usize;
                let bytes = self.registers[register];
                let new_end = self.heap.len() as i32 + bytes;
                self.heap.resize(new_end as usize, 0);
            },
            Opcode::PRTS => {
                // Takes one operand either a starting index in the RO section of memory
                // Or a symbol (in the form of @symbol_name) which will look up the offset in the symbol table.
                // The instruction reads each byte and prints it, until it comes to NULL
                let starting_offset = self.next_16_bits() as usize;
                let mut ending_offset = starting_offset;
                let slice = self.ro_data.as_slice();
                while slice[ending_offset] != 0 {
                    ending_offset += 1;
                }

                let result = std::str::from_utf8(&slice[starting_offset..ending_offset]);
                match result {
                    Ok(s) => print!("{}", s),
                    Err(e) => println!("Error decoding string for PTRS instruction: {:?}", e),
                };
            },
            Opcode::IGL => {
                println!("Illegal Instruction encountered");
                return true;
            }
        }

        false
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;
    }

    #[allow(dead_code)]
    fn verify_header(&self) -> bool {
        self.program[0..4] == PIE_HEADER_PREFIX
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assembler::PIE_HEADER_LENGTH;

    fn prepend_header(mut b: Vec<u8>) -> Vec<u8> {
        let mut prepension = vec![];
        for byte in PIE_HEADER_PREFIX.iter() {
            prepension.push(byte.clone());
        }
        while prepension.len() <= PIE_HEADER_LENGTH {
            prepension.push(0);
        }
        prepension.append(&mut b);
        prepension
    }

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0,0,0,0];
        test_vm.program = prepend_header(test_bytes);
        test_vm.run_once();
        assert_eq!(test_vm.pc, 66);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = prepend_header(vec![1, 0, 1, 244]);
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_inc_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 100;
        test_vm.program = prepend_header(vec![2, 0, 0, 0]);
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 101);
    }

    #[test]
    fn test_add_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = prepend_header(vec![
            1, 0, 0, 1, // Load 1 into r0
            1, 1, 0, 1, // Load 1 into r1
            4, 0, 1, 2, // Add r0 and r1 and store in r2
            0]);
        test_vm.run();
        assert_eq!(test_vm.registers[2], 2);
    }

    #[test]
    fn test_sub_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = prepend_header(vec![
            1, 0, 0, 4, // Load 4 => r0
            1, 1, 0, 1, // Load 1 => r1
            5, 0, 1, 2, // SUB r0 - r1 => r2
            0]); // Halt
        test_vm.run();
        assert_eq!(test_vm.registers[2], 3);
    }

    #[test]
    fn test_mul_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = prepend_header(vec![
            1, 0, 0, 4, // Load 4 => r0
            1, 1, 0, 2, // Load 2 => r1
            6, 0, 1, 2, // MUL r0 * r1 => r2
            0]); // Halt
        test_vm.run();
        assert_eq!(test_vm.registers[2], 8);
    }

    #[test]
    fn test_div_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = prepend_header(vec![
            1, 0, 0, 5, // Load 4 => r0
            1, 1, 0, 2, // Load 2 => r1
            7, 0, 1, 2, // DIV r0 - r1 => r2
            0]); // Halt
        test_vm.run();
        assert_eq!(test_vm.registers[2], 2);
        assert_eq!(test_vm.remainder, 1);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = prepend_header(vec![
            1, 0, 0, 1, // Load 1 into r0
            8, 0, 0, 0,// JMP from r0 (pc = 1)
        0]); // Halt
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = prepend_header(vec![
            1, 0, 0, 2, // Load 1 into r0
            9, 0, 0, 0,// JMPF from r0 (pc = 8)
            0]); // Halt
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.pc, 73);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = prepend_header(vec![
            1,  0, 0, 2, // Load 1 into r0
            10, 0, 0, 0,// JMPB from r0 (pc = 4)
            0]); // Halt
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.pc, 69);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = prepend_header(vec![
            11, 0, 1, 0, // EQ r0 == r1 (ignore last 0)
            11, 0, 1, 0, // EQ r0 == r1 (ignore last 0)
        0]); // Halt
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_neq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = prepend_header(vec![
            12, 0, 1, 0, // NEQ r0 != r1 (ignore last 0)
            12, 0, 1, 0, // NEQ r0 != r1 (ignore last 0)
            0]); // Halt
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_gt_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = prepend_header(vec![
            13, 0, 1, 0, // EQ r0 == r1 (ignore last 0)
            13, 0, 1, 0, // EQ r0 == r1 (ignore last 0)
            0]); // Halt
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_lt_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = prepend_header(vec![
            14, 0, 1, 0, // EQ r0 == r1 (ignore last 0)
            14, 0, 1, 0, // EQ r0 == r1 (ignore last 0)
            0]); // Halt
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_gtq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = prepend_header(vec![
            15, 0, 1, 0, // EQ r0 == r1 (ignore last 0)
            15, 0, 1, 0, // EQ r0 == r1 (ignore last 0)
            15, 0, 1, 0, // EQ r0 == r1 (ignore last 0)
            0]); // Halt
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 30;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_ltq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = prepend_header(vec![
            16, 0, 1, 0, // EQ r0 == r1 (ignore last 0)
            16, 0, 1, 0, // EQ r0 == r1 (ignore last 0)
            16, 0, 1, 0, // EQ r0 == r1 (ignore last 0)
            0]); // Halt
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 0;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = true;
        test_vm.program = prepend_header(vec![
            17, 0, 0, 0,
            17, 0, 0, 0,
            17, 0, 0, 0,
        ]);
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }

    #[test]
    fn test_igl_opcode() {
        let mut test_vm = VM::new();
        let test_bytes = prepend_header(vec![200,0,0,0]);
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 66);
    }

    #[test]
    fn test_aloc_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1024;
        test_vm.program = prepend_header(vec![18, 0, 0, 0]);
        test_vm.run_once();
        assert_eq!(test_vm.heap.len(), 1024);
    }
}