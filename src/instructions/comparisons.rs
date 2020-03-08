use crate::instructions::{Instruction, branch_logic};
use crate::jvm_stack::Frame;
use std::cell::RefCell;
use std::rc::Rc;
use std::io::{BufReader, Cursor};
use crate::read_bytes_ext::ReadBytesExt;

pub struct LCMP {}
impl Instruction for LCMP {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_long();
        let v1 = frame.borrow_mut().operand_stack().pop_long();
        if v1 == v2 {
            frame.borrow_mut().operand_stack().push_int(0);
        } else {
            frame.borrow_mut().operand_stack().push_int(if v1 > v2 { 1 } else { -1 });
        }
    }
}

pub struct FCMPG {}
impl Instruction for FCMPG {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_float();
        let v1 = frame.borrow_mut().operand_stack().pop_float();
        frame.borrow_mut().operand_stack().push_int(fcmp(v1, v2, true))
    }
}

pub struct FCMPL {}
impl Instruction for FCMPL {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_float();
        let v1 = frame.borrow_mut().operand_stack().pop_float();
        frame.borrow_mut().operand_stack().push_int(fcmp(v1, v2, false))
    }
}

pub struct DCMPG {}
impl Instruction for DCMPG {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_double();
        let v1 = frame.borrow_mut().operand_stack().pop_double();
        frame.borrow_mut().operand_stack().push_int(dcmp(v1, v2, true))
    }
}

pub struct DCMPL {}
impl Instruction for DCMPL {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_double();
        let v1 = frame.borrow_mut().operand_stack().pop_double();
        frame.borrow_mut().operand_stack().push_int(dcmp(v1, v2, false))
    }
}

/// ifcond

pub struct IFEQ {
    pub offset: i32,
}
impl Instruction for IFEQ {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let val = frame.borrow_mut().operand_stack().pop_int();
        if val == 0 {
            branch_logic(frame, self.offset);
        }
    }
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.offset = reader.parse_u2() as i32;
    }
}

pub struct IFNE {
    pub offset: i32,
}
impl Instruction for IFNE {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let val = frame.borrow_mut().operand_stack().pop_int();
        if val != 0 {
            branch_logic(frame, self.offset);
        }
    }
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.offset = reader.parse_u2() as i32;
    }
}

pub struct IFLT {
    pub offset: i32,
}
impl Instruction for IFLT {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let val = frame.borrow_mut().operand_stack().pop_int();
        if val < 0 {
            branch_logic(frame, self.offset);
        }
    }
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.offset = reader.parse_u2() as i32;
    }
}

pub struct IFLE {
    pub offset: i32,
}
impl Instruction for IFLE {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let val = frame.borrow_mut().operand_stack().pop_int();
        if val <= 0 {
            branch_logic(frame, self.offset);
        }
    }
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.offset = reader.parse_u2() as i32;
    }
}

pub struct IFGT {
    pub offset: i32,
}
impl Instruction for IFGT {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let val = frame.borrow_mut().operand_stack().pop_int();
        if val > 0 {
            branch_logic(frame, self.offset);
        }
    }
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.offset = reader.parse_u2() as i32;
    }
}

pub struct IFGE {
    pub offset: i32,
}
impl Instruction for IFGE {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let val = frame.borrow_mut().operand_stack().pop_int();
        if val >= 0 {
            branch_logic(frame, self.offset);
        }
    }
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.offset = reader.parse_u2() as i32;
    }
}

pub struct IF_ICMPEQ {
    pub offset: i32,
}
impl Instruction for IF_ICMPEQ {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let (v1, v2) = icmp_pop(frame.clone());
        if v1 == v2 {
            branch_logic(frame, self.offset);
        }
    }
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.offset = reader.parse_u2() as i32;
    }
}

pub struct IF_ICMPNE {
    pub offset: i32,
}
impl Instruction for IF_ICMPNE {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let (v1, v2) = icmp_pop(frame.clone());
        if v1 != v2 {
            branch_logic(frame, self.offset);
        }
    }
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.offset = reader.parse_u2() as i32;
    }
}

pub struct IF_ICMPLT {
    pub offset: i32,
}
impl Instruction for IF_ICMPLT {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let (v1, v2) = icmp_pop(frame.clone());
        if v1 < v2 {
            branch_logic(frame, self.offset);
        }
    }
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.offset = reader.parse_u2() as i32;
    }
}

pub struct IF_ICMPLE {
    pub offset: i32,
}
impl Instruction for IF_ICMPLE {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let (v1, v2) = icmp_pop(frame.clone());
        if v1 <= v2 {
            branch_logic(frame, self.offset);
        }
    }
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.offset = reader.parse_u2() as i32;
    }
}

pub struct IF_ICMPGT {
    pub offset: i32,
}
impl Instruction for IF_ICMPGT {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let (v1, v2) = icmp_pop(frame.clone());
        if v1 > v2 {
            branch_logic(frame, self.offset);
        }
    }
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.offset = reader.parse_u2() as i32;
    }
}

pub struct IF_ICMPGE {
    pub offset: i32,
}
impl Instruction for IF_ICMPGE {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let (v1, v2) = icmp_pop(frame.clone());
        if v1 >= v2 {
            branch_logic(frame, self.offset);
        }
    }
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.offset = reader.parse_u2() as i32;
    }
}

/// if_acmp

pub struct IF_ACMPEQ {
    pub offset: i32,
}
impl Instruction for IF_ACMPEQ {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let r2 = frame.borrow_mut().operand_stack().pop_ref();
        let r1 = frame.borrow_mut().operand_stack().pop_ref();
        if r1 == r2 {
            branch_logic(frame, self.offset);
        }
    }
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.offset = reader.parse_u2() as i32;
    }
}

pub struct IF_ACMPNE {
    pub offset: i32,
}
impl Instruction for IF_ACMPNE {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let r2 = frame.borrow_mut().operand_stack().pop_ref();
        let r1 = frame.borrow_mut().operand_stack().pop_ref();
        if r1 != r2 {
            branch_logic(frame, self.offset);
        }
    }
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.offset = reader.parse_u2() as i32;
    }
}

fn icmp_pop(frame: Rc<RefCell<Frame>>) -> (i32, i32) {
    let v2 = frame.borrow_mut().operand_stack().pop_int();
    let v1 = frame.borrow_mut().operand_stack().pop_int();
    return (v1, v2);
}

fn fcmp(v1: f32, v2: f32, flag: bool) -> i32 {
    if v1 == v2 {
        0
    } else if v1 > v2 {
        1
    } else if v1 < v2 {
        -1
    } else if flag {
        1
    } else {
        -1
    }
}

fn dcmp(v1: f64, v2: f64, flag: bool) -> i32 {
    if v1 == v2 {
        0
    } else if v1 > v2 {
        1
    } else if v1 < v2 {
        -1
    } else if flag {
        1
    } else {
        -1
    }
}