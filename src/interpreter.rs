use crate::thread::Thread;
use std::io::{BufReader, Cursor, Seek, SeekFrom};
use crate::read_bytes_ext::ReadBytesExt;
use crate::instructions::factory::InstFactory;
use std::rc::{Rc, Weak};
use crate::jvm_stack::Frame;
use std::borrow::{BorrowMut, Borrow};
use std::cell::RefCell;
use std::panic;
use crate::main;

pub fn interpret(stack: u16, locals: u16, code: Vec<u8>) {
    let result = panic::catch_unwind(|| {
        let mut thread = Rc::new(RefCell::new(Thread::new()));
        trace!("New Thread: {:?}", thread);
        let frame = Rc::new(RefCell::new(Frame::new(Weak::new(), locals as usize, stack as usize)));
        trace!("New frame: {:?}", frame);
        (*thread).borrow_mut().push_frame(frame.clone());
        (*frame).borrow_mut().set_thread(Rc::downgrade(&thread));
        start_loop(thread, code);
    });
    match result {
        Ok(_) => trace!("Finish"),
        Err(e) => trace!("Err: {:?}", e)
    };
}

fn start_loop(mut thread: Rc<RefCell<Thread>>, code: Vec<u8>) {
    trace!("Start loop");
    let frame = (*thread).borrow_mut().pop_frame();
    let mut reader = BufReader::new(Cursor::new(code));
    loop {
        let pc = (*frame).borrow().get_next_pc();
        (*thread).borrow_mut().set_pc(pc);
        reader.seek(SeekFrom::Start(pc as u64));
        let opcode = reader.parse_u1();
        let mut inst = InstFactory::new(opcode);
        inst.fetch_operands(&mut reader);
        let pc = reader.seek(SeekFrom::Current(0)).unwrap();
        (*frame).borrow_mut().set_next_pc(pc);
        trace!("pc: {} opcode {} local: {}", pc, opcode, (*frame).borrow_mut().local_vars().get_int(1));
        inst.execute(frame.clone());
    }
    trace!("Finish. {:?}", (*frame).borrow_mut().local_vars());
}