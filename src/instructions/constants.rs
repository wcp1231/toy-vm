use crate::instructions::Instruction;
use crate::jvm_stack::Frame;
use std::io::{BufReader, Cursor};
use crate::read_bytes_ext::ReadBytesExt;
use std::cell::RefCell;
use std::rc::Rc;

/// const
pub struct NOP {}
impl Instruction for NOP {}

pub struct AconstNull {}
impl Instruction for AconstNull {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        frame.borrow_mut().operand_stack().push_ref(0);
    }
}

pub struct Dconst0 {}
impl Instruction for Dconst0 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        frame.borrow_mut().operand_stack().push_double(0.0);
    }
}

pub struct Dconst1 {}
impl Instruction for Dconst1 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        frame.borrow_mut().operand_stack().push_double(1.0);
    }
}

pub struct Fconst0 {}
impl Instruction for Fconst0 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        frame.borrow_mut().operand_stack().push_float(0.0);
    }
}

pub struct Fconst1 {}
impl Instruction for Fconst1 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        frame.borrow_mut().operand_stack().push_float(1.0);
    }
}

pub struct Fconst2 {}
impl Instruction for Fconst2 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        frame.borrow_mut().operand_stack().push_float(2.0);
    }
}

pub struct IconstM1 {}
impl Instruction for IconstM1 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        frame.borrow_mut().operand_stack().push_int(-1);
    }
}

pub struct Iconst0 {}
impl Instruction for Iconst0 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        frame.borrow_mut().operand_stack().push_int(0);
    }
}

pub struct Iconst1 {}
impl Instruction for Iconst1 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        frame.borrow_mut().operand_stack().push_int(1);
    }
}

pub struct Iconst2 {}
impl Instruction for Iconst2 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        frame.borrow_mut().operand_stack().push_int(2);
    }
}

pub struct Iconst3 {}
impl Instruction for Iconst3 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        frame.borrow_mut().operand_stack().push_int(3);
    }
}

pub struct Iconst4 {}
impl Instruction for Iconst4 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        frame.borrow_mut().operand_stack().push_int(4);
    }
}

pub struct Iconst5 {}
impl Instruction for Iconst5 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        frame.borrow_mut().operand_stack().push_int(5);
    }
}

pub struct Lconst0 {}
impl Instruction for Lconst0 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        frame.borrow_mut().operand_stack().push_long(0);
    }
}

pub struct Lconst1 {}
impl Instruction for Lconst1 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        frame.borrow_mut().operand_stack().push_long(1);
    }
}

/// ipush

pub struct BIPush {
    pub val: i8,
}
impl Instruction for BIPush {
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.val = reader.parse_u1() as i8;
    }
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        frame.borrow_mut().operand_stack().push_int(self.val as i32);
    }
}

pub struct SIPush {
    pub val: i16,
}
impl SIPush {
    pub fn new() -> SIPush {
        SIPush { val: 0 }
    }
}
impl Instruction for SIPush {
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.val = reader.parse_u2() as i16;
    }
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        frame.borrow_mut().operand_stack().push_int(self.val as i32);
    }
}