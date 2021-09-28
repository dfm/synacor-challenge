use std::io::{self, Read};
use std::vec::Vec;

fn main() {
    let mut arch = Arch::load("data/challenge.bin");
    arch.run();
}

struct Arch {
    terminate: bool,
    cursor: usize,
    program: [u16; 32768],
    reg: [u16; 8],
    stack: Vec<u16>,
}

struct Address {
    value: u16,
}

impl Address {
    fn is_valid(&self) -> bool {
        self.value < 32776
    }

    fn is_register(&self) -> bool {
        (32768 <= self.value) & (self.value < 32776)
    }

    fn to_register(&self) -> usize {
        if !self.is_register() {
            panic!("Invalid register");
        }
        self.value as usize - 32768
    }

    fn to_address(&self) -> usize {
        if self.is_register() | !self.is_valid() {
            panic!("Invalid address");
        }
        self.value as usize
    }
}

impl Arch {
    fn load(path: &str) -> Arch {
        let mut arch = Arch {
            terminate: false,
            cursor: 0,
            program: [0; 32768],
            reg: [0; 8],
            stack: vec![],
        };

        let bytes = std::fs::read(path).unwrap();
        for (n, byte_pair) in bytes.chunks_exact(2).enumerate() {
            arch.program[n] = u16::from_le_bytes([byte_pair[0], byte_pair[1]]);
        }
        arch
    }

    fn run(&mut self) {
        while !self.terminate {
            self.step();
        }
    }

    fn next(&mut self) -> u16 {
        let value = self.program[self.cursor];
        self.cursor += 1;
        value
    }

    fn next_value(&mut self) -> u16 {
        let value = self.next();
        let addr = Address { value: value };
        if addr.is_register() {
            self.reg[addr.to_register()]
        } else {
            value
        }
    }

    fn write_register(&mut self, index: u16, value: u16) {
        let addr = Address { value: index };
        self.reg[addr.to_register()] = value
    }

    fn read_memory(&mut self, index: u16) -> u16 {
        let addr = Address { value: index };
        if addr.is_register() {
            self.program[self.reg[addr.to_register()] as usize]
        } else {
            self.program[addr.to_address()]
        }
    }

    fn write_memory(&mut self, index: u16, value: u16) {
        let addr = Address { value: index };
        if addr.is_register() {
            self.program[self.reg[addr.to_register()] as usize] = value
        } else {
            self.program[addr.to_address()] = value
        }
    }

    fn step(&mut self) {
        let instr = self.next();
        // println!("{} => ", instr);
        match instr {
            0 => {
                // halt
                self.terminate = true;
                println!();
            }
            1 => {
                // set
                let a = self.next();
                let b = self.next_value();
                self.write_register(a, b);
            }
            2 => {
                // push
                let a = self.next_value();
                self.stack.push(a);
            }
            3 => {
                // pop
                let a = self.next();
                let value = self.stack.pop().unwrap();
                self.write_register(a, value);
            }
            4 => {
                // eq
                let a = self.next();
                let b = self.next_value();
                let c = self.next_value();
                self.write_register(a, if b == c { 1 } else { 0 });
            }
            5 => {
                // gt
                let a = self.next();
                let b = self.next_value();
                let c = self.next_value();
                self.write_register(a, if b > c { 1 } else { 0 });
            }
            6 => {
                // jmp
                self.cursor = self.next_value() as usize;
            }
            7 => {
                // jt
                let a = self.next_value();
                let b = self.next_value();
                if a != 0 {
                    self.cursor = b as usize;
                }
            }
            8 => {
                // jf
                let a = self.next_value();
                let b = self.next_value();
                if a == 0 {
                    self.cursor = b as usize;
                }
            }
            9 => {
                // add
                let a = self.next();
                let b = self.next_value();
                let c = self.next_value();
                self.write_register(a, (b + c) % 32768);
            }
            10 => {
                // mult
                let a = self.next();
                let b = self.next_value();
                let c = self.next_value();
                self.write_register(a, ((b as u64 * c as u64) % 32768) as u16);
            }
            11 => {
                // mod
                let a = self.next();
                let b = self.next_value();
                let c = self.next_value();
                self.write_register(a, b % c);
            }
            12 => {
                // and
                let a = self.next();
                let b = self.next_value();
                let c = self.next_value();
                self.write_register(a, b & c);
            }
            13 => {
                // or
                let a = self.next();
                let b = self.next_value();
                let c = self.next_value();
                self.write_register(a, b | c);
            }
            14 => {
                // not
                let a = self.next();
                let b = self.next_value();
                self.write_register(a, !b & !32768);
            }
            15 => {
                // rmem
                let a = self.next();
                let b = self.next();
                let b = self.read_memory(b);
                self.write_register(a, b);
            }
            16 => {
                // wmem
                let a = self.next();
                let b = self.next_value();
                self.write_memory(a, b);
            }
            17 => {
                // call
                self.stack.push(self.cursor as u16 + 1);
                let a = self.next_value();
                self.cursor = a as usize;
            }
            18 => {
                // ret
                match self.stack.pop() {
                    None => {
                        self.terminate;
                    }
                    Some(a) => {
                        self.cursor = a as usize;
                    }
                }
            }
            19 => {
                // out
                let a = self.next() as u8 as char;
                print!("{}", a);
            }
            20 => {
                // in
                let a = self.next();
                let mut buf: [u8; 1] = [0];
                io::stdin().read_exact(&mut buf).unwrap();
                self.write_register(a, buf[0] as u16);
            }
            21 => {}
            _ => panic!("Unrecogized instruction: {}", instr),
        }
    }
}
