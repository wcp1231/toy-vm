use crate::instructions::Instruction;
use crate::jvm_stack::Frame;
use std::io::{BufReader, Cursor};
use crate::read_bytes_ext::ReadBytesExt;
use std::cell::RefCell;
use std::rc::Rc;

pub struct IADD {}
impl Instruction for IADD {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_int();
        let v1 = frame.borrow_mut().operand_stack().pop_int();
        frame.borrow_mut().operand_stack().push_int(v1 + v2);
    }
}

pub struct LADD {}
impl Instruction for LADD {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_long();
        let v1 = frame.borrow_mut().operand_stack().pop_long();
        frame.borrow_mut().operand_stack().push_long(v1 + v2);
    }
}

pub struct FADD {}
impl Instruction for FADD {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_float();
        let v1 = frame.borrow_mut().operand_stack().pop_float();
        frame.borrow_mut().operand_stack().push_float(v1 + v2);
    }
}

pub struct DADD {}
impl Instruction for DADD {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_double();
        let v1 = frame.borrow_mut().operand_stack().pop_double();
        frame.borrow_mut().operand_stack().push_double(v1 + v2);
    }
}

/// sub

pub struct ISUB {}
impl Instruction for ISUB {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_int();
        let v1 = frame.borrow_mut().operand_stack().pop_int();
        frame.borrow_mut().operand_stack().push_int(v1 - v2);
    }
}

pub struct LSUB {}
impl Instruction for LSUB {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_long();
        let v1 = frame.borrow_mut().operand_stack().pop_long();
        frame.borrow_mut().operand_stack().push_long(v1 - v2);
    }
}

pub struct FSUB {}
impl Instruction for FSUB {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_float();
        let v1 = frame.borrow_mut().operand_stack().pop_float();
        frame.borrow_mut().operand_stack().push_float(v1 - v2);
    }
}

pub struct DSUB {}
impl Instruction for DSUB {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_double();
        let v1 = frame.borrow_mut().operand_stack().pop_double();
        frame.borrow_mut().operand_stack().push_double(v1 - v2);
    }
}

/// mul

pub struct IMUL {}
impl Instruction for IMUL {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_int();
        let v1 = frame.borrow_mut().operand_stack().pop_int();
        frame.borrow_mut().operand_stack().push_int(v1 * v2);
    }
}

pub struct LMUL {}
impl Instruction for LMUL {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_long();
        let v1 = frame.borrow_mut().operand_stack().pop_long();
        frame.borrow_mut().operand_stack().push_long(v1 * v2);
    }
}

pub struct FMUL {}
impl Instruction for FMUL {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_float();
        let v1 = frame.borrow_mut().operand_stack().pop_float();
        frame.borrow_mut().operand_stack().push_float(v1 * v2);
    }
}

pub struct DMUL {}
impl Instruction for DMUL {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_double();
        let v1 = frame.borrow_mut().operand_stack().pop_double();
        frame.borrow_mut().operand_stack().push_double(v1 * v2);
    }
}

/// sub

pub struct IDIV {}
impl Instruction for IDIV {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_int();
        let v1 = frame.borrow_mut().operand_stack().pop_int();
        frame.borrow_mut().operand_stack().push_int(v1 / v2);
    }
}

pub struct LDIV {}
impl Instruction for LDIV {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_long();
        let v1 = frame.borrow_mut().operand_stack().pop_long();
        frame.borrow_mut().operand_stack().push_long(v1 / v2);
    }
}

pub struct FDIV {}
impl Instruction for FDIV {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_float();
        let v1 = frame.borrow_mut().operand_stack().pop_float();
        frame.borrow_mut().operand_stack().push_float(v1 / v2);
    }
}

pub struct DDIV {}
impl Instruction for DDIV {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_double();
        let v1 = frame.borrow_mut().operand_stack().pop_double();
        frame.borrow_mut().operand_stack().push_double(v1 / v2);
    }
}

/// DREM

pub struct DREM {}
impl Instruction for DREM {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_double();
        let v1 = frame.borrow_mut().operand_stack().pop_double();
        frame.borrow_mut().operand_stack().push_double(v1 % v2);
    }
}

pub struct FREM {}
impl Instruction for FREM {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_float();
        let v1 = frame.borrow_mut().operand_stack().pop_float();
        frame.borrow_mut().operand_stack().push_float(v1 % v2);
    }
}

pub struct IREM {}
impl Instruction for IREM {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_int();
        let v1 = frame.borrow_mut().operand_stack().pop_int();
        if v2 == 0 {
            panic!("java.lang.ArithmeticException: / by zero");
        }
        frame.borrow_mut().operand_stack().push_int(v1 % v2);
    }
}

