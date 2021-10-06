// *   8   -   1
// 4   *   11  *
// +   4   -   18
// 22  -   9   *
//
// 22 + 4 - 11 * 4 - 18 - 11 - 1

use std::collections::{HashSet, VecDeque};

fn main() {
    bfs((7, 30));
}

fn bfs(target: (usize, i64)) {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let score = GRAPH[0].value;
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
