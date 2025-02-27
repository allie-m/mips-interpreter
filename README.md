Mips Machine Code Interpreter
=

Made for CCS Computing 1B :)

Ingests .asm.out files (produced by running `spim -assemble [file.asm]`)

To run, execute
```
cargo run [path/to/file.asm.out]
```

IMPORTANT NOTE: .asm.out files don't know where "main" blocks are; the interpreter just starts from the beginning of the .text segment.
