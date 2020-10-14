#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Opcode {
    HLT,
    LOAD,
    INC,
    DEC,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPF,
    JMPB,
    EQ,
    NEQ,
    GT,
    LT,
    GTE,
    LTE,
    JMPE,
    DJMPE,
    ALOC,
    PRTS,
    NOP,
    IGL
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {

        match v {
            0  => Opcode::HLT,
            1  => Opcode::LOAD,
            2  => Opcode::INC,
            3  => Opcode::DEC,
            4  => Opcode::ADD,
            5  => Opcode::SUB,
            6  => Opcode::MUL,
            7  => Opcode::DIV,
            8  => Opcode::JMP,
            9  => Opcode::JMPF,
            10 => Opcode::JMPB,
            11 => Opcode::EQ,
            12 => Opcode::NEQ,
            13 => Opcode::GT,
            14 => Opcode::LT,
            15 => Opcode::GTE,
            16 => Opcode::LTE,
            17 => Opcode::JMPE,
            18 => Opcode::DJMPE,
            19 => Opcode::ALOC,
            20 => Opcode::PRTS,
            21 => Opcode::NOP,
            _ => Opcode::IGL
        }
    }
}

impl From<&str> for Opcode {
    fn from(v: &str) -> Self {
        match v {
            "hlt"  => Opcode::HLT,
            "load" => Opcode::LOAD,
            "inc"  => Opcode::INC,
            "dec"  => Opcode::DEC,
            "add"  => Opcode::ADD,
            "sub"  => Opcode::SUB,
            "mul"  => Opcode::MUL,
            "div"  => Opcode::DIV,
            "jmp"  => Opcode::JMP,
            "jmpf"  => Opcode::JMPF,
            "jmpb"  => Opcode::JMPB,
            "eq"  => Opcode::EQ,
            "neq" => Opcode::NEQ,
            "gt" => Opcode::GT,
            "lt" => Opcode::LT,
            "gte" => Opcode::GTE,
            "lte" => Opcode::LTE,
            "jmpe" => Opcode::JMPE,
            "djmpe" => Opcode::DJMPE,
            "aloc" => Opcode::ALOC,
            "ptrs" => Opcode::PRTS,
            "nop" => Opcode::NOP,
            _ => Opcode::IGL
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Instruction {
            opcode
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let inst = Instruction::new(Opcode::HLT);
        assert_eq!(inst.opcode, Opcode::HLT);
    }

    #[test]
    fn test_opcode_from() {
        let mut opcode = Opcode::from(0);
        assert_eq!(opcode, Opcode::HLT);

        opcode = Opcode::from(100);
        assert_eq!(opcode, Opcode::IGL);
    }

    #[test]
    fn test_opcode_from_str() {
        let mut opcode = Opcode::from("load");
        assert_eq!(opcode, Opcode::LOAD);

        opcode = Opcode::from("oadl");
        assert_eq!(opcode, Opcode::IGL);
    }
}