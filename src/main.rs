#[macro_use]
extern crate log;

use clap::{App, Arg, crate_version, ArgMatches};

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
        .arg(Arg::with_name("class"))
        .arg(Arg::with_name("args")
            .multiple(true))
        .get_matches();

    startVM(matches);
}

fn startVM(matches: ArgMatches) {
    let args: Vec<&str> = matches.values_of("args").unwrap().collect();
    info!("classpath:{} class:{} args:{:?}",
          matches.value_of("classpath").unwrap(),
          matches.value_of("class").unwrap(),
          args
    )
}