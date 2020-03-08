use crate::instructions::Instruction;
use crate::jvm_stack::Frame;
use std::cell::RefCell;
use std::rc::Rc;

pub struct D2F {}
impl Instruction for D2F {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let d = frame.borrow_mut().operand_stack().pop_double();
        frame.borrow_mut().operand_stack().push_float(d as f32);
    }
}

pub struct D2I {}
impl Instruction for D2I {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let d = frame.borrow_mut().operand_stack().pop_double();
        frame.borrow_mut().operand_stack().push_int(d as i32);
    }
}

pub struct D2L {}
impl Instruction for D2L {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let d = frame.borrow_mut().operand_stack().pop_double();
        frame.borrow_mut().operand_stack().push_long(d as i64);
    }
}

pub struct F2D {}
impl Instruction for F2D {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let d = frame.borrow_mut().operand_stack().pop_float();
        frame.borrow_mut().operand_stack().push_double(d as f64);
    }
}

pub struct F2I {}
impl Instruction for F2I {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let d = frame.borrow_mut().operand_stack().pop_float();
        frame.borrow_mut().operand_stack().push_int(d as i32);
    }
}

pub struct F2L {}
impl Instruction for F2L {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let d = frame.borrow_mut().operand_stack().pop_float();
        frame.borrow_mut().operand_stack().push_long(d as i64);
    }
}

pub struct I2F {}
impl Instruction for I2F {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let d = frame.borrow_mut().operand_stack().pop_int();
        frame.borrow_mut().operand_stack().push_float(d as f32);
    }
}

pub struct I2D {}
impl Instruction for I2D {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let d = frame.borrow_mut().operand_stack().pop_int();
        frame.borrow_mut().operand_stack().push_double(d as f64);
    }
}

pub struct I2L {}
impl Instruction for I2L {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let d = frame.borrow_mut().operand_stack().pop_int();
        frame.borrow_mut().operand_stack().push_long(d as i64);
    }
}

pub struct L2D {}
impl Instruction for L2D {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let d = frame.borrow_mut().operand_stack().pop_long();
        frame.borrow_mut().operand_stack().push_double(d as f64);
    }
}

pub struct L2F {}
impl Instruction for L2F {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let d = frame.borrow_mut().operand_stack().pop_long();
        frame.borrow_mut().operand_stack().push_float(d as f32);
    }
}

pub struct L2I {}
impl Instruction for L2I {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let d = frame.borrow_mut().operand_stack().pop_long();
        frame.borrow_mut().operand_stack().push_int(d as i32);
    }
}

pub struct I2B {}
impl Instruction for I2B {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let i = frame.borrow_mut().operand_stack().pop_int();
        let ret: i8 = i as i8;
        frame.borrow_mut().operand_stack().push_int(ret as i32);
    }
}

pub struct I2C {}
impl Instruction for I2C {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let i = frame.borrow_mut().operand_stack().pop_int();
        let ret: u16 = i as u16;
        frame.borrow_mut().operand_stack().push_int(ret as i32);
    }
}

pub struct I2S {}
impl Instruction for I2S {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let i = frame.borrow_mut().operand_stack().pop_int();
        let ret: i16 = i as i16;
        frame.borrow_mut().operand_stack().push_int(ret as i32);
    }
}