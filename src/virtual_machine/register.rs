use std::{num::ParseIntError, ops::{DerefMut, Index, IndexMut, Deref}};

use crate::data_structures::HexKeyMap;
use thiserror::Error;
use crossterm::event;


#[derive(Debug, Error)]
pub enum RegisterError {
    #[error("Unknown register identifier provided where a single word was expected (i.e. [0-F]).")]
    UnknownIdentifier(String),
    #[error("")]
    AddressValueLargerThan12Bytes,
    #[error("Register index out of bounds.")]
    RegisterIndexOutOfBounds
}

#[derive(Debug, Error)]
pub enum KeyError {
    #[error("Unable to recognize key input: `{0}`")]
    UnknownKeyInput(char)
}


impl From<ParseIntError> for RegisterError {
    fn from(err: ParseIntError) -> Self {
        Self::UnknownIdentifier(err.to_string())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DataRegisters([u8; 16]);

impl DataRegisters {

    pub fn write(&mut self, register_identifier: char, value: u8) -> Result<u8, RegisterError> {
        let index = usize::from_str_radix(&String::from(register_identifier), 16)?;
        self.write_idx(index, value)
    }

    pub fn read(&self, register_identifier: char) -> Result<u8, RegisterError> {
        let index = usize::from_str_radix(&String::from(register_identifier), 16)?;
        self.read_idx(index)
    }

    pub fn read_idx(&self, register_idx: usize) -> Result<u8, RegisterError> {
        if register_idx >= 16 {
            return Err(RegisterError::RegisterIndexOutOfBounds);
        }
        Ok(self.0[register_idx])
    }

    pub fn write_idx(&mut self, register_idx: usize, value: u8) -> Result<u8, RegisterError> {
        if register_idx >= 16 {
            return Err(RegisterError::RegisterIndexOutOfBounds);
        }
        let previous_value = self.0[register_idx];
        self.0[register_idx] = value;

        Ok(previous_value)

    }
}

impl Default for DataRegisters {
    fn default() -> Self {
        Self([0; 16])
    }
}

#[derive(Debug, Clone)]
pub struct AddressRegister(u16);

impl AddressRegister {
    pub fn write(&mut self, value: u16) -> Result<(), RegisterError> {
        self.0 = value & (0x0FFFu16);
        Ok(())
    }
    pub fn step(&mut self, size: usize) -> Result<(), RegisterError> {
        let prev_value = self.read() as usize;
        self.write((prev_value + size) as u16)
    }

    pub fn read(&self) -> u16 {
        self.0
    }
}

impl Default for AddressRegister {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Debug)]
pub struct ProgramCounter(AddressRegister);

impl Deref for ProgramCounter {
    type Target = AddressRegister;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ProgramCounter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl Default for ProgramCounter {
    fn default() -> Self {
        Self(AddressRegister(0x200))
    }
}

impl std::fmt::Display for ProgramCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04x}", self.read())
    }
}

#[derive(Debug)]
pub struct Screen([bool; 64 * 32]);

impl Default for Screen {
    fn default() -> Self {
        Self([false; 64 * 32])
    }
}

impl Screen {
    pub fn clear(&mut self) {
        for i in 0usize..(64 * 32) {
            self.0[i] = false;
        }
    }
}

impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const NUM_ROWS: usize = 32;
        const NUM_COLS: usize = 64;

        let mut s = String::from("");

        for row_idx in 0..NUM_ROWS {
            for col_idx in 0..NUM_COLS {
                let coordinate = NUM_ROWS * row_idx + col_idx;
                s += &format!("{:08b}", if self.0[coordinate] { 1 } else { 0 });
            }
            s += "\n";
        }
        s = s.replace("0", " ");
        s = s.replace("1", "*");

        writeln!(f, "{}", s)
    }
}

impl Index<usize> for Screen {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for Screen {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}


#[derive(Debug, Default)]
pub struct Timer(u8);
impl Timer {
    pub fn new(value: u8) -> Self {
        Self(value)
    }
    pub fn reset(&mut self, value: u8) {
        self.0 = value
    }
    pub fn value(&self) -> u8 {
        self.0
    }
    pub fn tick(&mut self) {
        self.0 = self.0 - 1;
    }
}

pub type Stack = [u16; 16];

pub type StackPointer = u16;

#[derive(Debug)]
pub struct Keypad {
    _inner: [bool; 16],
    pub keymap: HexKeyMap,
}

impl Default for Keypad {
    fn default() -> Self {
        Self { 
            _inner: [false; 16],
            keymap: HexKeyMap::default()
        }
    }
}

impl Keypad {
    pub fn press(&mut self, key_identifier: char) {
        if let Ok(index) = usize::from_str_radix(&String::from(key_identifier), 16) {
            self._inner[index] = true;
        }
    }
    pub fn unpress(&mut self, key_identifier: char) {
        if let Ok(index) = usize::from_str_radix(&String::from(key_identifier), 16) {
            self._inner[index] = false;
        }
    }
    pub fn is_pressed(&self, key: u8) -> bool {
        let key = (key & 0x0Fu8) as usize;
        self._inner[key]
    }
    pub fn read(&mut self) -> Option<u8> {
        match event::read() {
            Ok(event::Event::Key(k)) => {
                if let Some(mapped_value) = self.keymap.0.get(&k.code) {
                    return Some(*mapped_value as u8);
                }
            },
            _ => {
            }
        }
        None
    }
}