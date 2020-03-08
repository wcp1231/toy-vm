use crate::instructions::{Instruction};
use crate::jvm_stack::Frame;
use std::cell::RefCell;
use std::rc::Rc;
use std::io::{BufReader, Cursor};
use crate::read_bytes_ext::ReadBytesExt;

pub struct ASTORE {
    pub index: u32,
}
impl ASTORE {
    pub fn new(index: u32) -> ASTORE {
        ASTORE { index }
    }
}
impl Instruction for ASTORE {
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.index = reader.parse_u1() as u32;
    }
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        astore(frame, self.index as usize)
    }
}

pub struct ASTORE0 {}
impl Instruction for ASTORE0 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        astore(frame, 0)
    }
}

pub struct ASTORE1 {}
impl Instruction for ASTORE1 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        astore(frame, 1)
    }
}

pub struct ASTORE2 {}
impl Instruction for ASTORE2 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        astore(frame, 2)
    }
}

pub struct ASTORE3 {}
impl Instruction for ASTORE3 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        astore(frame, 3)
    }
}

///

pub struct DSTORE {
    pub index: u32,
}
impl DSTORE {
    pub fn new(index: u32) -> DSTORE {
        DSTORE { index }
    }
}
impl Instruction for DSTORE {
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.index = reader.parse_u1() as u32;
    }
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        dstore(frame, self.index as usize)
    }
}

pub struct DSTORE0 {}
impl Instruction for DSTORE0 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        dstore(frame, 0)
    }
}

pub struct DSTORE1 {}
impl Instruction for DSTORE1 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        dstore(frame, 1)
    }
}

pub struct DSTORE2 {}
impl Instruction for DSTORE2 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        dstore(frame, 2)
    }
}

pub struct DSTORE3 {}
impl Instruction for DSTORE3 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        dstore(frame, 3)
    }
}

/// LSTORE

pub struct FSTORE {
    pub index: u32,
}
impl FSTORE {
    pub fn new(index: u32) -> FSTORE {
        FSTORE { index }
    }
}
impl Instruction for FSTORE {
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.index = reader.parse_u1() as u32;
    }
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        fstore(frame, self.index as usize)
    }
}

pub struct FSTORE0 {}
impl Instruction for FSTORE0 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        fstore(frame, 0)
    }
}

pub struct FSTORE1 {}
impl Instruction for FSTORE1 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        fstore(frame, 1)
    }
}

pub struct FSTORE2 {}
impl Instruction for FSTORE2 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        fstore(frame, 2)
    }
}

pub struct FSTORE3 {}
impl Instruction for FSTORE3 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        fstore(frame, 3)
    }
}

///

pub struct ISTORE {
    pub index: u32,
}
impl ISTORE {
    pub fn new(index: u32) -> ISTORE {
        ISTORE { index }
    }
}
impl Instruction for ISTORE {
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.index = reader.parse_u1() as u32;
    }
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        istore(frame, self.index as usize)
    }
}

pub struct ISTORE0 {}
impl Instruction for ISTORE0 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        istore(frame, 0)
    }
}

pub struct ISTORE1 {}
impl Instruction for ISTORE1 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        istore(frame, 1)
    }
}

pub struct ISTORE2 {}
impl Instruction for ISTORE2 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        istore(frame, 2)
    }
}

pub struct ISTORE3 {}
impl Instruction for ISTORE3 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        istore(frame, 3)
    }
}

/// LSTORE

pub struct LSTORE {
    pub index: u32,
}
impl LSTORE {
    pub fn new(index: u32) -> LSTORE {
        LSTORE { index }
    }
}
impl Instruction for LSTORE {
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.index = reader.parse_u1() as u32;
    }
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        lstore(frame, self.index as usize)
    }
}

pub struct LSTORE0 {}
impl Instruction for LSTORE0 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        lstore(frame, 0)
    }
}

pub struct LSTORE1 {}
impl Instruction for LSTORE1 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        lstore(frame, 1)
    }
}

pub struct LSTORE2 {}
impl Instruction for LSTORE2 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        lstore(frame, 2)
    }
}

pub struct LSTORE3 {}
impl Instruction for LSTORE3 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        lstore(frame, 3)
    }
}

fn astore(frame: Rc<RefCell<Frame>>, offset: usize) {
    let val = frame.borrow_mut().operand_stack().pop_ref();
    frame.borrow_mut().local_vars().set_object(offset, val);
}

fn dstore(frame: Rc<RefCell<Frame>>, offset: usize) {
    let val = frame.borrow_mut().operand_stack().pop_double();
    frame.borrow_mut().local_vars().set_double(offset, val);
}

fn fstore(frame: Rc<RefCell<Frame>>, offset: usize) {
    let val = frame.borrow_mut().operand_stack().pop_float();
    frame.borrow_mut().local_vars().set_float(offset, val);
}

fn istore(frame: Rc<RefCell<Frame>>, offset: usize) {
    let val = frame.borrow_mut().operand_stack().pop_int();
    frame.borrow_mut().local_vars().set_int(offset, val);
}

fn lstore(frame: Rc<RefCell<Frame>>, offset: usize) {
    let val = frame.borrow_mut().operand_stack().pop_long();
    frame.borrow_mut().local_vars().set_long(offset, val);
}