pub struct LREM {}
impl Instruction for LREM {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_long();
        let v1 = frame.borrow_mut().operand_stack().pop_long();
        if v2 == 0 {
            panic!("java.lang.ArithmeticException: / by zero");
        }
        frame.borrow_mut().operand_stack().push_long(v1 % v2);
    }
}

/// neg

pub struct INEG {}
impl Instruction for INEG {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v = frame.borrow_mut().operand_stack().pop_int();
        frame.borrow_mut().operand_stack().push_int(-v);
    }
}

pub struct LNEG {}
impl Instruction for LNEG {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v = frame.borrow_mut().operand_stack().pop_long();
        frame.borrow_mut().operand_stack().push_long(-v);
    }
}

pub struct FNEG {}
impl Instruction for FNEG {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v = frame.borrow_mut().operand_stack().pop_float();
        frame.borrow_mut().operand_stack().push_float(-v);
    }
}

pub struct DNEG {}
impl Instruction for DNEG {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v = frame.borrow_mut().operand_stack().pop_double();
        frame.borrow_mut().operand_stack().push_double(-v);
    }
}

/// sh

pub struct ISHL {}
impl Instruction for ISHL {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_int();
        let v1 = frame.borrow_mut().operand_stack().pop_int();
        let s = (v2 as u32) & 0x1F;
        frame.borrow_mut().operand_stack().push_int(v1 << s as i32);
    }
}

pub struct ISHR {}
impl Instruction for ISHR {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_int();
        let v1 = frame.borrow_mut().operand_stack().pop_int();
        let s = (v2 as u32) & 0x1F;
        frame.borrow_mut().operand_stack().push_int(v1 >> s as i32);
    }
}

pub struct IUSHR {}
impl Instruction for IUSHR {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_int();
        let v1 = frame.borrow_mut().operand_stack().pop_int();
        let s = (v2 as u32) & 0x1F;
        frame.borrow_mut().operand_stack().push_int((v1 as u32 >> s) as i32);
    }
}

pub struct LSHL {}
impl Instruction for LSHL {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_int();
        let v1 = frame.borrow_mut().operand_stack().pop_long();
        let s = (v2 as u32) & 0x3F;
        frame.borrow_mut().operand_stack().push_long(v1 << s as i64);
    }
}

pub struct LSHR {}
impl Instruction for LSHR {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_int();
        let v1 = frame.borrow_mut().operand_stack().pop_long();
        let s = (v2 as u32) & 0x3F;
        frame.borrow_mut().operand_stack().push_long(v1 >> s as i64);
    }
}

pub struct LUSHR {}
impl Instruction for LUSHR {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_int();
        let v1 = frame.borrow_mut().operand_stack().pop_long();
        let s = (v2 as u64) & 0x3F;
        frame.borrow_mut().operand_stack().push_long((v1 as u64 >> s) as i64);
    }
}

/// and

pub struct IAND {}
impl Instruction for IAND {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_int();
        let v1 = frame.borrow_mut().operand_stack().pop_int();
        frame.borrow_mut().operand_stack().push_int(v1 & v2);
    }
}

pub struct LAND {}
impl Instruction for LAND {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_long();
        let v1 = frame.borrow_mut().operand_stack().pop_long();
        frame.borrow_mut().operand_stack().push_long(v1 & v2);
    }
}

/// or

pub struct IOR {}
impl Instruction for IOR {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_int();
        let v1 = frame.borrow_mut().operand_stack().pop_int();
        frame.borrow_mut().operand_stack().push_int(v1 | v2);
    }
}

pub struct LOR {}
impl Instruction for LOR {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_long();
        let v1 = frame.borrow_mut().operand_stack().pop_long();
        frame.borrow_mut().operand_stack().push_long(v1 | v2);
    }
}

/// xor

pub struct IXOR {}
impl Instruction for IXOR {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_int();
        let v1 = frame.borrow_mut().operand_stack().pop_int();
        frame.borrow_mut().operand_stack().push_int(v1 ^ v2);
    }
}

pub struct LXOR {}
impl Instruction for LXOR {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let v2 = frame.borrow_mut().operand_stack().pop_long();
        let v1 = frame.borrow_mut().operand_stack().pop_long();
        frame.borrow_mut().operand_stack().push_long(v1 ^ v2);
    }
}

/// iinc

pub struct IINC {
    pub idx: usize,
    pub cst: i32,
}
impl IINC {
    pub fn new(idx: usize, cst: i32) -> IINC {
        IINC { idx, cst }
    }
}
impl Instruction for IINC {
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.idx = reader.parse_u1() as usize;
        self.cst = reader.parse_u1() as i32;
    }
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let mut val = frame.borrow_mut().local_vars().get_int(self.idx);
        val += self.cst;
        frame.borrow_mut().local_vars().set_int(self.idx, val);
    }
}