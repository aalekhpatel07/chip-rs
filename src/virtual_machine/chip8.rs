use core::num;
use std::path::Path;

use rand::{rngs::ThreadRng, Rng};

use crate::data_structures::NibblePair;

use super::{
    Memory,
    AddressRegister,
    ProgramCounter,
    Stack,
    StackPointer, Keypad, OpCode, OpLiteral, Timer, Screen, DataRegisters,
    FontSet
};

#[derive(Debug, Default)]
pub struct Chip8 {
    memory: Memory,
    data_registers: DataRegisters,
    address_register: AddressRegister,
    program_counter: ProgramCounter,
    stack: Stack,
    stack_pointer: StackPointer,
    should_draw: bool,
    keypad: Keypad,
    delay_timer: Timer,
    sound_timer: Timer,
    screen: Screen,
    rng: ThreadRng,
}


pub fn draw_graphics() {

}


impl Chip8 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initialize(&mut self) {

        // Set program counter.
        self.program_counter.write(0x200).unwrap();

        // Load fontset.
        self.memory.load_font_data(&FontSet::default(), 0);
    }

    pub fn load_program<P: AsRef<Path>>(&mut self, path: P) {
        let program_offset = 512usize;
        if let Ok(program) = std::fs::read(path) {
            for (idx, item) in program.iter().enumerate() {
                self.memory[program_offset + idx] = *item;
            }
        }
    }

    fn fetch_opcode(&self) -> u16 {
        let current_pc = self.program_counter.read() as usize;
        ((self.memory[current_pc] as usize) << 8) as u16 | (self.memory[current_pc + 1] as u16)
    }

    fn apply_opcode(&mut self, opcode: u16) -> Result<(), Box<dyn std::error::Error>> {
        let maybe_opcode = OpCode::try_from(opcode);
        if maybe_opcode.is_err() {
            unreachable!("Only known opcodes may be applied.");
        }

        let opcode = maybe_opcode.unwrap();

        match opcode.literal {
            OpLiteral::_0NNN => {
                // Call machine code routine (RCA 1802 for COSMAC VIP) at address NNN. Not necessary for most ROMs.
            },
            OpLiteral::_00E0 => {
                // Clear the screen.
                self.screen.clear();
                self.program_counter.step(2)?;
            },
            OpLiteral::_00EE => {
                // Returns from a subroutine.

                self.stack_pointer -= 1;
                let previous_program_counter = self.stack[self.stack_pointer as usize];
                
                // Clear the stack entry.
                self.stack[self.stack_pointer as usize] = 0;

                // Restore the program counter.
                self.program_counter.write(previous_program_counter & 0x0FFF)?;
                self.program_counter.step(2)?;


            },
            OpLiteral::_1NNN => {
                // Jumps to address NNN.
                self.program_counter.write(opcode.value & 0x0FFF)?;

            }
            OpLiteral::_2NNN => {
                // Calls subroutine at NNN.

                // Save current program counter.
                self.stack[self.stack_pointer as usize] = self.program_counter.read();
                self.stack_pointer += 1;

                // Move program counter to the subroutine's address.
                self.program_counter.write(opcode.value & 0x0FFF)?;

            },
            OpLiteral::_3XNN => {
                // Skips the next instruction if VX equals NN (usually the next instruction is a jump to skip a code block.)
                let register_identifier = ((opcode.value & 0x0F00) >> 8) as u8;
                let quad: NibblePair = register_identifier.into();
                let register_identifier = quad.high.to_hex_char();
                let data = (opcode.value & 0x00FF) as u8;
                
                self.program_counter.step(2)?;

                if let Ok(value_in_register) = self.data_registers.read(register_identifier) {
                    if value_in_register == data {
                        // Skip next instruction if (Vx == NN).
                        self.program_counter.step(2)?;
                    }
                }
            },
            OpLiteral::_4XNN => {
                // Skips the next instruction if VX equals NN (usually the next instruction is a jump to skip a code block.)
                let register_identifier = ((opcode.value & 0x0F00) >> 8) as u8;

                let quad: NibblePair = register_identifier.into();
                let register_identifier = quad.low.to_hex_char();
                let data = (opcode.value & 0x00FF) as u8;
                
                self.program_counter.step(2)?;

                if let Ok(value_in_register) = self.data_registers.read(register_identifier) {
                    if value_in_register != data {
                        // Skip next instruction if (Vx != NN).
                        self.program_counter.step(2)?;
                    }
                }
            },
            OpLiteral::_5XY0 => {
                // Skips the next instruction if VX equals Vy (usually the next instruction is a jump to skip a code block.)
                let register_x = ((opcode.value & 0x0F00) >> 8) as u8;
                let register_y = ((opcode.value & 0x00F0) >> 4) as u8;
                
                let data_x = self.data_registers.read(NibblePair::from(register_x).low.to_hex_char())?;
                let data_y = self.data_registers.read(NibblePair::from(register_y).low.to_hex_char())?;
                
                self.program_counter.step(2)?;

                if data_x == data_y {
                    self.program_counter.step(2)?;
                }
            },
            OpLiteral::_6XNN => {
                // Sets Vx to NN.

                let nn = (opcode.value & 0x00FF) as u8;
                
                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();

                self.data_registers.write(register_x, nn)?;
                self.program_counter.step(2)?;
            },
            OpLiteral::_7XNN => {
                // Adds NN to Vx (carry flag is not changed).
                let nn = (opcode.value & 0x00FF) as u8;
                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();

                self.data_registers.write(
                    register_x, 
                    self.data_registers.read(register_x)? + nn
                )?;
                self.program_counter.step(2)?;
            },
            OpLiteral::_8XY0 => {
                // Sets Vx to the value of Vy.
                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                let register_y = NibblePair::from(((opcode.value & 0x00F0) >> 4) as u8).low.to_hex_char();

                let data_y = self.data_registers.read(register_y)?;
                self.data_registers.write(register_x, data_y)?;
                
                self.program_counter.step(2)?;
            },
            OpLiteral::_8XY1 => {
                // Sets Vx to the value of Vx | Vy.
                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                let register_y = NibblePair::from(((opcode.value & 0x00F0) >> 4) as u8).low.to_hex_char();

                let data_y = self.data_registers.read(register_y)?;
                let data_x = self.data_registers.read(register_x)?;

                self.data_registers.write(register_x, data_y | data_x)?;
                
                self.program_counter.step(2)?;
            },
            OpLiteral::_8XY2 => {
                // Sets Vx to the value of Vx & Vy.
                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                let register_y = NibblePair::from(((opcode.value & 0x00F0) >> 4) as u8).low.to_hex_char();

                let data_y = self.data_registers.read(register_y)?;
                let data_x = self.data_registers.read(register_x)?;

                self.data_registers.write(register_x, data_y & data_x)?;
                
                self.program_counter.step(2)?;
            },
            OpLiteral::_8XY3 => {
                // Sets Vx to the value of Vx ^ Vy.
                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                let register_y = NibblePair::from(((opcode.value & 0x00F0) >> 4) as u8).low.to_hex_char();

                let data_y = self.data_registers.read(register_y)?;
                let data_x = self.data_registers.read(register_x)?;

                self.data_registers.write(register_x, data_y ^ data_x)?;
                
                self.program_counter.step(2)?;
            },
            OpLiteral::_8XY4 => {

                // Adds Vy to Vx. Vf is set to 1 when there's a carry, and to 0 when there is not.
                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                let register_y = NibblePair::from(((opcode.value & 0x00F0) >> 4) as u8).low.to_hex_char();

                let data_y = self.data_registers.read(register_y)?;
                let data_x = self.data_registers.read(register_x)?;
                
                // The sum exceeds u8::MAX, set the carry.
                if (data_x as u16 + data_y as u16) > (u8::MAX as u16) {
                    self.data_registers.write('f', 1)?;
                } 
                else {
                    self.data_registers.write('f', 0)?;
                }

                self.data_registers.write(register_x, data_y + data_x)?;
                
                self.program_counter.step(2)?;
            },
            OpLiteral::_8XY5 => {

                // Subtract Vy from Vx. Vf is set to 0 when there's a borrow, and to 1 when there is not.

                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                let register_y = NibblePair::from(((opcode.value & 0x00F0) >> 4) as u8).low.to_hex_char();

                let data_y = self.data_registers.read(register_y)?;
                let data_x = self.data_registers.read(register_x)?;
                
                if data_x as u16 > (0xFF - data_y as u16) {
                    self.data_registers.write('f', 1)?;
                } 
                else {
                    self.data_registers.write('f', 0)?;
                }

                self.data_registers.write(register_x, data_x - data_y)?;                
                self.program_counter.step(2)?;
            },
            OpLiteral::_8XY6 => {
                // Stores the least significant bit of Vx in Vf and then shift Vx to the right by 1.

                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                let data_x = self.data_registers.read(register_x)?;
                self.data_registers.write('f', data_x & 0b1u8)?;
                self.data_registers.write(register_x, data_x >> 1)?;

                self.program_counter.step(2)?;
            },
            OpLiteral::_8XY7 => {

                // Subtract Vx from Vy and assign to Vx. Vf is set to 0 when there's a borrow, and to 1 when there is not.

                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                let register_y = NibblePair::from(((opcode.value & 0x00F0) >> 4) as u8).low.to_hex_char();

                let data_y = self.data_registers.read(register_y)?;
                let data_x = self.data_registers.read(register_x)?;
                
                if data_y as u16 > (0xFF - data_x as u16) {
                    self.data_registers.write('f', 1)?;
                } 
                else {
                    self.data_registers.write('f', 0)?;
                }

                self.data_registers.write(register_x, data_y - data_x)?;                
                self.program_counter.step(2)?;
            },
            OpLiteral::_8XYE => {
                // Stores the most significant bit of Vx in Vf and then shift Vx to the left by 1.

                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                let data_x = self.data_registers.read(register_x)?;
                self.data_registers.write('f', data_x & (1u8 << 7))?;
                self.data_registers.write(register_x, data_x << 1)?;

                self.program_counter.step(2)?;
            },
            OpLiteral::_9XY0 => {
                // Skip the next instruction if Vx does not equal Vy. (Usually the instruction is a jump to skip a code block)

                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                let register_y = NibblePair::from(((opcode.value & 0x00F0) >> 4) as u8).low.to_hex_char();

                let data_y = self.data_registers.read(register_y)?;
                let data_x = self.data_registers.read(register_x)?;

                self.program_counter.step(2)?;

                if data_x != data_y {
                    self.program_counter.step(2)?;
                }
            },
            OpLiteral::_ANNN => {
                // Sets the address_register to NNN.

                // Extract the last three quads.
                let value = opcode.value & 0x0FFF;
                self.address_register.write(value)?;
                self.program_counter.step(2)?;
            },
            OpLiteral::_BNNN => {
                // Jumps to the address NNN plus V0.
                let value = opcode.value & 0x0FFF;
                self.program_counter.write((value + self.data_registers.read('0')? as u16) & 0x0FFF as u16)?;
            }
            OpLiteral::_CXNN => {
                // Sets Vx to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
                let nn = (opcode.value & 0x00FF) as u8;

                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                self.data_registers.write(register_x, nn & self.rng.gen::<u8>())?;
                self.program_counter.step(2)?;
            },
            OpLiteral::_DXYN => {
                // Draws a sprite at coordinate (Vx, Vy) that has a width of 8 pixels and height of N pixels. 
                // Each row of 8 pixels is read as bit-coded starting from memory location I; I value does not change
                // after the execution of this instruction. Vf is set to 1 if any screen pixels are flipped from set to unset when
                // the sprite is drawn, and to 0 if that does not happen.

                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                let register_y = NibblePair::from(((opcode.value & 0x00F0) >> 4) as u8).low.to_hex_char();

                let data_x = self.data_registers.read(register_x)?;
                let data_y = self.data_registers.read(register_y)?;

                let height = NibblePair::from((opcode.value & 0x000F) as u8).low;
                
                self.data_registers.write_idx(15, 0)?;

                let num_rows = height.to_u8();


                // FIXME: Rework this display logic because it is has strange
                // out of bound access panics rn.
                for yline in 0..num_rows as usize {

                    let current_address = self.address_register.read() as usize;
                    let pixel = self.memory[current_address + yline];

                    for xline in 0..8 {
                        if ((pixel as u16) & (0x80 >> xline)) != 0 {
                            if self.screen[(data_x as usize + xline + ((data_y as usize + yline) * 64))] {
                                // That pixel was already on.
                                self.data_registers.write_idx(15, 1)?;
                            }
                            let current_value = self.screen[(data_x as usize + xline + ((data_y as usize + yline) * 64))];
                            self.screen[(data_x as usize + xline + ((data_y as usize + yline) * 64))] = !current_value;
                        }
                    }
                }


                self.should_draw = true;
                self.program_counter.step(2)?;
            },
            OpLiteral::_EX9E => {
                // Skips the next instruction if the key stored in Vx is pressed (usually the next instruction is a jump to skip a code block).

                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                let data_x = self.data_registers.read(register_x)?;

                self.program_counter.step(2)?;

                if self.keypad.is_pressed(data_x) {
                    self.program_counter.step(2)?;
                }
            },
            OpLiteral::_EXA1 => {
                // Skips the next instruction if the key stored in Vx is NOT pressed (usually the next instruction is a jump to skip a code block).

                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                let data_x = self.data_registers.read(register_x)?;

                self.program_counter.step(2)?;

                if !self.keypad.is_pressed(data_x) {
                    self.program_counter.step(2)?;
                }
            },
            OpLiteral::_FX07 => {
                // Set Vx to the value of the delay timer.
                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                self.data_registers.write(register_x, self.delay_timer.value())?;
                self.program_counter.step(2)?;

            },
            OpLiteral::_FX0A => {
                // A key press is awaited, and the stored in Vx (blocking operation, all instruction halted until next key event).
                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();

                if let Some(key_event) = self.keypad.read() {
                    self.data_registers.write(register_x, key_event & 0x0F)?;
                }
                self.program_counter.step(2)?;

            }
            OpLiteral::_FX15 => {
                // Set the delay timer to Vx.

                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                self.delay_timer.reset(self.data_registers.read(register_x)?);

                self.program_counter.step(2)?;
            },
            OpLiteral::_FX18 => {
                // Set the sound timer to Vx.

                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                self.sound_timer.reset(self.data_registers.read(register_x)?);

                self.program_counter.step(2)?;
            },
            OpLiteral::_FX1E => {
                // Adds Vx to I. Vf is unaffected.
                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                let data_x = self.data_registers.read(register_x)?;

                self.address_register.write(
                    (self.address_register.read() as usize + data_x as usize) as u16 
                    & 0x0FFFu16
                )?;

                self.program_counter.step(2)?;
            },
            OpLiteral::_FX29 => {
                // Sets I to the location of the sprite for the character in Vx. Characters 0-F (in hexadecimal) are represented by a 4x5 font.

                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                let data_x = self.data_registers.read(register_x)? & 0x0F;

                let sprite_location = 4096 - 80 + (data_x as usize * 5);
                self.address_register.write(sprite_location as u16)?;

                self.program_counter.step(2)?;

            },
            OpLiteral::_FX33 => {
                // Store the binary-coded decimal representation of VX, with the hundreds digit in memory at location in I,
                // the tens digit at location I + 1, and the ones digit at location I + 2.

                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                let data_x = self.data_registers.read(register_x)? & 0x0F;

                self.memory[self.address_register.read() as usize] = ((data_x as usize) / 100) as u8;
                self.memory[self.address_register.read() as usize + 1] = (((data_x as usize) / 10) % 10) as u8;
                self.memory[self.address_register.read() as usize + 2] = (((data_x as usize) % 100) % 10) as u8;
                
                self.program_counter.step(2)?;

            },
            OpLiteral::_FX55 => {
                // Stores from V0 to Vx (including Vx) in memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.

                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                let register_breakpoint = usize::from_str_radix(&String::from(register_x), 16)?;
                let mut current_address = self.address_register.read() as usize;

                for register_idx in 0..=register_breakpoint {

                    let to_store = self.data_registers.read_idx(register_idx)?;
                    self.memory[current_address] = to_store;
                    current_address += 1;

                }
                self.program_counter.step(2)?;
            },
            OpLiteral::_FX65 => {
                // Fills from V0 to Vx (including Vx) with values from memory, starting at address I. The offset from I is increased by 1 for each value read, but I itself is left unmodified.

                let register_x = NibblePair::from(((opcode.value & 0x0F00) >> 8) as u8).low.to_hex_char();
                let register_breakpoint = usize::from_str_radix(&String::from(register_x), 16)?;
                let mut current_address = self.address_register.read() as usize;

                for register_idx in 0..=register_breakpoint {

                    let to_fill = self.memory[current_address];
                    self.data_registers.write_idx(register_idx, to_fill)?;

                    current_address += 1;
                }
                self.program_counter.step(2)?;
            }
        }

        Ok(())
    }

    fn step(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        let opcode = self.fetch_opcode();
        println!("program counter: {:04x}", self.program_counter.read() - 0x200);
        println!("will apply opcode: {:#?}", OpCode::try_from(opcode)?);
        self.apply_opcode(opcode)?;
        println!("after application: program counter: {:04x}", self.program_counter.read() - 0x200);

        if self.delay_timer.value() > 0 {
            self.delay_timer.tick();
        }
        if self.sound_timer.value() > 0 {
            if self.sound_timer.value() == 1 {
                println!(
                    "BEEP!\n"
                );
            }
            self.sound_timer.tick();
        }
        Ok(())

    }
    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        loop {
            self.step()?;

            if self.should_draw {
                println!("{}", self.screen);
                self.should_draw = false;
            }
        }
    }
}