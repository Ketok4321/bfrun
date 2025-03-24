use std::io::{self, Read, Write};
use std::num::Wrapping;

use crate::syntax::*;
use crate::syntax::Op::*;

pub fn run(cell_count: usize, ops: &[Op]) {
    let mut cells: Vec<Wrapping<u8>> = vec![Wrapping(0); cell_count];
    let mut cell_n: usize = 0;

    let mut i = 0;
    while i < ops.len() {
        match &ops[i] {
            Inc(v) => {
                cells[cell_n] += *v as u8;
            },
            MvPtr(v) => {
                cell_n = cell_n.wrapping_add(*v as usize) % cell_count;
            },
            In => {
                let mut buff = [0u8];
                io::stdin().read_exact(&mut buff).unwrap();
                cells[cell_n] = Wrapping(buff[0]);
            },
            Out => {
                io::stdout().write(&[cells[cell_n].0]).unwrap();
            },
            LoopStart(l) => {
                if cells[cell_n] == Wrapping(0u8) {
                    i = *l - 1;
                }
            },
            LoopEnd(l) => {
                i = *l - 1;
            },
        }
        i += 1;
    }
}
