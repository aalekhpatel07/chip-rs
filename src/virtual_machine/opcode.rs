#[deny(clippy::incorrect_ident_case)]

use thiserror::Error;

use crate::data_structures::{Nibble, NibblePair};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OpKind {
    Assig,
    BCD,
    BitOp,
    Call,
    Cond,
    Const,
    Display,
    Flow,
    KeyOp,
    Math,
    MEM,
    Rand,
    Sound,
    Timer
}

/// #### The symbols:
/// - NNN: Address
/// - NN: 8-bit constant
/// - N: 4-bit constant
/// - X and Y: 4-bit register identifier
/// - PC: Program Counter
/// - I: 16-bit register (For memory address)(Similar to void pointer);
/// - VN: One of the 16 available variables. N may be 0 to F (hexadecimal);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OpLiteral {
    _0NNN,
    _00E0,
    _00EE,
    _1NNN,
    _2NNN,
    _3XNN,
    _4XNN,
    _5XY0,
    _6XNN,
    _7XNN,
    _8XY0,
    _8XY1,
    _8XY2,
    _8XY3,
    _8XY4,
    _8XY5,
    _8XY6,
    _8XY7,
    _8XYE,
    _9XY0,
    _ANNN,
    _BNNN,
    _CXNN,
    _DXYN,
    _EX9E,
    _EXA1,
    _FX07,
    _FX0A,
    _FX15,
    _FX18,
    _FX1E,
    _FX29,
    _FX33,
    _FX55,
    _FX65
}


#[derive(Debug, Copy, Clone)]
pub struct OpCode {
    pub(crate) value: u16,
    pub(crate) literal: OpLiteral,
    pub(crate) kind: OpKind,
}


#[derive(Error, Debug)]
pub enum OpCodeError {
    #[error("Failed to convert `{0}` to a known opcode. ")]
    Unknown(u16)
}


impl TryFrom<u16> for OpCode {
    type Error = OpCodeError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let high_byte = (value >> 8) as u8;
        let low_byte = (value & 0b1111_1111u16) as u8;

        let high_pair: NibblePair = high_byte.into();
        let low_pair: NibblePair = low_byte.into();

        let first = high_pair.high.to_hex_char();
        let second = high_pair.low.to_hex_char();
        let third = low_pair.high.to_hex_char();
        let fourth = low_pair.low.to_hex_char();

        match (first, second, third, fourth) {
            ('0', '0', 'E', '0')  => Ok(OpCode { value, literal: OpLiteral::_00E0, kind: OpKind::Display }),
            ('0', '0', 'E', 'E')  => Ok(OpCode { value, literal: OpLiteral::_00EE, kind: OpKind::Flow }),
            ('0', _, _, _) => Ok(OpCode { value, literal: OpLiteral::_0NNN, kind: OpKind::Call }),
            ('1', _, _, _) => Ok(OpCode { value, literal: OpLiteral::_1NNN, kind: OpKind::Flow }),
            ('2', _, _, _) => Ok(OpCode { value, literal: OpLiteral::_2NNN, kind: OpKind::Flow }),
            ('3', _, _, _) => Ok(OpCode { value, literal: OpLiteral::_3XNN, kind: OpKind::Cond }),
            ('4', _, _, _) => Ok(OpCode { value, literal: OpLiteral::_4XNN, kind: OpKind::Cond }),
            ('5', _, _, _) => Ok(OpCode { value, literal: OpLiteral::_5XY0, kind: OpKind::Cond }),
            ('6', _, _, _) => Ok(OpCode { value, literal: OpLiteral::_6XNN, kind: OpKind::Const }),
            ('7', _, _, _) => Ok(OpCode { value, literal: OpLiteral::_7XNN, kind: OpKind::Const }),
            ('8', _, _, '0') => Ok(OpCode { value, literal: OpLiteral::_8XY0, kind: OpKind::Assig }),
            ('8', _, _, '1') => Ok(OpCode { value, literal: OpLiteral::_8XY1, kind: OpKind::BitOp }),
            ('8', _, _, '2') => Ok(OpCode { value, literal: OpLiteral::_8XY2, kind: OpKind::BitOp }),
            ('8', _, _, '3') => Ok(OpCode { value, literal: OpLiteral::_8XY3, kind: OpKind::BitOp }),
            ('8', _, _, '4') => Ok(OpCode { value, literal: OpLiteral::_8XY4, kind: OpKind::Math }),
            ('8', _, _, '5') => Ok(OpCode { value, literal: OpLiteral::_8XY5, kind: OpKind::Math }),
            ('8', _, _, '6') => Ok(OpCode { value, literal: OpLiteral::_8XY6, kind: OpKind::BitOp }),
            ('8', _, _, '7') => Ok(OpCode { value, literal: OpLiteral::_8XY7, kind: OpKind::Math }),
            ('8', _, _, 'E') => Ok(OpCode { value, literal: OpLiteral::_8XYE, kind: OpKind::BitOp }),
            ('9', _, _, '0') => Ok(OpCode { value, literal: OpLiteral::_9XY0, kind: OpKind::Cond }),
            ('A', _, _, _) => Ok(OpCode { value, literal: OpLiteral::_ANNN, kind: OpKind::MEM }),
            ('B', _, _, _) => Ok(OpCode { value, literal: OpLiteral::_BNNN, kind: OpKind::Flow }),
            ('C', _, _, _) => Ok(OpCode { value, literal: OpLiteral::_CXNN, kind: OpKind::Rand }),
            ('D', _, _, _) => Ok(OpCode { value, literal: OpLiteral::_DXYN, kind: OpKind::Display }),
            ('E', _, '9', 'E') => Ok(OpCode { value, literal: OpLiteral::_EX9E, kind: OpKind::KeyOp }),
            ('E', _, 'A', '1') => Ok(OpCode { value, literal: OpLiteral::_EXA1, kind: OpKind::KeyOp }),
            ('F', _, '0', '7') => Ok(OpCode { value, literal: OpLiteral::_FX07, kind: OpKind::Timer }),
            ('F', _, '0', 'A') => Ok(OpCode { value, literal: OpLiteral::_FX0A, kind: OpKind::KeyOp }),
            ('F', _, '1', '5') => Ok(OpCode { value, literal: OpLiteral::_FX15, kind: OpKind::Timer }),
            ('F', _, '1', '8') => Ok(OpCode { value, literal: OpLiteral::_FX18, kind: OpKind::Sound }),
            ('F', _, '1', 'E') => Ok(OpCode { value, literal: OpLiteral::_FX1E, kind: OpKind::MEM }),
            ('F', _, '2', '9') => Ok(OpCode { value, literal: OpLiteral::_FX29, kind: OpKind::MEM }),
            ('F', _, '3', '3') => Ok(OpCode { value, literal: OpLiteral::_FX33, kind: OpKind::MEM }),
            ('F', _, '5', '5') => Ok(OpCode { value, literal: OpLiteral::_FX55, kind: OpKind::MEM }),
            ('F', _, '6', '5') => Ok(OpCode { value, literal: OpLiteral::_FX65, kind: OpKind::MEM }),
            _ => Err(OpCodeError::Unknown(value))
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::{OpCode, OpCodeError};
    use std::result::Result;
    use std::error::Error;

    #[test]
    fn test_opcode_parse() -> Result<(), Box<dyn Error>>{
        let mut counter = 0;
        for valid_u16 in u16::MIN..u16::MAX {
            let opcode: Result<OpCode, OpCodeError> = valid_u16.try_into();
            if !opcode.is_ok() {
                counter += 1;
            }
        }

        assert_eq!(counter, 13647);
        Ok(())
    }
}