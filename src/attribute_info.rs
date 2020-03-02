use std::io::{BufReader, Read};
use crate::constant_pool::ConstantPool;
use crate::read_bytes_ext::ReadBytesExt;
use crate::attribute_info::AttributeInfo::*;
use std::rc::Rc;

#[derive(Debug)]
pub enum AttributeInfo {
    ConstantValue(String, u32, u16),
    Code {
        name: String,
        length: u32,
        max_stack: u16,
        max_locals: u16,
        code_length: u32,
        code: Vec<u8>,
        exception_table: Vec<ExceptionTable>,
        attributes: Vec<AttributeInfo>,
    },
    Unparsed(String, u32, Vec<u8>),
    Deprecated(String, u32),
    Synthetic(String, u32),
    SourceFile(String, u32, u16),
    Exceptions(String, u32, Vec<u16>),
    LineNumberTable {
        name: String,
        length: u32,
        table_length: u16,
        table: Vec<LineNumberTableEntry>,
    },
}

#[derive(Debug)]
pub struct ExceptionTable {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

#[derive(Debug)]
pub struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16
}

impl AttributeInfo {
    pub fn read_attributes(reader: &mut BufReader<Box<dyn Read>>, cp: Rc<ConstantPool>) -> Vec<AttributeInfo> {
        let count = reader.parse_u2();
        trace!("Read attributes count: {}", count);
        let mut attributes = Vec::new();
        for _i in 0..count {
            attributes.push(AttributeInfo::read_attribute(reader, cp.clone()));
        }
        return attributes;
    }

    pub fn read_attribute(reader: &mut BufReader<Box<dyn Read>>, cp: Rc<ConstantPool>) -> AttributeInfo {
        let name_index = reader.parse_u2();
        let length = reader.parse_u4();

        let name: String = cp.get_utf8(name_index as usize).into();
        trace!("Read attr name index: {}, name: {}, length: {}", name_index, name, length);
        match name.as_str() {
            "CODE" => {
                let max_stack = reader.parse_u2();
                let max_locals = reader.parse_u2();
                let code_length = reader.parse_u4();
                let code = reader.parse_u1_with_len(code_length as u32);
                let exception_table = AttributeInfo::read_exception_table(reader);
                let attributes = AttributeInfo::read_attributes(reader, cp);
                Code {
                    name,
                    length,
                    max_stack,
                    max_locals,
                    code_length,
                    code,
                    exception_table,
                    attributes,
                }
            },
            "ConstantValue" => {
                ConstantValue(name, length, reader.parse_u2())
            },
            "Deprecated" => Deprecated(name, length),
            "Exceptions" => {
                let count = reader.parse_u2();
                let mut exception_table = Vec::new();
                for _i in 0..count {
                    exception_table.push(reader.parse_u2());
                }
                Exceptions(name, length, exception_table)
            },
            "LineNumberTable" => {
                let table_length = reader.parse_u2();
                let mut table = Vec::new();
                for _i in 0..table_length {
                    table.push(LineNumberTableEntry {
                        start_pc: reader.parse_u2(),
                        line_number: reader.parse_u2(),
                    });
                }
                LineNumberTable {
                    name,
                    length,
                    table_length,
                    table
                }
            },
            //"LocalVariableTable" => {},
            "SourceFile" => SourceFile(name, length, reader.parse_u2()),
            "Synthetic" => Synthetic(name, length),
            _ => {
                Unparsed(name, length, reader.parse_u1_with_len(length))
            }
        }
    }

    fn read_exception_table(reader: &mut BufReader<Box<dyn Read>>) -> Vec<ExceptionTable> {
        let count = reader.parse_u2();
        let mut table = Vec::new();
        for _i in 0..count {
            table.push(ExceptionTable {
                start_pc: reader.parse_u2(),
                end_pc: reader.parse_u2(),
                handler_pc: reader.parse_u2(),
                catch_type: reader.parse_u2()
            });
        }
        return table;
    }
}