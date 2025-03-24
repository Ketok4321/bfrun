use std::iter::Peekable;

use crate::syntax::*;
use crate::syntax::Op::*;

fn get_net_sum(iter: &mut Peekable<impl Iterator<Item = char>>, pos_c: char, neg_c: char) -> isize {
    let mut sum = 0;

    while let Some(c) = iter.next_if(|c| c == &pos_c || c == &neg_c) {
        match c {
            a if a == pos_c => sum += 1,
            b if b == neg_c => sum -= 1,
            _ => panic!("this shouldn't be possible"),
        }
    }

    sum
}

pub fn parse(code: &str) -> Vec<Op> {
    let mut res: Vec<Op> = vec![];

    let mut it = code.chars().peekable();
    
    let mut loop_stack: Vec<usize> = vec![];

    while let Some(c) = it.next() {
        match c {
            '+' | '-' => {
                let s = get_net_sum(&mut it, '+', '-');
                let s = if c == '+' { s + 1 } else { s - 1 };
                if s != 0 {
                    res.push(Inc(s as i8));
                }
            },
            '>' | '<' => {
                let s = get_net_sum(&mut it, '>', '<');
                let s = if c == '>' { s + 1 } else { s - 1 };
                if s != 0 {
                    res.push(MvPtr(s));
                }
            },
            '.' => res.push(Out),
            ',' => res.push(In),
            '[' => {
                loop_stack.push(res.len());
                res.push(LoopStart(0));
            },
            ']' => {
                let i = loop_stack.pop().unwrap();
                res[i] = LoopStart(res.len() + 1); // TODO: That can be out of bounds
                res.push(LoopEnd(i));
            },
            _ => (),
        }
    }

    res
}
