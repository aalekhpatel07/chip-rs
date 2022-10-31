pub struct FontSet(pub [u8; 16 * 5]);

impl Default for FontSet {
    fn default() -> Self {
        let fontset = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
        Self(fontset)
    }
}

impl FontSet {
    pub fn show_digit(&self, digit: usize) -> Result<String, Box<dyn std::error::Error>> {
        if digit >= 16 {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "out of range.")));
        }
        
        let start_index = digit * 5;

        let mut s = String::from("");
        for shift in 0..5usize {
            s += &format!("{:04b}\n", self.0[start_index + shift] >> 4);
        }
        s = s.replace("0", " ");
        s = s.replace("1", "*");

        Ok(s)
    }

    pub fn show_all_digits(&self) -> Result<(), Box<dyn std::error::Error>> {
        for digit in 0..16usize {
            let s = self.show_digit(digit)?;
            println!("{}", s);
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn default_fontset() -> Result<(), Box<dyn std::error::Error>>{
        let fontset = FontSet::default();
        fontset.show_all_digits()?;
        Ok(())
    }
}