use std::io;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

pub fn import_test_files() {
    let files = read_test_files();
    files.into_iter().for_each(|f| move_to_finished(f, false));

    println!("Import finished!")
}

pub fn read_test_files() -> Vec<PathBuf> {
    let imp_path = Path::new("./import");

    if !imp_path.try_exists().unwrap() {
        match fs::create_dir(imp_path) {
           Err(_) => println!("Could not create dir"),
           _ => ()
        }
        return vec![];
    }

    let mut files = fs::read_dir(imp_path).unwrap()
        .map(|item| item.map(|i| i.path()))
        .filter(|p| !p.is_err())
        .map(|p| p.unwrap())
        .filter(|p| !p.is_dir())
        .collect::<Vec<_>>();
    files.sort();
    files
}

fn move_to_finished(path: PathBuf, remove: bool) { 
    let mut moved = PathBuf::new();
    moved.push(r"./finished");
    moved.push(path.file_name().unwrap());
    println!("Moving file to: {:?}", moved.clone());
    let _ = fs::copy(&path, moved);
    if remove {
        println!("Deleting file");
        let _ = fs::remove_file(path);
    }
}
