use crate::jvm_stack::Frame;
use std::rc::Rc;
use std::borrow::{BorrowMut, Borrow};
use std::cell::RefCell;
use std::ops::Deref;

#[derive(Debug)]
pub struct Thread {
    pc: i32,
    stack: Vec<Rc<RefCell<Frame>>>,
}

/*
impl Deref for Thread {
    type Target = Thread;

    fn deref(&self) -> &Thread {
        self
    }
}*/

impl Thread {
    pub fn new() -> Thread {
        Thread {
            pc: 0,
            stack: Vec::new()
        }
    }

    pub fn new_frame(&mut self, locals: usize, stack: usize) {
        //let frame = Frame::new(Rc::new(self), locals as usize, stack as usize);
        //self.push_frame(frame);
        //return frame;
    }

    pub fn pc(&self) -> i32 {
        self.pc
    }

    pub fn set_pc(&mut self, pc: i32) {
        self.pc = pc;
    }

    pub fn push_frame(&mut self, frame: Rc<RefCell<Frame>>) {
        // TODO check size
        self.stack.push(frame);
    }

    pub fn pop_frame(&mut self) -> Rc<RefCell<Frame>> {
        match self.stack.pop() {
            Some(f) => f.clone(),
            None => panic!("jvm stack empty")
        }
    }

    pub fn current_frame(&self) -> Rc<RefCell<Frame>> {
        match self.stack.last() {
            Some(f) => f.clone(),
            None => panic!("jvm stack empty")
        }
    }
}