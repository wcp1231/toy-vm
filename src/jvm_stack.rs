use std::mem::transmute;

type Slot = [u8; PTR_SIZE];
type WideSlot = [u8; PTR_SIZE * 2];

// 32-bit vm
const PTR_SIZE: usize = 4;

const NULL: Slot = [0x00; PTR_SIZE];
const LONG_NULL: WideSlot = [0x00; PTR_SIZE * 2];

pub struct Frame {
    local_vars: LocalVars,
    operand_stack: OperandStack,
}

pub struct LocalVars(Vec<u8>);

pub struct OperandStack {
    operands: Vec<u8>,
    pub operands_ptr: *mut u8,
}

impl Frame {
    pub fn new(max_locals: usize, max_stack: usize) -> Frame {
        Frame {
            local_vars: LocalVars::new(max_locals),
            operand_stack: OperandStack::new(max_stack),
        }
    }
}

impl LocalVars {
    pub fn new(max_locals: usize) -> LocalVars {
        LocalVars(vec![0u8; PTR_SIZE * max_locals as usize])
    }

    pub fn set_int(&mut self, offset: usize, val: i32) {
        self.0[offset * PTR_SIZE..(offset + 1) * PTR_SIZE].copy_from_slice(&val.to_ne_bytes());
    }

    pub fn get_int(&self, offset: usize) -> i32 {
        let mut data = NULL;
        &data[..].copy_from_slice(&self.0[offset * PTR_SIZE..(offset + 1) * PTR_SIZE]);
        i32::from_ne_bytes(data)
    }

    pub fn set_float(&mut self, offset: usize, val: f32) {
        self.0[offset * PTR_SIZE..(offset + 1) * PTR_SIZE].copy_from_slice(&val.to_ne_bytes());
    }
    pub fn get_float(&self, offset: usize) -> f32 {
        let mut data = NULL;
        &data[..].copy_from_slice(&self.0[offset * PTR_SIZE..(offset + 1) * PTR_SIZE]);
        f32::from_ne_bytes(data)
    }

    pub fn set_long(&mut self, offset: usize, val: i64) {
        self.0[offset * PTR_SIZE..(offset + 2) * PTR_SIZE].copy_from_slice(&val.to_ne_bytes());
    }

    pub fn get_long(&self, offset: usize) -> i64 {
        let mut data = LONG_NULL;
        &data[..].copy_from_slice(&self.0[offset * PTR_SIZE..(offset + 2) * PTR_SIZE]);
        i64::from_ne_bytes(data)
    }

    pub fn set_double(&mut self, offset: usize, val: f64) {
        self.0[offset * PTR_SIZE..(offset + 2) * PTR_SIZE].copy_from_slice(&val.to_ne_bytes());
    }

    pub fn get_double(&self, offset: usize) -> f64 {
        let mut data = LONG_NULL;
        &data[..].copy_from_slice(&self.0[offset * PTR_SIZE..(offset + 2) * PTR_SIZE]);
        f64::from_ne_bytes(data)
    }

    pub fn set_object(&mut self, offset: usize, val: u32) {
        self.0[offset * PTR_SIZE..(offset + 1) * PTR_SIZE].copy_from_slice(&val.to_ne_bytes());
    }

    pub fn get_object(&self, offset: usize) -> u32 {
        let mut data = NULL;
        &data[..].copy_from_slice(&self.0[offset * PTR_SIZE..(offset + 1) * PTR_SIZE]);
        u32::from_ne_bytes(data)
    }
}

impl OperandStack {
    pub fn new(max_stack: usize) -> OperandStack {
        let mut operands: Vec<u8> = vec![0u8; PTR_SIZE * max_stack as usize];
        let operands_ptr = operands.as_mut_ptr();
        OperandStack {
            operands,
            operands_ptr,
        }
    }

    fn push(&mut self, val: &[u8], count: usize) {
        unsafe {
            self.operands_ptr.copy_from(val.as_ptr(), count);
            self.operands_ptr = self.operands_ptr.add(count);
        }
    }

