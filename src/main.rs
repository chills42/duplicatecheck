use std::io;
use std::io::Read;
use std::fs::{self, DirEntry, File};
use std::path::Path;
use std::hash::{Hash, Hasher};
use std::vec::Vec;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::cell::RefCell;

fn main() {
    let records = RefCell::new(HashMap::new());

    let list_name = |d: &DirEntry| {
        if let Ok(hash) = hash(&d.path()) {
            let mut rec = records.borrow_mut();
            let set = rec.entry(hash).or_insert(Vec::new());
            set.push(d.path());
        }
    };

    visit_dirs(Path::new("./"), &list_name).ok();
    let values = records.borrow();
    for set in values.values() {
        if set.len() > 1 {
            println!("{:?}", set);
        }
    }
}

fn hash(p: &Path) -> io::Result<u64> {
    let mut s = DefaultHasher::new();
    let mut f = try!(File::open(p));
    loop {
        let mut buffer = [0; 8192];
        if let Ok(size) = f.read(&mut buffer) {
            if size > 0 {
                buffer[0..size].hash(&mut s);
            }
            else {
                break;
            }
        }
        else {
            break;
        }
    }
    Ok(s.finish())
}

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            let path = entry.path();
            if path.is_dir() {
                try!(visit_dirs(&path, cb));
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
