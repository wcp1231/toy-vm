use crate::instructions::Instruction;
use crate::jvm_stack::Frame;
use std::cell::RefCell;
use std::rc::Rc;

pub struct POP {}
impl Instruction for POP {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        frame.borrow_mut().operand_stack().pop_slot();
    }
}

pub struct POP2 {}
impl Instruction for POP2 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        frame.borrow_mut().operand_stack().pop_slot();
        frame.borrow_mut().operand_stack().pop_slot();
    }
}

/// dup

pub struct DUP {}
impl Instruction for DUP {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let slot = frame.borrow_mut().operand_stack().pop_slot();
        frame.borrow_mut().operand_stack().push_slot(slot);
        frame.borrow_mut().operand_stack().push_slot(slot);
    }
}

pub struct DupX1 {}
impl Instruction for DupX1 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let slot1 = frame.borrow_mut().operand_stack().pop_slot();
        let slot2 = frame.borrow_mut().operand_stack().pop_slot();
        frame.borrow_mut().operand_stack().push_slot(slot1);
        frame.borrow_mut().operand_stack().push_slot(slot2);
        frame.borrow_mut().operand_stack().push_slot(slot1);
    }
}

pub struct DupX2 {}
impl Instruction for DupX2 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let slot1 = frame.borrow_mut().operand_stack().pop_slot();
        let slot2 = frame.borrow_mut().operand_stack().pop_slot();
        let slot3 = frame.borrow_mut().operand_stack().pop_slot();
        frame.borrow_mut().operand_stack().push_slot(slot1);
        frame.borrow_mut().operand_stack().push_slot(slot3);
        frame.borrow_mut().operand_stack().push_slot(slot2);
        frame.borrow_mut().operand_stack().push_slot(slot1);
    }
}

pub struct DUP2 {}
impl Instruction for DUP2 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let slot1 = frame.borrow_mut().operand_stack().pop_slot();
        let slot2 = frame.borrow_mut().operand_stack().pop_slot();
        frame.borrow_mut().operand_stack().push_slot(slot2);
        frame.borrow_mut().operand_stack().push_slot(slot1);
        frame.borrow_mut().operand_stack().push_slot(slot2);
        frame.borrow_mut().operand_stack().push_slot(slot1);
    }
}

pub struct Dup2X1 {}
impl Instruction for Dup2X1 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let slot1 = frame.borrow_mut().operand_stack().pop_slot();
        let slot2 = frame.borrow_mut().operand_stack().pop_slot();
        let slot3 = frame.borrow_mut().operand_stack().pop_slot();
        frame.borrow_mut().operand_stack().push_slot(slot2);
        frame.borrow_mut().operand_stack().push_slot(slot1);
        frame.borrow_mut().operand_stack().push_slot(slot3);
        frame.borrow_mut().operand_stack().push_slot(slot2);
        frame.borrow_mut().operand_stack().push_slot(slot1);
    }
}

pub struct Dup2X2 {}
impl Instruction for Dup2X2 {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let slot1 = frame.borrow_mut().operand_stack().pop_slot();
        let slot2 = frame.borrow_mut().operand_stack().pop_slot();
        let slot3 = frame.borrow_mut().operand_stack().pop_slot();
        let slot4 = frame.borrow_mut().operand_stack().pop_slot();
        frame.borrow_mut().operand_stack().push_slot(slot2);
        frame.borrow_mut().operand_stack().push_slot(slot1);
        frame.borrow_mut().operand_stack().push_slot(slot4);
        frame.borrow_mut().operand_stack().push_slot(slot3);
        frame.borrow_mut().operand_stack().push_slot(slot2);
        frame.borrow_mut().operand_stack().push_slot(slot1);
    }
}

/// swap

pub struct SWAP {}
impl Instruction for SWAP {
    fn execute(&self, frame: Rc<RefCell<Frame>>) {
        let slot1 = frame.borrow_mut().operand_stack().pop_slot();
        let slot2 = frame.borrow_mut().operand_stack().pop_slot();
        frame.borrow_mut().operand_stack().push_slot(slot1);
        frame.borrow_mut().operand_stack().push_slot(slot2);
    }
}