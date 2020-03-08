mod comparisons;
mod constants;
mod control;
mod conversions;
mod extended;
pub(crate) mod factory;
mod load;
mod math;
mod stack;
mod stores;

use std::io::{Read, BufReader, Seek, Cursor};
use crate::read_bytes_ext::ReadBytesExt;
use crate::jvm_stack::Frame;
use std::cell::RefCell;
use std::rc::Rc;
use std::borrow::Borrow;

pub trait Instruction {
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {} // nothing to do
    fn execute(&self, frame: Rc<RefCell<Frame>>) {} // nothing to do
}

pub fn branch_logic(frame: Rc<RefCell<Frame>>, offset: i32) {
    let next_pc = frame.borrow_mut().thread().upgrade().unwrap().borrow_mut().pc() + offset;
    frame.borrow_mut().set_next_pc(next_pc as u64);
}