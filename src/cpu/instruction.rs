pub enum Instruction {
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    XOR(ArithmeticTarget),
    OR(ArithmeticTarget),
    CP(ArithmeticTarget),
}
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    BC,
    DE,
    HL,
    SP,
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Self> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_unprefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            _ => None,
        }
    }

    fn from_byte_unprefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            _ => None,
        }
    }
}
