use std::io;
use std::mem::transmute;
use std::io::{Seek, SeekFrom};

pub trait ReadBytesExt: io::Read {
    fn parse_u1_with_len(&mut self, length: u32) -> Vec<u8> {
        let mut buf = Vec::<u8>::with_capacity(length as usize);

        for _x in 0..length {
            buf.push(self.parse_u1());
        }
        return buf;
    }

    fn parse_u1(&mut self) -> u8 {
        let mut buf = [0; 1];
        self.read_exact(&mut buf);
        buf[0]
    }

    fn parse_u2(&mut self) -> u16 {
        let mut buf = [0; 2];
        self.read_exact(&mut buf);
        unsafe {
            transmute::<[u8; 2], u16>(buf)
        }.to_be()
    }

    fn parse_u4(&mut self) -> u32 {
        let mut buf = [0; 4];
        self.read_exact(&mut buf);
        unsafe {
            transmute::<[u8; 4], u32>(buf)
        }.to_be()
    }

    fn parse_u8(&mut self) -> u64 {
        let mut buf = [0; 8];
        self.read_exact(&mut buf);
        unsafe {
            transmute::<[u8; 8], u64>(buf)
        }.to_be()
    }

    fn parse_f32(&mut self) -> f32 {
        let v = self.parse_u4();
        unsafe { transmute::<u32, f32>(v) }
    }

    fn parse_f64(&mut self) -> f64 {
        let v = self.parse_u8();
        unsafe { transmute::<u64, f64>(v) }
    }

    fn parse_i32(&mut self) -> i32 {
        let v = self.parse_u4();
        unsafe { transmute::<u32, i32>(v) }
    }

    fn parse_i64(&mut self) -> i64 {
        let v = self.parse_u8();
        unsafe { transmute::<u64, i64>(v) }
    }
}

impl<R: io::Read + ?Sized> ReadBytesExt for R {}

pub trait SeekExt: Seek {
    fn seek_padding(&mut self) {
        let curr = self.seek(SeekFrom::Current(0)).unwrap();
        let offset = curr % 4;
        self.seek(SeekFrom::Current(offset as i64));
    }
}

impl<R: Seek> SeekExt for R {}
