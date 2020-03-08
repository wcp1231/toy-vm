use crate::instructions::{Instruction, branch_logic};
use std::io::{BufReader, Cursor};
use crate::jvm_stack::Frame;
use crate::read_bytes_ext::ReadBytesExt;

use crate::instructions::load::*;
use crate::instructions::stores::*;
use crate::instructions::math::IINC;
use std::cell::RefCell;
use std::rc::Rc;

pub struct WIDE {
    pub modified_instruction: Box<dyn Instruction>
}
impl Instruction for WIDE {
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        let opcode = reader.parse_u1();
        self.modified_instruction = match opcode {
            0x15 => Box::new(ILOAD::new(reader.parse_u2() as u32)),
            0x16 => Box::new(LLOAD::new(reader.parse_u2() as u32)),
            0x17 => Box::new(FLOAD::new(reader.parse_u2() as u32)),
            0x18 => Box::new(DLOAD::new(reader.parse_u2() as u32)),
            0x19 => Box::new(ALOAD::new(reader.parse_u2() as u32)),
            0x36 => Box::new(ISTORE::new(reader.parse_u2() as u32)),
            0x37 => Box::new(LSTORE::new(reader.parse_u2() as u32)),
            0x38 => Box::new(FSTORE::new(reader.parse_u2() as u32)),
            0x39 => Box::new(DSTORE::new(reader.parse_u2() as u32)),
            0x3A => Box::new(ASTORE::new(reader.parse_u2() as u32)),
            0x84 => Box::new(IINC::new(reader.parse_u2() as usize, reader.parse_u2() as i32)),
            a => panic!("Unsupported opcode: {}", a),
        };
    }

    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        self.modified_instruction.execute(frame);
    }
}

pub struct IFNULL {
    pub offset: i32,
}
impl Instruction for IFNULL {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let r = frame.borrow_mut().operand_stack().pop_ref();
        if r == 0 {
            branch_logic(frame, self.offset);
        }
    }
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.offset = reader.parse_u2() as i32;
    }
}

pub struct IFNONNULL {
    pub offset: i32,
}
impl Instruction for IFNONNULL {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let r = frame.borrow_mut().operand_stack().pop_ref();
        if r != 0 {
            branch_logic(frame, self.offset);
        }
    }
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.offset = reader.parse_u2() as i32;
    }
}

pub struct GOTOW {
    pub offset: i32,
}
impl Instruction for GOTOW {
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.offset = reader.parse_u4() as i32;
    }

    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        branch_logic(frame, self.offset);
    }
}