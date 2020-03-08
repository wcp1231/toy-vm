use crate::instructions::{Instruction, branch_logic};
use crate::jvm_stack::Frame;
use std::io::{BufReader, Cursor};
use crate::read_bytes_ext::{ReadBytesExt, SeekExt};
use std::fs::read;
use std::cell::RefCell;
use std::rc::Rc;

pub struct GOTO {
    pub offset: i32,
}
impl Instruction for GOTO {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        branch_logic(frame, self.offset);
    }

    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        self.offset = (reader.parse_u2() as i16) as i32;
    }
}


pub struct TABLE_SWITCH {
    pub default_offset: i32,
    pub low: i32,
    pub high: i32,
    pub jump_offsets: Vec<i32>
}
impl Instruction for TABLE_SWITCH {
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        reader.seek_padding();
        self.default_offset = reader.parse_i32();
        self.low = reader.parse_i32();
        self.high = reader.parse_i32();
        let count = self.high - self.low + 1;
        self.jump_offsets = Vec::with_capacity(count as usize);
        for _i in 0..count {
            self.jump_offsets.push(reader.parse_i32());
        }
    }

    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let idx = frame.borrow_mut().operand_stack().pop_int();
        let offset = if self.low <= idx && idx <= self.high {
            let offset = (idx - self.low) as usize;
            *self.jump_offsets.get(offset).unwrap()
        } else {
            self.default_offset
        };
        branch_logic(frame, offset);
    }
}

pub struct LOOKUP_SWITCH {
    pub default_offset: i32,
    pub npairs: i32,
    pub match_offsets: Vec<i32>,
}
impl Instruction for LOOKUP_SWITCH {
    fn fetch_operands(&mut self, reader: &mut BufReader<Cursor<Vec<u8>>>) {
        reader.seek_padding();
        self.default_offset = reader.parse_i32();
        self.npairs = reader.parse_i32();
        self.match_offsets = Vec::new();
        for _i in 0..self.npairs {
            self.match_offsets.push(reader.parse_i32());
            self.match_offsets.push(reader.parse_i32());
        }
    }

    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let key = frame.borrow_mut().operand_stack().pop_int();
        let size = self.match_offsets.len();
        let mut i = 0;
        while i < size {
            if self.match_offsets.get(i).map_or(false, |n| *n == key) {
                branch_logic(frame, *self.match_offsets.get(i+1)
                    .unwrap_or(&self.default_offset));
                return;
            }
            i += 2;
        }
        branch_logic(frame, self.default_offset);
    }
}