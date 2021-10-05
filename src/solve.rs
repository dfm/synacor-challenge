// The disassembled program:
//
//   6027   jt $0 6035
//   6030  add $0 $1 1
//   6034  ret
//   6035   jt $1 6048
//   6038  add $0 $0 32767
//   6042  set $1 $7
//   6045 call 6027
//   6047  ret
//   6048 push $0
//   6050  add $1 $1 32767
//   6054 call 6027
//   6056  set $1 $0
//   6059  pop $0
//   6061  add $0 $0 32767
//   6065 call 6027
//   6067  ret
//
// Result: 25734

fn main() {
    for n in 1..MAX_VALUE {
        let mut cache: Cache = [[None; MAX_VALUE]; 5];
        let result = func(&mut cache, n as u16, 4, 1);
        if result == 6 {
            println!("{}", n);
            break;
        }
    }
}

const MAX_VALUE: usize = 32768;
type Cache = [[Option<u16>; MAX_VALUE]; 5];

fn wrap(a: usize) -> u16 {
    (a % MAX_VALUE) as u16
}

fn add(a: u16, b: u16) -> u16 {
    wrap((a as usize) + (b as usize))
}

fn mult(a: u16, b: u16) -> u16 {
    wrap((a as usize) * (b as usize))
}

fn func(cache: &mut Cache, c: u16, a: u16, b: u16) -> u16 {
    match cache[a as usize][b as usize] {
        None => {}
        Some(cached) => {
            return cached;
        }
    }

    let result = match (a, b) {
        // (0, b) => add(b, 1),
        (2, b) => add(mult(b, add(c, 1)), add(mult(2, c), 1)),
        (a, 0) => func(cache, c, add(a, 32767), c),
        (a, b) => {
            let b = func(cache, c, a, add(b, 32767));
            return func(cache, c, add(a, 32767), b);
        }
    };

    cache[a as usize][b as usize] = Some(result);

    result
}