    fn pop(&mut self, bytes: *mut u8, count: usize) {
        unsafe {
            self.operands_ptr = self.operands_ptr.sub(count);
            self.operands_ptr.copy_to(bytes, count);
        }
    }

    pub fn push_int(&mut self, val: i32) {
        self.push(&val.to_ne_bytes(), PTR_SIZE);
    }

    pub fn pop_int(&mut self) -> i32 {
        let mut data = NULL;
        self.pop(data.as_mut_ptr(), PTR_SIZE);
        return i32::from_ne_bytes(data);
    }

    pub fn push_long(&mut self, val: i64) {
        self.push(&val.to_ne_bytes(), PTR_SIZE * 2);
    }

    pub fn pop_long(&mut self) -> i64 {
        let mut data = LONG_NULL;
        self.pop(data.as_mut_ptr(), PTR_SIZE * 2);
        return i64::from_ne_bytes(data);
    }

    pub fn push_float(&mut self, val: f32) {
        self.push(&val.to_ne_bytes(), PTR_SIZE);
    }

    pub fn pop_float(&mut self) -> f32 {
        let mut data = NULL;
        self.pop(data.as_mut_ptr(), PTR_SIZE);
        return f32::from_ne_bytes(data);
    }

    pub fn push_double(&mut self, val: f64) {
        self.push(&val.to_ne_bytes(), PTR_SIZE * 2);
    }

    pub fn pop_double(&mut self) -> f64 {
        let mut data = LONG_NULL;
        self.pop(data.as_mut_ptr(), PTR_SIZE * 2);
        return f64::from_ne_bytes(data);
    }

    pub fn push_ref(&mut self, val: u32) {
        self.push(&val.to_ne_bytes(), PTR_SIZE);
    }

    pub fn pop_ref(&mut self) -> u32 {
        let mut data = NULL;
        self.pop(data.as_mut_ptr(), PTR_SIZE);
        return u32::from_ne_bytes(data);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_local_vars() {
        let mut frame = Frame::new(100, 100);
        frame.local_vars.set_int(0, 100);
        frame.local_vars.set_int(1, -100);
        frame.local_vars.set_long(2, 2997924580);
        frame.local_vars.set_long(4, -2997924580);
        frame.local_vars.set_float(6, 3.1415926_f32);
        frame.local_vars.set_double(7, 2.71828182845_f64);
        frame.local_vars.set_object(9, 0);
        assert_eq!(frame.local_vars.get_int(0), 100);
        assert_eq!(frame.local_vars.get_int(1), -100);
        assert_eq!(frame.local_vars.get_long(2), 2997924580);
        assert_eq!(frame.local_vars.get_long(4), -2997924580);
        assert_eq!(frame.local_vars.get_float(6), 3.1415926_f32);
        assert_eq!(frame.local_vars.get_double(7), 2.71828182845_f64);
        assert_eq!(frame.local_vars.get_object(9), 0);
    }

    #[test]
    fn test_operand_stack() {
        let mut frame = Frame::new(100, 100);
        frame.operand_stack.push_int(100);
        frame.operand_stack.push_int(-100);
        frame.operand_stack.push_long(2997924580);
        frame.operand_stack.push_long(-2997924580);
        frame.operand_stack.push_float(3.1415926_f32);
        frame.operand_stack.push_double(2.71828182845_f64);
        frame.operand_stack.push_ref(0);
        assert_eq!(frame.operand_stack.pop_ref(), 0);
        assert_eq!(frame.operand_stack.pop_double(), 2.71828182845_f64);
        assert_eq!(frame.operand_stack.pop_float(), 3.1415926_f32);
        assert_eq!(frame.operand_stack.pop_long(), -2997924580);
        assert_eq!(frame.operand_stack.pop_long(), 2997924580);
        assert_eq!(frame.operand_stack.pop_int(), -100);
        assert_eq!(frame.operand_stack.pop_int(), 100);
    }
}