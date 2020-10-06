#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Opcode {
    HLT,
    LOAD,
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
    IGL
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {

        match v {
            0  => Opcode::HLT,
            1  => Opcode::LOAD,
            2  => Opcode::ADD,
            3  => Opcode::SUB,
            4  => Opcode::MUL,
            5  => Opcode::DIV,
            6  => Opcode::JMP,
            7  => Opcode::JMPF,
            8  => Opcode::JMPB,
            9  => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GT,
            12 => Opcode::LT,
            13 => Opcode::GTE,
            14 => Opcode::LTE,
            15 => Opcode::JMPE,
            _ => Opcode::IGL
        }
    }
}

impl From<&str> for Opcode {
    fn from(v: &str) -> Self {
        match v {
            "hlt"  => Opcode::HLT,
            "load" => Opcode::LOAD,
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