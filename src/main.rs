mod instruction;
mod interpreter;
mod parse;

fn main() {
    let mut args = std::env::args();
    let _ = args.next().unwrap();
    let path = args
        .next()
        .expect("mips-interpreter requires an argument providing a path to a .asm.out file");
    let path: std::path::PathBuf = path.into();
    let mut interp = parse::load_asm(&path);
    // set the stack pointer to halfway through the memory
    // arbitrary choice to avoid overriding the global memory
    // since the file format doesn't tell me where to put the stack segment
    interp.register_file[29] = (interp.data_mem_start + interp.data_mem.len()/2) as u32;
    while interp
        .process_next_instruction()
        .unwrap_or_else(|err| panic!("Interpreter error: {:?};\n{:?}", err, interp))
    {}
}
