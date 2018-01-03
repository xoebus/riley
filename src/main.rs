extern crate itertools;
extern crate permutohedron;

use permutohedron::Heap;
use std::process;

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn main() {
    let a = [Operation::Add, Operation::Subtract, Operation::Multiply, Operation::Divide];

    let first = Repeater::new(a.iter(), a.len().pow(0)).cycle();
    let second = Repeater::new(a.iter(), a.len().pow(1)).cycle();
    let third = Repeater::new(a.iter(), a.len().pow(2)).cycle();
    let fourth = Repeater::new(a.iter(), a.len().pow(3)).cycle();

    let size = a.len().pow(5);

    let comb: Vec<_> = itertools::multizip((first, second, third, fourth))
        .take(size)
        .collect();

    let mut data = [25, 25, 3, 8, 5, 4];
    let solution = 345;

    let heap = Heap::new(&mut data);
    for item in heap {
        solve(solution, &item, comb.clone())
    }
}

fn solve(target: i32, numbers: &[i32], ops: Vec<(&Operation, &Operation, &Operation, &Operation)>) -> () {
    'outer: for op_set in ops {
        let mut nums = numbers.clone().to_vec();
        let mut os = vec!(op_set.0, op_set.1, op_set.2, op_set.3);

        let mut used = vec!();

        loop {
            let fst = nums.pop().unwrap();
            let snd = nums.pop();

            if snd.is_none() {
                break
            }

            let res = match os.pop() {
                Some(&Operation::Add) => {
                    used.push("+");
                    fst + snd.unwrap()
                },
                Some(&Operation::Subtract) => {
                    used.push("-");
                    fst - snd.unwrap()
                }
                Some(&Operation::Multiply) => {
                    used.push("*");
                    fst * snd.unwrap()
                },
                Some(&Operation::Divide) => {
                    if fst % snd.unwrap() != 0 {
                        continue 'outer;
                    }
                    used.push("/");
                    fst / snd.unwrap()
                },
                None => break
            };

            if res == target {
                show_result(res, &used, numbers);
                process::exit(0)
            }

            nums.push(res)
        }
    }
}

fn show_result(res: i32, used: &Vec<&str>, nums: &[i32]) -> () {
    let mut n = nums.to_vec();

    let mut u = used.clone();
    u.reverse();

    let mut exps = vec!();
    exps.push(format!("({} {} {})", n.pop().unwrap(), u.pop().unwrap(), n.pop().unwrap()));

    while n.len() > 0 && u.len() > 0 {
        let e = exps.pop().unwrap();

        exps.push(format!("({} {} {})", e, u.pop().unwrap(), n.pop().unwrap()));
    }

    println!("{} = {}", exps.pop().unwrap(), res)
}

#[derive(Clone, Debug)]
pub struct Repeater<I: Iterator> {
    iter: I,
    dups: usize,

    curr: Option<I::Item>,
    done: usize,
}

impl<I> Repeater<I> where I: Clone + Iterator {
    pub fn new(it: I, dups: usize) -> Self {
        return Repeater{
            iter: it,
            dups: dups,
            curr: None,
            done: 0,
        }
    }
}

impl<I> Iterator for Repeater<I> where I: Clone + Iterator, I::Item: Clone {
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        if self.done == 0 {
            self.curr = self.iter.next()
        }

        if self.done < self.dups {
            self.done += 1;
            self.curr.clone()
        } else {
            self.done = 0;
            self.next()
        }
    }
}
