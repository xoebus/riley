#[macro_use]
extern crate itertools;
extern crate permutohedron;

use std::env;
use std::fmt;
use std::process;

use itertools::Itertools;
use permutohedron::Heap;

#[derive(Clone, Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op = match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
        };
        write!(f, "{}", op)
    }
}

struct Attempt {
    nums: Vec<i32>,
    ops: Vec<Operator>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 8 {
        eprintln!("usage: riley nums... target");
        process::exit(1)
    }

    let mut numbers: Vec<i32> = args.iter().filter_map(|v| v.parse().ok()).collect();
    let target = numbers.pop().unwrap();
    let number_permutations = Heap::new(&mut numbers);

    let operators = vec![
        Operator::Add,
        Operator::Subtract,
        Operator::Multiply,
        Operator::Divide,
    ];
    let operator_product = itertools::repeat_n(operators, 5).multi_cartesian_product();

    let attempts =
        iproduct!(number_permutations, operator_product).map(|(nums, ops)| Attempt { nums, ops });

    for attempt in attempts {
        if let Some(n) = solve(&attempt, target) {
            display(&attempt, n, target);
            break;
        }
    }
}

fn display(attempt: &Attempt, size: usize, answer: i32) {
    let mut exprs = Vec::new();
    for num in &attempt.nums {
        exprs.push(format!("{}", num))
    }

    for op in &attempt.ops[..size] {
        let x = exprs.pop().unwrap();
        let y = exprs.pop().unwrap();
        exprs.push(format!("({} {} {})", x, op, y))
    }

    println!("{} = {}", exprs.pop().unwrap(), answer)
}

fn solve(attempt: &Attempt, target: i32) -> Option<usize> {
    let mut stack = attempt.nums.clone();

    let mut n = 0;
    for op in &attempt.ops {
        let x = stack.pop().unwrap();
        let y = stack.pop().unwrap();

        let res = match op {
            Operator::Add => x + y,
            Operator::Subtract => x - y,
            Operator::Multiply => x * y,
            Operator::Divide => {
                if x % y != 0 {
                    return None;
                }

                x / y
            }
        };
        n += 1;

        if res <= 0 {
            return None;
        }

        if res == target {
            return Some(n);
        }

        stack.push(res)
    }

    match stack.pop() {
        Some(x) if x == target => Some(n),
        _ => None,
    }
}
