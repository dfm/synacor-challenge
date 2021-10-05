use std::collections::VecDeque;
use std::io::{self, Write};
use std::vec::Vec;

fn main() {
    let mut dbg = Debugger {
        machine: Arch::load("data/challenge.bin"),
    };
    dbg.prompt();
}

struct Debugger {
    machine: Arch,
}

impl Debugger {
    fn step(&mut self) {
        self.machine.step(true);
    }

    fn run(&mut self) {
        self.machine.run(true);
        if self.machine.exit {
            return;
        } else {
            self.machine.pause = false;
        }
    }

    fn prompt(&mut self) {
        print!("dbg> ");
        io::stdout().flush().unwrap();
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).unwrap();
        cmd.pop().unwrap();

        if cmd.starts_with("read ") {
            let mut parts = cmd.split_ascii_whitespace();
            parts.next();
            let bytes = std::fs::read(&cmd[5..]).unwrap();
            for c in bytes {
                self.machine.stdin.push_back(c as u16);
            }
            self.run();
            return self.prompt();
        }

        if cmd.starts_with("set ") {
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            let reg = parts[1].parse::<usize>().unwrap();
            let value = parts[2].parse::<u16>().unwrap();
            self.machine.reg[reg] = value;
            return self.prompt();
        }

        if cmd.starts_with("break ") {
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            let value = parts[1].parse::<usize>().unwrap();
            self.machine.breakpoint = value;
            return self.prompt();
        }

        if cmd.starts_with("goto ") {
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            let value = parts[1].parse::<usize>().unwrap();
            self.machine.cursor = value;
            return self.prompt();
        }

        match cmd.as_str() {
            "run" => {
                self.run();
            }
            "step" => {
                self.step();
            }
            "exit" => {
                return;
            }
            "show" => {
                println!("==================");
                println!("exit: {}", self.machine.exit);
                println!("cursor: {}", self.machine.cursor);

                println!("------------------");
                for (n, v) in self.machine.reg.iter().enumerate() {
                    println!("reg {}: {}", n, v);
                }

                println!("------------------");
                let size = self.machine.stack.len();
                for (n, v) in self.machine.stack.iter().rev().enumerate() {
                    println!("stack {} {}", size - n, v);
                    if n > 5 {
                        break;
                    }
                }

                println!("------------------");
                let start = if self.machine.cursor >= 5 {
                    self.machine.cursor - 5
                } else {
                    0
                };
                let end = std::cmp::min(self.machine.cursor + 15, MAX_VALUE);
                for (n, v) in self.machine.mem[start..end].iter().enumerate() {
                    if start + n == self.machine.cursor {
                        println!("mem {} {}   <==", start + n, v);
                    } else {
                        println!("mem {} {}", start + n, v);
                    }
                }

                println!("==================");
            }
            _ => {
                for c in cmd.chars() {
                    self.machine.stdin.push_back(c as u16);
                }
                self.machine.stdin.push_back('\n' as u16);
                self.run();
            }
        }

        self.prompt();
    }
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
    breakpoint: usize,
    pause: bool,
    exit: bool,
    cursor: usize,
    mem: [u16; MAX_VALUE],
    reg: [u16; 8],
    stack: Vec<u16>,
    stdin: VecDeque<u16>,
}

impl Arch {
    fn load(path: &str) -> Arch {
        let mut arch = Arch {
            breakpoint: 0,
            pause: false,
            exit: false,
            cursor: 0,
            mem: [0; 32768],
            reg: [0; 8],
            stack: vec![],
            stdin: VecDeque::new(),
        };

        let bytes = std::fs::read(path).unwrap();
        for (n, byte_pair) in bytes.chunks_exact(2).enumerate() {
            arch.mem[n] = u16::from_le_bytes([byte_pair[0], byte_pair[1]]);
        }
        arch
    }

    fn run(&mut self, force: bool) {
        if force {
            self.step(true);
        }
        while !self.exit && !self.pause {
            self.step(false);
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

    fn step(&mut self, force: bool) {
        if self.cursor == self.breakpoint && !force {
            self.pause = true;
            return;
        }
        let instr = self.next();
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
                if self.stdin.len() > 0 {
                    let a = self.next();
                    let value = self.stdin.pop_front().unwrap();
                    self.write(a, value);
                } else {
                    self.pause = true;
                    self.cursor -= 1;
                }
            }
            21 => {
                // noop
            }
            _ => panic!("Unrecogized instruction: {}", instr),
        }
    }
}
