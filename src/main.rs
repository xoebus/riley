#[macro_use]
extern crate itertools;
extern crate permutohedron;

use std::process;
use std::env;

mod repeater;
use repeater::Repeater;

use permutohedron::Heap;

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 8 {
        eprintln!("usage: riley nums... target");
        process::exit(1)
    }

    let target = match args.pop().map(|a| a.parse()) {
        Some(Ok(t)) => t,
        _ => {
            eprintln!("invalid target number");
            process::exit(1)
        }
    };

    let mut numbers: Vec<u32> = args.iter().skip(1).filter_map(|v| v.parse().ok()).collect();

    let ops = [
        Operation::Add,
        Operation::Subtract,
        Operation::Multiply,
        Operation::Divide,
    ];

    let first = Repeater::new(ops.iter(), ops.len().pow(0)).cycle();
    let second = Repeater::new(ops.iter(), ops.len().pow(1)).cycle();
    let third = Repeater::new(ops.iter(), ops.len().pow(2)).cycle();
    let fourth = Repeater::new(ops.iter(), ops.len().pow(3)).cycle();
    let fifth = Repeater::new(ops.iter(), ops.len().pow(4)).cycle();
    let size = ops.len().pow(6);

    let all_operations: Vec<_> = first
        .zip(second)
        .zip(third)
        .zip(fourth)
        .zip(fifth)
        .take(size)
        .map(|((((a, b), c), d), e)| vec![a, b, c, d, e])
        .collect();

    let all_numbers = Heap::new(&mut numbers);

    for (numbers, operations) in iproduct!(all_numbers, all_operations) {
        solve(target, &numbers, operations)
    }
}

fn solve(target: u32, numbers: &[u32], mut ops: Vec<&Operation>) -> () {
    let mut nums = numbers.to_owned();
    let mut used = vec![];

    loop {
        let fst = nums.pop().unwrap();
        let snd = nums.pop();

        if snd.is_none() {
            return;
        }

        let res = match ops.pop() {
            Some(&Operation::Add) => {
                used.push("+");
                fst + snd.unwrap()
            }
            Some(&Operation::Subtract) => {
                used.push("-");
                fst - snd.unwrap()
            }
            Some(&Operation::Multiply) => {
                used.push("*");
                fst * snd.unwrap()
            }
            Some(&Operation::Divide) => {
                if fst % snd.unwrap() != 0 {
                    return;
                }
                used.push("/");
                fst / snd.unwrap()
            }
            None => return,
        };

        if res == target {
            show_result(res, &used, numbers);
            process::exit(0)
        }

        nums.push(res)
    }
}

fn show_result(res: u32, used: &[&str], nums: &[u32]) -> () {
    let mut n = nums.to_vec();

    let mut u = used.to_owned();
    u.reverse();

    let mut exps = vec![];
    exps.push(format!(
        "({} {} {})",
        n.pop().unwrap(),
        u.pop().unwrap(),
        n.pop().unwrap()
    ));

    while !n.is_empty() && !u.is_empty() {
        let e = exps.pop().unwrap();

        exps.push(format!("({} {} {})", e, u.pop().unwrap(), n.pop().unwrap()));
    }

    println!("{} = {}", exps.pop().unwrap(), res)
}
