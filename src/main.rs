use std::io::{self, Read};
use std::vec::Vec;

fn main() {
    let mut arch = Arch::load("data/challenge.bin");
    arch.run();
}

// ****
// MATH
// ****
const MAX_VALUE: usize = 32768;

fn wrap(a: usize) -> u16 {
    (a % MAX_VALUE) as u16
}

fn add(a: u16, b: u16) -> u16 {
    wrap((a as usize) + (b as usize))
}

fn mult(a: u16, b: u16) -> u16 {
    wrap((a as usize) * (b as usize))
}

// ***********
// OUR MACHINE
// ***********

struct Arch {
    exit: bool,
    cursor: usize,
    mem: [u16; MAX_VALUE],
    reg: [u16; 8],
    stack: Vec<u16>,
}

impl Arch {
    fn load(path: &str) -> Arch {
        let mut arch = Arch {
            exit: false,
            cursor: 0,
            mem: [0; 32768],
            reg: [0; 8],
            stack: vec![],
        };

        let bytes = std::fs::read(path).unwrap();
        for (n, byte_pair) in bytes.chunks_exact(2).enumerate() {
            arch.mem[n] = u16::from_le_bytes([byte_pair[0], byte_pair[1]]);
        }
        arch
    }

    fn run(&mut self) {
        while !self.exit {
            self.step();
        }
    }

    fn next(&mut self) -> u16 {
        let value = self.mem[self.cursor];
        self.cursor += 1;
        value
    }

    fn next_value(&mut self) -> u16 {
        let value = self.next();
        self.read(value)
    }

    fn read(&self, index: u16) -> u16 {
        let idx = index as usize;
        if idx < MAX_VALUE {
            index
        } else {
            self.reg[idx % MAX_VALUE]
        }
    }

    fn write(&mut self, index: u16, value: u16) {
        let idx = index as usize;
        if idx < MAX_VALUE {
            self.mem[idx] = value;
        } else {
            self.reg[idx % MAX_VALUE] = value;
        }
    }

    fn step(&mut self) {
        let instr = self.next();
        // println!("{} => ", instr);
        match instr {
            0 => {
                // halt
                self.exit = true;
                println!();
            }
            1 => {
                // set
                let a = self.next();
                let b = self.next_value();
                self.write(a, b);
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
                self.write(a, self.read(value));
            }
            4 => {
                // eq
                let a = self.next();
                let b = self.next_value();
                let c = self.next_value();
                self.write(a, if b == c { 1 } else { 0 });
            }
            5 => {
                // gt
                let a = self.next();
                let b = self.next_value();
                let c = self.next_value();
                self.write(a, if b > c { 1 } else { 0 });
            }
            6 => {
                // jmp
                self.cursor = self.next_value() as usize;
            }
            7 => {
                // jt
                let a = self.next();
                let b = self.next();
                if self.read(a) != 0 {
                    self.cursor = self.read(b) as usize;
                }
            }
            8 => {
                // jf
                let a = self.next();
                let b = self.next();
                if self.read(a) == 0 {
                    self.cursor = self.read(b) as usize;
                }
            }
            9 => {
                // add
                let a = self.next();
                let b = self.next_value();
                let c = self.next_value();
                self.write(a, add(b, c));
            }
            10 => {
                // mult
                let a = self.next();
                let b = self.next_value();
                let c = self.next_value();
                self.write(a, mult(b, c));
            }
            11 => {
                // mod
                let a = self.next();
                let b = self.next_value();
                let c = self.next_value();
                self.write(a, b % c);
            }
            12 => {
                // and
                let a = self.next();
                let b = self.next_value();
                let c = self.next_value();
                self.write(a, b & c);
            }
            13 => {
                // or
                let a = self.next();
                let b = self.next_value();
                let c = self.next_value();
                self.write(a, b | c);
            }
            14 => {
                // not
                let a = self.next();
                let b = self.next_value();
                self.write(a, (MAX_VALUE as u16 - 1) ^ b);
            }
            15 => {
                // rmem
                let a = self.next();
                let b = self.next_value();
                let b = self.mem[b as usize];
                self.write(a, b);
            }
            16 => {
                // wmem
                let a = self.next_value();
                let b = self.next_value();
                self.mem[a as usize] = b;
            }
            17 => {
                // call
                let a = self.next_value();
                self.stack.push(self.cursor as u16);
                self.cursor = a as usize;
            }
            18 => {
                // ret
                match self.stack.pop() {
                    None => {
                        self.exit = true;
                    }
                    Some(a) => {
                        self.cursor = self.read(a) as usize;
                    }
                }
            }
            19 => {
                // out
                let a = self.next_value() as u8 as char;
                print!("{}", a);
            }
            20 => {
                // in
                let a = self.next();
                let mut buf: [u8; 1] = [0];
                io::stdin().read_exact(&mut buf).unwrap();
                self.write(a, buf[0] as u16);
            }
            21 => {
                // noop
            }
            _ => panic!("Unrecogized instruction: {}", instr),
        }
    }
}
