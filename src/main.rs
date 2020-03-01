#[macro_use]
extern crate log;

use clap::{App, Arg, crate_version, ArgMatches, Values};
use crate::classpath::Classpath;
use std::io::{Read};

mod classpath;
mod entry;

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

    let class_data = cp.read_class(class_to_load);
    match class_data {
        Ok(mut data) => {
            let mut bytes = Vec::new();
            match data.read_to_end(&mut bytes) {
                Ok(_) => info!("class data: {:?}", bytes),
                _ => {}
            }

        },
        Err(err) => warn!("Could not find or load main class {}, err: {}", class, err),
    }
}