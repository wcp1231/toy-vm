use crate::instructions::{Instruction};
use crate::jvm_stack::Frame;
use std::cell::RefCell;
use std::rc::Rc;
use std::io::{BufReader, Cursor};
use crate::read_bytes_ext::ReadBytesExt;

pub struct ILOAD {
    pub index: u32,
}
impl ILOAD {
    pub fn new(index: u32) -> ILOAD {
        ILOAD { index }
    }
}
impl Instruction for ILOAD {
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.index = reader.parse_u1() as u32;
    }

    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        iload(frame, self.index as usize);
    }
}

pub struct ILOAD0 {}
impl Instruction for ILOAD0 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        iload(frame, 0);
    }
}

pub struct ILOAD1 {}
impl Instruction for ILOAD1 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        iload(frame, 1);
    }
}

pub struct ILOAD2 {}
impl Instruction for ILOAD2 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        iload(frame, 2);
    }
}

pub struct ILOAD3 {}
impl Instruction for ILOAD3 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        iload(frame, 3);
    }
}

/// LLOAD

pub struct LLOAD {
    pub index: u32,
}
impl LLOAD {
    pub fn new(index: u32) -> LLOAD {
        LLOAD { index }
    }
}
impl Instruction for LLOAD {
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.index = reader.parse_u1() as u32;
    }
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        lload(frame, self.index as usize);
    }
}

pub struct LLOAD0 {}
impl Instruction for LLOAD0 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        lload(frame, 0);
    }
}

pub struct LLOAD1 {}
impl Instruction for LLOAD1 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        lload(frame, 1);
    }
}

pub struct LLOAD2 {}
impl Instruction for LLOAD2 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        lload(frame, 2);
    }
}

pub struct LLOAD3 {}
impl Instruction for LLOAD3 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        lload(frame, 3);
    }
}

/// FLOAD

pub struct FLOAD {
    pub index: u32,
}
impl FLOAD {
    pub fn new(index: u32) -> FLOAD {
        FLOAD { index }
    }
}
impl Instruction for FLOAD {
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.index = reader.parse_u1() as u32;
    }
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        fload(frame, self.index as usize);
    }
}

pub struct FLOAD0 {}
impl Instruction for FLOAD0 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        fload(frame, 0);
    }
}

pub struct FLOAD1 {}
impl Instruction for FLOAD1 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        fload(frame, 1);
    }
}

pub struct FLOAD2 {}
impl Instruction for FLOAD2 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        fload(frame, 2);
    }
}

pub struct FLOAD3 {}
impl Instruction for FLOAD3 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        fload(frame, 3);
    }
}

/// DLOAD

pub struct DLOAD {
    pub index: u32,
}
impl DLOAD {
    pub fn new(index: u32) -> DLOAD {
        DLOAD { index }
    }
}
impl Instruction for DLOAD {
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.index = reader.parse_u1() as u32;
    }
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        dload(frame, self.index as usize);
    }
}

pub struct DLOAD0 {}
impl Instruction for DLOAD0 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        dload(frame, 0);
    }
}

pub struct DLOAD1 {}
impl Instruction for DLOAD1 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        dload(frame, 1);
    }
}

pub struct DLOAD2 {}
impl Instruction for DLOAD2 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        dload(frame, 2);
    }
}

pub struct DLOAD3 {}
impl Instruction for DLOAD3 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        dload(frame, 3);
    }
}

/// ALOAD

pub struct ALOAD {
    pub index: u32,
}
impl ALOAD {
    pub fn new(index: u32) -> ALOAD {
        ALOAD { index }
    }
}
impl Instruction for ALOAD {
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.index = reader.parse_u1() as u32;
    }
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        aload(frame, self.index as usize);
    }
}

pub struct ALOAD0 {}
impl Instruction for ALOAD0 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        aload(frame, 0);
    }
}

pub struct ALOAD1 {}
impl Instruction for ALOAD1 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        aload(frame, 1);
    }
}

pub struct ALOAD2 {}
impl Instruction for ALOAD2 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        aload(frame, 2);
    }
}

pub struct ALOAD3 {}
impl Instruction for ALOAD3 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        aload(frame, 3);
    }
}

fn iload(frame: Rc<RefCell<Frame>>, idx: usize) {
    let val = frame.borrow_mut().local_vars().get_int(idx);
    frame.borrow_mut().operand_stack().push_int(val);
}

fn lload(frame: Rc<RefCell<Frame>>, idx: usize) {
    let val = frame.borrow_mut().local_vars().get_long(idx);
    frame.borrow_mut().operand_stack().push_long(val);
}

fn fload(frame: Rc<RefCell<Frame>>, idx: usize) {
    let val = frame.borrow_mut().local_vars().get_float(idx);
    frame.borrow_mut().operand_stack().push_float(val);
}

fn dload(frame: Rc<RefCell<Frame>>, idx: usize) {
    let val = frame.borrow_mut().local_vars().get_double(idx);
    frame.borrow_mut().operand_stack().push_double(val);
}

fn aload(frame: Rc<RefCell<Frame>>, idx: usize) {
    let val = frame.borrow_mut().local_vars().get_object(idx);
    frame.borrow_mut().operand_stack().push_ref(val);
}