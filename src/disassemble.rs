fn main() {
    let mut mem = [0; MAX_VALUE];
    let bytes = std::fs::read("data/challenge.bin").unwrap();
    for (n, byte_pair) in bytes.chunks_exact(2).enumerate() {
        mem[n] = u16::from_le_bytes([byte_pair[0], byte_pair[1]]);
    }

    let mut cursor: usize = 0;
    while cursor < MAX_VALUE {
        let idx = mem[cursor] as usize;
        if idx >= INSTRUCTIONS.len() {
            cursor += 1;
            continue;
        }
        let instr = &INSTRUCTIONS[idx];
        print!("{:>width$} ", cursor, width = 6);
        print!("{:>width$} ", instr.name, width = 4);
        cursor += 1;
        for _n in 0..instr.num_args {
            let value = mem[cursor];
            if (value as usize) < MAX_VALUE {
                print!("{} ", value);
            } else {
                print!("${} ", (value as usize) % MAX_VALUE);
            }
            cursor += 1;
        }
        print!("\n");
    }
}

const MAX_VALUE: usize = 32768;

struct Instr {
    name: &'static str,
    num_args: usize,
}

const INSTRUCTIONS: [Instr; 22] = [
    Instr {
        name: "halt",
        num_args: 0,
    },
    Instr {
        name: "set",
        num_args: 2,
    },
    Instr {
        name: "push",
        num_args: 1,
    },
    Instr {
        name: "pop",
        num_args: 1,
    },
    Instr {
        name: "eq",
        num_args: 3,
    },
    Instr {
        name: "gt",
        num_args: 3,
    },
    Instr {
        name: "jmp",
        num_args: 1,
    },
    Instr {
        name: "jt",
        num_args: 2,
    },
    Instr {
        name: "jf",
        num_args: 2,
    },
    Instr {
        name: "add",
        num_args: 3,
    },
    Instr {
        name: "mult",
        num_args: 3,
    },
    Instr {
        name: "mod",
        num_args: 3,
    },
    Instr {
        name: "and",
        num_args: 3,
    },
    Instr {
        name: "or",
        num_args: 3,
    },
    Instr {
        name: "not",
        num_args: 2,
    },
    Instr {
        name: "rmem",
        num_args: 2,
    },
    Instr {
        name: "wmem",
        num_args: 2,
    },
    Instr {
        name: "call",
        num_args: 1,
    },
    Instr {
        name: "ret",
        num_args: 0,
    },
    Instr {
        name: "out",
        num_args: 1,
    },
    Instr {
        name: "in",
        num_args: 1,
    },
    Instr {
        name: "noop",
        num_args: 0,
    },
];
