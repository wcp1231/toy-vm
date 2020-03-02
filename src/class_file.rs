use std::io::{BufReader, Read};
use crate::constant_pool::ConstantPool;
use crate::read_bytes_ext::ReadBytesExt;
use crate::attribute_info::AttributeInfo;
use std::rc::Rc;

pub struct ClassFile {
    minor_version: u16,
    major_version: u16,
    constant_pool: Rc<ConstantPool>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<MemberInfo>,
    methods: Vec<MemberInfo>,
    attributes: Vec<AttributeInfo>
}

#[derive(Debug)]
pub struct MemberInfo {
    cp: Rc<ConstantPool>,
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeInfo>,
}

impl ClassFile {
    pub fn parse(reader: &mut BufReader<Box<dyn Read>>) -> Box<ClassFile> {
        ClassFile::read_and_check_magic(reader);
        let (minor, major) = ClassFile::read_and_check_version(reader);
        let constant_pool = ConstantPool::read_constant_pool(reader);

        return Box::new(ClassFile {
            minor_version: minor,
            major_version: major,
            constant_pool: constant_pool.clone(),
            access_flags: reader.parse_u2(),
            this_class: reader.parse_u2(),
            super_class: reader.parse_u2(),
            interfaces: ClassFile::read_interfaces(reader),
            fields: MemberInfo::read_members(reader, constant_pool.clone()),
            methods: MemberInfo::read_members(reader, constant_pool.clone()),
            attributes: AttributeInfo::read_attributes(reader, constant_pool.clone()),
        });
    }

    fn read_and_check_magic(reader: &mut BufReader<Box<dyn Read>>) {
        let magic = reader.parse_u4();
        if magic != 0xCAFEBABE {
            panic!("java.lang.ClassFormatError: magic!");
        }
    }

    fn read_and_check_version(reader: &mut BufReader<Box<dyn Read>>) -> (u16, u16) {
        let minor_version = reader.parse_u2();
        let major_version = reader.parse_u2();
        match major_version {
            45 => return (minor_version, major_version),
            46..=52 => {
                if minor_version == 0 {
                    return (minor_version, major_version);
                }
            }
            _ => {}
        }
        panic!("java.lang.UnsupportedClassVersionError!")
    }

    fn read_interfaces(reader: &mut BufReader<Box<dyn Read>>) -> Vec<u16> {
        let count = reader.parse_u2();
        let mut interfaces = Vec::new();
        for _i in 0..count {
            interfaces.push(reader.parse_u2());
        }
        trace!("Read interfaces count: {}, length: {}", count, interfaces.len());
        return interfaces;
    }

    pub fn minor_version(&self) -> u16 {
        self.minor_version
    }

    pub fn major_version(&self) -> u16 {
        self.major_version
    }

    pub fn constant_pool(&self) -> Rc<ConstantPool> {
        self.constant_pool.clone()
    }

    pub fn access_flags(&self) -> u16 {
        self.access_flags
    }

    pub fn fields(&self) -> &Vec<MemberInfo> {
        &self.fields
    }

    pub fn methods(&self) -> &Vec<MemberInfo> {
        &self.methods
    }

    pub fn class_name(&self) -> &String {
        self.constant_pool.get_class_name(self.this_class as usize)
    }

    pub fn supper_class_name(&self) -> &String {
        self.constant_pool.get_class_name(self.super_class as usize)
    }

    pub fn interface_names(&self) -> Vec<&String> {
        let mut names = Vec::new();
        for idx in &self.interfaces {
            names.push(self.constant_pool.get_class_name(*idx as usize));
        }
        return names;
    }
}

impl MemberInfo {
    pub fn read_members(reader: &mut BufReader<Box<dyn Read>>, cp: Rc<ConstantPool>) -> Vec<MemberInfo> {
        let count = reader.parse_u2();
        trace!("Read members count: {}", count);
        let mut member_infos = Vec::new();
        for _i in 0..count {
            member_infos.push(MemberInfo::read_member(reader, cp.clone()));
        }
        return member_infos
    }

    pub fn read_member(reader: &mut BufReader<Box<dyn Read>>, cp: Rc<ConstantPool>) -> MemberInfo {
        let access_flags = reader.parse_u2();
        let name_index = reader.parse_u2();
        let descriptor_index = reader.parse_u2();
        let attributes = AttributeInfo::read_attributes(reader, cp.clone());
        trace!(" member access: {}, name: {}, desc: {}", access_flags, name_index, descriptor_index);
        MemberInfo {
            cp,
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        }
    }

    pub fn access_flags(self) -> u16 {
        self.access_flags
    }

    pub fn name(&self) -> String {
        self.cp.get_utf8(self.name_index as usize).into()
    }

    pub fn descriptor(&self) -> String {
        self.cp.get_utf8(self.descriptor_index as usize).into()
    }
}