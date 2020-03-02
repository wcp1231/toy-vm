use std::io::{BufReader, Read};
use crate::constant_pool::ConstantInfo::*;
use crate::read_bytes_ext::ReadBytesExt;
use std::rc::Rc;

const UTF8_TAG: u8 = 1;
const INTEGER_TAG: u8 = 3;
const FLOAT_TAG: u8 = 4;
const LONG_TAG: u8 = 5;
const DOUBLE_TAG: u8 = 6;
const CLASS_TAG: u8 = 7;
const STRING_TAG: u8 = 8;
const FIELD_REF_TAG: u8 = 9;
const METHOD_REF_TAG: u8 = 10;
const INTERFACE_METHOD_REF_TAG: u8 = 11;
const NAME_AND_TYPE_TAG: u8 = 12;
const METHOD_HANDLE_TAG: u8 = 15;
const METHOD_TYPE_TAG: u8 = 16;
const INVOKE_DYNAMIC_TAG: u8 = 18;

#[derive(Debug)]
pub struct  ConstantPool(Vec<ConstantInfo>);

#[derive(Debug)]
pub enum ConstantInfo {
    UTF8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(u16),
    JString(u16),
    FieldRef(u16, u16),
    MethodRef(u16, u16),
    InterfaceMethodRef(u16, u16),
    NameAndType(u16, u16),
    MethodHandle(u8, u16),
    MethodType(u16),
    InvokeDynamic(u16, u16),
    NIL(String),
    PADDING,
}

impl ConstantPool {
    pub fn read_constant_pool(reader: &mut BufReader<Box<dyn Read>>) -> Rc<ConstantPool> {
        let count = reader.parse_u2();
        trace!("Constants count: {}", count);
        let mut cp = Vec::new();
        cp.push(NIL(String::from("")));
        let mut offset = 1;
        while offset < count {
            let info = ConstantInfo::read_constant_info(reader);
            trace!("Read constant {} => {:?}", offset, info);
            cp.push(info);
            offset += 1;
            match cp.last().unwrap() {
                Long(_) | Double(_) => {
                    offset += 1;
                    cp.push(PADDING);
                }
                _ => {}
            }
        }
        return Rc::new(ConstantPool(cp));
    }

    fn get_constant_info(&self, index: usize) -> &ConstantInfo {
        match self.0.get(index) {
            Some(info) => info,
            None => panic!("Invalid constant pool index!")
        }
    }

    pub fn get_name_and_type(&self, index: usize) -> (&String, &String) {
        match self.get_constant_info(index) {
            NameAndType(nidx, tidx) => {
                let name = self.get_utf8(*nidx as usize);
                let _type = self.get_utf8(*tidx as usize);
                return (name, _type);
            }
            _ => panic!("Invalid constant pool type!")
        }
    }

    pub fn get_class_name(&self, index: usize) -> &String {
        trace!("get constant class name, idx: {}", index);
        match self.get_constant_info(index) {
            NIL(s) => s,
            Class(idx) => {
                 self.get_utf8(*idx as usize)
            },
            _ => panic!("Invalid constant pool type!")
        }
    }

    pub fn get_utf8(&self, index: usize) -> &String {
        trace!("get constant utf8, idx: {}", index);
        match self.get_constant_info(index) {
            UTF8(s) => s,
            _ => panic!("Invalid constant pool type!")
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl ConstantInfo {
    pub fn read_constant_info(reader: &mut BufReader<Box<dyn Read>>) -> ConstantInfo {
        let tag = reader.parse_u1();
        match tag {
            UTF8_TAG => {
                let len = reader.parse_u2();
                let buf = reader.parse_u1_with_len(len as u32);
                let s = std::str::from_utf8(&buf).unwrap();
                UTF8(s.to_string())
            }
            INTEGER_TAG => {
                let v = reader.parse_i32();
                Integer(v)
            }
            FLOAT_TAG => {
                let v = reader.parse_f32();
                Float(v)
            }
            LONG_TAG => {
                let v = reader.parse_i64();
                Long(v)
            }
            DOUBLE_TAG => {
                let v = reader.parse_f64();
                Double(v)
            }
            CLASS_TAG => {
                let name_index = reader.parse_u2();
                Class(name_index)
            }
            STRING_TAG => {
                let string_index = reader.parse_u2();
                JString(string_index)
            }
            FIELD_REF_TAG => {
                let class_index = reader.parse_u2();
                let name_and_type_index = reader.parse_u2();
                FieldRef(class_index, name_and_type_index)
            }
            METHOD_REF_TAG => {
                let class_index = reader.parse_u2();
                let name_and_type_index = reader.parse_u2();
                MethodRef(class_index, name_and_type_index)
            }
            INTERFACE_METHOD_REF_TAG => {
                let class_index = reader.parse_u2();
                let name_and_type_index = reader.parse_u2();
                InterfaceMethodRef(class_index, name_and_type_index)
            }
            NAME_AND_TYPE_TAG => {
                let name_index = reader.parse_u2();
                let descriptor_index = reader.parse_u2();
                NameAndType(name_index, descriptor_index)
            }
            _ => {
                warn!("Unknown tag: {}", tag);
                panic!("java.lang.ClassFormatError: constant pool tag!")
            }
        }
    }
}