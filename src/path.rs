// *   8   -   1
// 4   *   11  *
// +   4   -   18
// 22  -   9   *
//
// 22 + 4 + 4 + 22 - 4 - 18 * 1

use std::collections::{HashSet, VecDeque};

fn main() {
    bfs((7, 30));
}

fn bfs(target: (usize, i64)) {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let score = GRAPH[0].value;
    // visited.insert((0, score));
    queue.push_back(vec![(Op::Add, 0, score)]);

    let path = loop {
        let path = queue.pop_front().unwrap();
        let &(_, idx, score) = path.last().unwrap();

        if (idx, score) == target {
            break path;
        }
        visited.insert((idx, score));

        for &(idx_, op) in GRAPH[idx].edges.iter() {
            let value = GRAPH[idx_].value;
            let score_ = match op {
                Op::Add => score + value,
                Op::Sub => score - value,
                Op::Mul => score * value,
            };
            if visited.contains(&(idx_, score_)) {
                continue;
            }
            let mut path_ = path.clone();
            path_.push((op, idx_, score_));
            queue.push_back(path_);
        }
    };

    for &(op, idx, _) in path.iter() {
        match op {
            Op::Add => print!(" + {}", GRAPH[idx].value),
            Op::Sub => print!(" - {}", GRAPH[idx].value),
            Op::Mul => print!(" * {}", GRAPH[idx].value),
        }
    }
    println!("");
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Op {
    Add,
    Sub,
    Mul,
}

struct Node {
    value: i64,
    edges: &'static [(usize, Op)],
}

const GRAPH: [Node; 8] = [
    Node {
        value: 22,
        edges: &[(1, Op::Add), (2, Op::Add), (2, Op::Sub), (3, Op::Sub)],
    },
    Node {
        value: 4,
        edges: &[
            // (0, Op::Add),
            (2, Op::Add),
            (2, Op::Mul),
            (4, Op::Mul),
            (5, Op::Mul),
        ],
    },
    Node {
        value: 4,
        edges: &[
            // (0, Op::Add),
            // (0, Op::Sub),
            (1, Op::Add),
            (1, Op::Mul),
            (3, Op::Sub),
            (4, Op::Mul),
            (5, Op::Mul),
            (5, Op::Sub),
            (6, Op::Sub),
        ],
    },
    Node {
        value: 9,
        edges: &[
            // (0, Op::Sub),
            (2, Op::Sub),
            (5, Op::Sub),
            (6, Op::Sub),
            (6, Op::Mul),
        ],
    },
    Node {
        value: 8,
        edges: &[
            (1, Op::Mul),
            (2, Op::Mul),
            (5, Op::Mul),
            (5, Op::Sub),
            (7, Op::Sub),
        ],
    },
    Node {
        value: 11,
        edges: &[
            (1, Op::Mul),
            (2, Op::Mul),
            (2, Op::Sub),
            (3, Op::Sub),
            (4, Op::Mul),
            (4, Op::Sub),
            (6, Op::Sub),
            (6, Op::Mul),
            (7, Op::Sub),
            (7, Op::Mul),
        ],
    },
    Node {
        value: 18,
        edges: &[
            (2, Op::Sub),
            (3, Op::Sub),
            (3, Op::Mul),
            (5, Op::Sub),
            (5, Op::Mul),
            (7, Op::Mul),
        ],
    },
    Node {
        value: 1,
        edges: &[],
    },
];

// #[derive(Debug)]
// enum Room {
//     Number(i64),
//     Operation(fn(i64, i64) -> i64),
//     Vault(i64),
// }

// #[derive(Debug)]
// enum Direction {
//     North,
//     East,
//     South,
//     West,
// }

// fn add(a: i64, b: i64) -> i64 {
//     a + b
// }
// fn sub(a: i64, b: i64) -> i64 {
//     a - b
// }
// fn mul(a: i64, b: i64) -> i64 {
//     a * b
// }

// const ADD: Room = Room::Operation(add);
// const SUB: Room = Room::Operation(sub);
// const MUL: Room = Room::Operation(mul);

// struct Edge {
//     op: fn(i64, i64) -> i64,
//     target: Node,
// }

// struct Node {
//     value: i64,
//     children: Vec<Edge>,
// }

// fn build_tree() {
//     let vault = Node {
//         value: 1,
//         children: vec![],
//     };
//     let level1 = [
//         Node {
//             value: 8,
//             edges:
//         }
//     ];
// }

// // 22

// // 4a 4b 9

// // 8  11  18

// // 1

// const MAP: [[Room; 4]; 4] = [
//     [Room::Number(22), SUB, Room::Number(9), MUL],
//     [ADD, Room::Number(4), SUB, Room::Number(18)],
//     [Room::Number(4), MUL, Room::Number(11), MUL],
//     [MUL, Room::Number(8), SUB, Room::Vault(1)],
// ];

// fn next(x: usize, y: usize, dir: Direction) -> Option<(usize, usize)> {
//     let (x_, y_) = match dir {
//         North => (x, y + 1),
//         East => (x + 1, y),
//         South => (x, y - 1),
//         West => (x - 1, y),
//     };
//     if x_ < 0 || x_ >= 4 || y_ < 0 || y_ >= 4 {
//         None
//     } else {
//         Some((x_, y_))
//     }
// }

// fn step(
//     x: usize,
//     y: usize,
//     current: i64,
//     dir1: Direction,
//     dir2: Direction,
// ) -> Option<(usize, usize, i64)> {
//     let op_coords = next(x, y, dir1);
//     let (x_, y_) = match op_coords {
//         None => return None,
//         Some((x_, y_)) => (x_, y_),
//     };
//     let op = MAP[x_][y_];

//     let next_coords = next(x_, y_, dir2);
//     let (x_, y_) = match next_coords {
//         None => return None,
//         Some((x_, y_)) => (x_, y_),
//     };
// }

// fn main() {
//     let mut x: usize = 0;
//     let mut y: usize = 0;

//     println!("{:?}", MAP[x][y]);
// }
