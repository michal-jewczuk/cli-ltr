use crate::models::test;

use std::io;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

pub fn import_test_files() {
    let files = read_test_files();
    let tests = files.iter()
        .map(|f| read_file_content(f))
        .collect::<Vec<test::TestModel>>();
    tests.iter().for_each(|t| println!("{:#?}", t));
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
        let _ = fs::remove_file(path);
    }
}

fn read_file_content(path: &PathBuf) -> test::TestModel {
    // have a method that checks file structure
    // title, a least 1 question
    // each question with 4 answers and only one answer with a + as correct
    let message: String = fs::read_to_string(path).unwrap();
    let split_t_q: Vec<&str> = message.split("====").collect();
    let questions = &split_t_q[1..].iter()
        .map(|qf| parse_question(qf))
        .collect::<Vec<test::QuestionModel>>();

    test::TestModel::new(String::from("0"), split_t_q[0].trim().to_string(), questions.to_vec())
}

fn parse_question(unparsed: &str) -> test::QuestionModel {
    let splits: Vec<&str> = unparsed.split("----").collect();
    let mut idx = 0;
    let mut correct = 0;
    let mut answers = vec![];
    splits[1].split("\n")
        .filter(|l| l.len() > 0)
        .for_each(|q| {
            if q.starts_with("+") {
                correct = idx;
            }
            answers.push(q[1..].trim().to_string());
            idx += 1;
        });

    test::QuestionModel::new(splits[0].trim().to_string(), answers, correct.try_into().unwrap())
}
