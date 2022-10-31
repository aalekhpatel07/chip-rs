use std::path::{Path, PathBuf};
use std::error::Error;

use clap::Parser;


#[derive(Parser, Debug)]
pub struct Args {
    #[
        arg(
            short = 'p', 
            long, 
            help="The path to the chip-8 program to run.",
        )
    ]
    program: Option<PathBuf>
}


fn main() -> Result<(), Box<dyn Error>>{
    let args = Args::parse();
    let mut my_chip = chip8_emulator::virtual_machine::Chip8::new();
    if args.program.is_none() {
        my_chip.load_program("pong2.c8");
    } else {
        my_chip.load_program(args.program.unwrap());
    }
    my_chip.initialize();
    my_chip.start()?;

    Ok(())
}
