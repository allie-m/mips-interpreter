// ingests the .asm.out yielded by `spim -assemble [filename]`

use std::path::Path;

use crate::interpreter::InterpreterState;

// will crash gracelessly & opaquely if provided with a file
// not conforming perfectly to what i expect in a .asm.out artifact
pub fn load_asm(path: &Path) -> InterpreterState {
    let contents = std::fs::read_to_string(path)
        .expect(&format!("Error reading from .asm.out file at {:?}", path));
    let mut lines = contents.lines();
    let instruction_mem_start = u32::from_str_radix(
        &lines.next().unwrap().split("#").nth(1).unwrap()[1..]
            .split(" ")
            .next()
            .unwrap()[2..],
        16,
    )
    .unwrap();
    let instructions = lines.next().unwrap();
    let instruction_mem: Vec<u32> = instructions[6..]
        .split(", ")
        .map(|s| u32::from_str_radix(&s[2..], 16).unwrap())
        .collect();
    let data_mem_start = usize::from_str_radix(
        &lines.next().unwrap().split("#").nth(1).unwrap()[1..]
            .split(" ")
            .next()
            .unwrap()[2..],
        16,
    )
    .unwrap();
    let data = lines.next().unwrap();
    let data_mem: Vec<u8> = data[6..]
        .split(", ")
        .filter(|s| !s.is_empty())
        .map(|s| u32::from_str_radix(&s[2..], 16).unwrap().to_le_bytes())
        .flatten()
        .collect();
    InterpreterState {
        program_counter: 0,
        register_file: [0; 32],
        instruction_mem,
        data_mem,
        instruction_mem_start,
        data_mem_start,
    }
}
