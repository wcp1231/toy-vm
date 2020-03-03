#[macro_use]
extern crate log;

mod attribute_info;
mod classpath;
mod class_file;
mod constant_pool;
mod entry;
mod jvm_stack;
mod read_bytes_ext;

use clap::{App, Arg, crate_version, ArgMatches, Values};
use std::borrow::Borrow;
use crate::classpath::Classpath;
use crate::class_file::ClassFile;

fn main() {
    env_logger::init();

    let matches = App::new("Toy-vm")
        .version(crate_version!())
        .author("wcp1231 <wcp1231@gmail.com>")
        .about("A Toy jvm")
        .arg(Arg::with_name("classpath")
            .short("cp")
            .long("classpath")
            .takes_value(true)
            .help("Classpath"))
        .arg(Arg::with_name("jre_option")
            .long("Xjre")
            .takes_value(true)
            .help("Path to jre"))
        .arg(Arg::with_name("class"))
        .arg(Arg::with_name("args")
            .multiple(true))
        .get_matches();

    start_vm(matches);
}

fn start_vm(matches: ArgMatches) {
    let jre_option = matches.value_of("jre_option");
    let classpath = matches.value_of("classpath");
    let args: Vec<&str> = matches.values_of("args").unwrap_or(Values::default()).collect();
    trace!("classpath:{} class:{} args:{:?}",
          classpath.unwrap_or("."),
          matches.value_of("class").unwrap(),
          args
    );

    let cp = Classpath::parse(jre_option, classpath);
    trace!("Get classpath: {}", cp.string());
    let class = matches.value_of("class").unwrap();
    let class_to_load = class.replace(".", "/");
    trace!("class to load: {}", class_to_load);

    let class_file = load_class(class_to_load, cp);
    print_class_info(&class_file);
}

fn load_class(class_name: String, cp: Box<Classpath>) -> Box<ClassFile> {
    return match cp.read_class(class_name.borrow()) {
        Ok(mut reader) => ClassFile::parse(&mut reader),
        Err(err) => {
            warn!("Could not find or load main class {}, err: {}", class_name, err);
            panic!()
        },
    };
}

fn print_class_info(class_file: &ClassFile) {
    info!("version: {}.{}", class_file.major_version(), class_file.minor_version());
    info!("constant count: {}", class_file.constant_pool().len());
    info!("access flags: {:#018b}", class_file.access_flags());
    info!("this class: {}", class_file.class_name());
    info!("super class: {}", class_file.supper_class_name());
    info!("interfaces: {:?}", class_file.interface_names());
    info!("fields count: {}", class_file.fields().len());
    for field in class_file.fields() {
        info!("  {} -> {}", field.name(), field.descriptor());
    }
    info!("methods count: {}", class_file.methods().len());
    for field in class_file.methods() {
        info!("  {} -> {}", field.name(), field.descriptor());
    }
}