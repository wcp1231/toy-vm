use crate::entry::{Entry, new_wildcard_entry, new_entry};
use std::io::{BufReader, Read};
use std::path::{PathBuf, Path};
use std::env;

pub struct Classpath {
    boot_classpath: Box<Entry>,
    ext_classpath: Box<Entry>,
    user_classpath: Box<Entry>,
}

impl Classpath {
    pub fn parse(jre_option: Option<&str>, cp_option: Option<&str>) -> Box<Classpath> {
        let (boot_classpath, ext_classpath) = Classpath::parse_boot_and_ext_classpath(jre_option);
        trace!("Parsed boot and ext classpath");
        let user_classpath = Classpath::parse_user_classpath(cp_option);
        trace!("Parsed user classpath");
        return Box::new(Classpath {
            boot_classpath,
            ext_classpath,
            user_classpath,
        });
    }

    fn parse_boot_and_ext_classpath(jre_option: Option<&str>) -> (Box<Entry>, Box<Entry>) {
        let jre_dir = Classpath::get_jre_dir(jre_option);
        trace!("Get jre: {}", jre_dir);
        let jre_lib_path = PathBuf::from(jre_dir.as_str()).join("lib").join("*");
        trace!("Get jre lib path: {:?}", jre_lib_path.to_str());
        let boot_classpath = new_wildcard_entry(jre_lib_path.to_str().unwrap());
        let jre_ext_path = PathBuf::from(jre_dir.as_str()).join("lib").join("ext").join("*");
        trace!("Get jre ext path: {:?}", jre_ext_path.to_str());
        let ext_classpath = new_wildcard_entry(jre_ext_path.to_str().unwrap());
        return (boot_classpath, ext_classpath);
    }

    fn parse_user_classpath(cp_option: Option<&str>) -> Box<Entry> {
        return new_entry(cp_option.unwrap_or("."));
    }

    fn get_jre_dir(jre_option: Option<&str>) -> String {
        let jre = jre_option.unwrap_or("./jre");
        if Path::new(jre).exists() {
            return jre.to_string();
        }
        if let Some(java_home) = env::var_os("JAVA_HOME") {
            let mut jre = java_home.to_str().unwrap().to_string();
            jre.push_str("/jre");
            return jre;
        }
        panic!("Can not find jre folder!");
    }

    pub fn read_class(self, class_name: &String) -> Result<BufReader<Box<dyn Read>>, String> {
        let mut class_name = class_name.clone();
        class_name.push_str(".class");
        let class_name = class_name.as_str();
        //let class_name = format!("{}.class", class_name).as_str();
        trace!("Try to read class {} from boot classpath", class_name);
        let ret = self.boot_classpath.read_class(class_name);
        if ret.is_ok() {
            return ret;
        }
        trace!("Try to read class {} from ext classpath", class_name);
        let ret = self.ext_classpath.read_class(class_name);
        if ret.is_ok() {
            return ret;
        }
        trace!("Try to read class {} from user classpath", class_name);
        return self.user_classpath.read_class(class_name);
    }

    pub fn string(&self) -> String {
        return self.user_classpath.string();
    }
}

