use std::fs;
use std::path::{MAIN_SEPARATOR, PathBuf};
use std::io::{BufReader, Read, Cursor};
use std::fs::File;

const PATH_LIST_SEPARATOR: &str = ":";

pub trait Entry {
    fn read_class(&self, class_name: &str) -> Result<BufReader<Box<dyn Read>>, String>;

    fn string(&self) -> String;
}

pub fn new_entry(path: &str) -> Box<dyn Entry> {
    if path.contains(PATH_LIST_SEPARATOR) {
        return new_composite_entry(path);
    }
    if path.ends_with("*") {
        return new_wildcard_entry(path);
    }
    if path.ends_with(".jar") || path.ends_with(".JAR")
        || path.ends_with(".zip") || path.ends_with(".ZIP") {
        return new_zip_entry(path);
    }
    return new_dir_entry(path);
}

struct DirEntry {
    abs_dir: PathBuf
}

fn new_dir_entry(path: &str) -> Box<DirEntry> {
    let abs_dir = fs::canonicalize(&PathBuf::from(path)).unwrap();
    return Box::new(DirEntry { abs_dir });
}

impl Entry for DirEntry {
    fn read_class(&self, class_name: &str) -> Result<BufReader<Box<dyn Read>>, String> {
        let file_name = self.abs_dir.join(class_name);
        return match File::open(file_name) {
            Ok(file) => Ok(BufReader::new(Box::new(file))),
            Err(e) => Err(e.to_string())
        };
    }

    fn string(&self) -> String {
        String::from(self.abs_dir.to_str().unwrap())
    }
}

struct ZipEntry {
    abs_dir: PathBuf
}

fn new_zip_entry(path: &str) -> Box<ZipEntry> {
    let abs_dir = fs::canonicalize(&PathBuf::from(path)).unwrap();
    return Box::new(ZipEntry { abs_dir });
}

impl Entry for ZipEntry {
    fn read_class<'a>(&'a self, class_name: &'a str) -> Result<BufReader<Box<dyn Read>>, String> {
        let file = fs::File::open(self.abs_dir.clone()).unwrap();
        trace!("Read zip file: {:?}", self.abs_dir);
        let mut archive = zip::ZipArchive::new(file).unwrap();
        return match archive.by_name(class_name.clone()) {
            Ok(mut file) => {
                trace!("Found class {} in zip", class_name.clone());
                let mut bytes = Vec::new();
                match file.read_to_end(&mut bytes) {
                    Ok(_) => Ok(BufReader::new(Box::new(Cursor::new(bytes)))),
                    Err(_) => Err(format!("class not found: {}", class_name))
                }
            },
            Err(..) => Err(format!("class not found: {}", class_name)),
        };
    }

    fn string(&self) -> String {
        String::from(self.abs_dir.to_str().unwrap())
    }
}

type CompositeEntry = Vec<Box<dyn Entry>>;

fn new_composite_entry(path_list: &str) -> Box<CompositeEntry> {
    let mut composite = Vec::new();
    let paths: Vec<&str> = path_list.split(PATH_LIST_SEPARATOR).collect();
    for path in paths {
        composite.push(new_entry(path));
    }
    trace!("composite entry size: {}", composite.len());
    return Box::new(composite);
}

impl Entry for CompositeEntry {
    fn read_class(&self, class_name: &str) -> Result<BufReader<Box<dyn Read>>, String> {
        for entry in self {
            trace!("Try read class {} from {}", class_name, entry.string());
            let result = entry.read_class(class_name);
            if result.is_ok() {
                return result;
            }
        }
        return Err(format!("class not found: {}", class_name));
    }

    fn string(&self) -> String {
        let mut result = Vec::new();
        for entry in self {
            result.push(entry.string());
        }
        return result.join(MAIN_SEPARATOR.to_string().as_str());
    }
}

pub fn new_wildcard_entry(path_list: &str) -> Box<CompositeEntry> {
    let mut base_dir = path_list.to_string();
    base_dir.pop(); // remove *
    let mut composite = Vec::new();
    trace!("Walk dir: {}", base_dir);
    let read_dir = fs::read_dir(base_dir);
    if read_dir.is_err() {
        return Box::new(composite);
    }
    for entry in read_dir.unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = fs::metadata(&path).unwrap();
        if !metadata.is_file() {
            continue;
        }

        let file_name = path.to_str().unwrap();
        trace!("Found path: {:?}.", file_name);
        if file_name.ends_with(".jar") || file_name.ends_with(".JAR") {
            let jar_entry = new_zip_entry(file_name);
            composite.push(jar_entry as Box<dyn Entry>);
        }
    }
    trace!("wildcard entry size: {}", composite.len());
    return Box::new(composite);
}