use crate::models::test;
use crate::service::dbservice;

use std::io;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use rust_i18n::t;
use rusqlite::Connection;

pub fn import_test_files(locale: &str, conn: &Connection) -> Vec<String> {
    let mut logs: Vec<String> = vec![];
    let files = read_test_files();
    files.iter()
        .map(|f| format!("{}: {:?}", t!("import.parsing", locale = locale), f.file_name().unwrap()))
        .for_each(|l| logs.push(l));

    let tests = files.iter()
        .map(|f| read_file_content(f))
        .collect::<Vec<test::TestModel>>();

    let mut valid_logs: Vec<String> = vec![];
    let mut invalid_logs: Vec<String> = vec![];
    let mut idx = 0;
    for test in &tests {
        if validate_structure(&test) {
            valid_logs.push(format!("{}: {:?}", t!("import.valid", locale = locale), files[idx].file_name().unwrap()))
        } else {
            invalid_logs.push(format!("{}: {:?}", t!("import.invalid", locale = locale), files[idx].file_name().unwrap()))
        }
        idx += 1;
    }
    logs.push(String::from(" "));
    valid_logs.into_iter().for_each(|l| logs.push(l));
    logs.push(String::from(" "));
    invalid_logs.into_iter().for_each(|l| logs.push(l));
    logs.push(String::from(" "));

    tests.iter()
        .filter(|t| validate_structure(t))
        .map(|t| save_to_db(t, locale, conn))
        .for_each(|l| logs.push(l));
    
    // move valid and invalid to the same directory?
    files.into_iter().for_each(|f| move_to_finished(f, false));

    logs.push(String::from(" "));
    logs.push(format!("{}", t!("import.finished", locale = locale)));
    logs.iter()
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
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
    //println!("Moving file to: {:?}", moved.clone());
    let _ = fs::copy(&path, moved);
    if remove {
        let _ = fs::remove_file(path);
    }
}

fn read_file_content(path: &PathBuf) -> test::TestModel {
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
    let mut correct = 10;
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

fn validate_structure(model: &test::TestModel) -> bool {
    if model.title.len() < 2 || model.questions.len() < 2 {
        return false;
    }

    for question in &model.questions {
        if question.question.len() < 2 || question.correct == 10 || question.answers.len() != 4 {
            return false;
        }
    }

    return true;
}

fn save_to_db(model: &test::TestModel, locale: &str, conn: &Connection) -> String {
    match dbservice::save_new_test(conn, model) {
        Ok(_) => format!("{}: {:?}",t!("import.save", locale = locale),  model.title),
        Err(e) => format!("Error saving to db test: {} with error: {}", model.title, e),
    }
}

