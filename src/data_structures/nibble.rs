use core::fmt::Display;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Nibble {
    B0000 = 0b0000,
    B0001 = 0b0001,
    B0010 = 0b0010,
    B0011 = 0b0011,
    B0100 = 0b0100,
    B0101 = 0b0101,
    B0110 = 0b0110,
    B0111 = 0b0111,
    B1000 = 0b1000,
    B1001 = 0b1001,
    B1010 = 0b1010,
    B1011 = 0b1011,
    B1100 = 0b1100,
    B1101 = 0b1101,
    B1110 = 0b1110,
    B1111 = 0b1111,
}

impl Display for Nibble {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let value =  match self {
            Self::B0000 => "0000",
            Self::B0001 => "0001",
            Self::B0010 => "0010",
            Self::B0011 => "0011",
            Self::B0100 => "0100",
            Self::B0101 => "0101",
            Self::B0110 => "0110",
            Self::B0111 => "0111",
            Self::B1000 => "1000",
            Self::B1001 => "1001",
            Self::B1010 => "1010",
            Self::B1011 => "1011",
            Self::B1100 => "1100",
            Self::B1101 => "1101",
            Self::B1110 => "1110",
            Self::B1111 => "1111",
        };
        write!(f, "{}", value)
    }
}

impl Nibble {
    pub fn to_hex_char(&self) -> char {
        match self {
            Self::B0000 => '0',
            Self::B0001 => '1',
            Self::B0010 => '2',
            Self::B0011 => '3',
            Self::B0100 => '4',
            Self::B0101 => '5',
            Self::B0110 => '6',
            Self::B0111 => '7',
            Self::B1000 => '8',
            Self::B1001 => '9',
            Self::B1010 => 'A',
            Self::B1011 => 'B',
            Self::B1100 => 'C',
            Self::B1101 => 'D',
            Self::B1110 => 'E',
            Self::B1111 => 'F',
        }
    }
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::B0000 => 0,
            Self::B0001 => 1,
            Self::B0010 => 2,
            Self::B0011 => 3,
            Self::B0100 => 4,
            Self::B0101 => 5,
            Self::B0110 => 6,
            Self::B0111 => 7,
            Self::B1000 => 8,
            Self::B1001 => 9,
            Self::B1010 => 10,
            Self::B1011 => 11,
            Self::B1100 => 12,
            Self::B1101 => 13,
            Self::B1110 => 14,
            Self::B1111 => 15,
        }
    }

    pub fn from_hex_char(hex_char: char) -> Self {
        match hex_char {
            '0' => Self::B0000,
            '1' => Self::B0001,
            '2' => Self::B0010,
            '3' => Self::B0011,
            '4' => Self::B0100,
            '5' => Self::B0101,
            '6' => Self::B0110,
            '7' => Self::B0111,
            '8' => Self::B1000,
            '9' => Self::B1001,
            'A' => Self::B1010,
            'B' => Self::B1011,
            'C' => Self::B1100,
            'D' => Self::B1101,
            'E' => Self::B1110,
            'F' => Self::B1111,
            _ => unreachable!("Provided a char that is not [0-F].")
        }
    }
}


#[derive(Debug)]
pub struct NibblePair {
    pub low: Nibble,
    pub high: Nibble
}

impl From<(Nibble, Nibble)> for NibblePair {
    fn from(pair: (Nibble, Nibble)) -> Self {
        NibblePair { low: pair.1, high: pair.0 }
    }
}

impl From<u8> for NibblePair {
    fn from(full_byte: u8) -> Self {
        let high = match full_byte >> 4 {
            0b0000 => Nibble::B0000,
            0b0001 => Nibble::B0001,
            0b0010 => Nibble::B0010,
            0b0011 => Nibble::B0011,
            0b0100 => Nibble::B0100,
            0b0101 => Nibble::B0101,
            0b0110 => Nibble::B0110,
            0b0111 => Nibble::B0111,
            0b1000 => Nibble::B1000,
            0b1001 => Nibble::B1001,
            0b1010 => Nibble::B1010,
            0b1011 => Nibble::B1011,
            0b1100 => Nibble::B1100,
            0b1101 => Nibble::B1101,
            0b1110 => Nibble::B1110,
            0b1111 => Nibble::B1111,
            _ => unreachable!("Theoretically there are only 16 possibilies for the first word in this byte.")
        };

        let low = match full_byte & 0b1111u8 {
            0b0000 => Nibble::B0000,
            0b0001 => Nibble::B0001,
            0b0010 => Nibble::B0010,
            0b0011 => Nibble::B0011,
            0b0100 => Nibble::B0100,
            0b0101 => Nibble::B0101,
            0b0110 => Nibble::B0110,
            0b0111 => Nibble::B0111,
            0b1000 => Nibble::B1000,
            0b1001 => Nibble::B1001,
            0b1010 => Nibble::B1010,
            0b1011 => Nibble::B1011,
            0b1100 => Nibble::B1100,
            0b1101 => Nibble::B1101,
            0b1110 => Nibble::B1110,
            0b1111 => Nibble::B1111,
            _ => unreachable!("Theoretically there are only 16 possibilies for the second word in this byte.")
        };

        Self {
            high, 
            low
        }
    }
}


impl From<NibblePair> for u8 {
    fn from(pair: NibblePair) -> Self {
        let concatenated_str = format!("{}{}", pair.high, pair.low);
        u8::from_str_radix(&concatenated_str, 2).unwrap()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn split_word_all_bytes() {
        for full_byte in 0u8..=u8::MAX {
            let observed: NibblePair = full_byte.into();
            let full_byte_as_str = format!("{:08b}", full_byte);

            let expected_high_word = &full_byte_as_str[0..4];
            let expected_low_word = &full_byte_as_str[4..8];

            assert_eq!(expected_high_word, observed.high.to_string());
            assert_eq!(expected_low_word, observed.low.to_string());

            let back_as_byte: u8 = observed.into();
            assert_eq!(back_as_byte, full_byte);
        }
    }
}