use crate::models::test;
use rusqlite::Connection;
use std::time::{Duration, SystemTime};
use chrono::Utc;


#[derive(Debug)]
struct TestE {
    id: usize,
    name: String,
    date: String,
}

impl TestE {
    fn get_full_name(&self) -> String {
        format!("[{}] {}", self.date, self.name)
    }

    fn get_short(&self) -> (String, String) {
        (format!("{}", self.id), self.get_full_name())
    }

    fn to_model(&self, questions: Vec<test::QuestionModel>) -> test::TestModel {
        test::TestModel::new(
            format!("{}", self.id),
            self.get_full_name(),
            questions,
        )
    }

    fn to_result(&self, answers: Vec<test::AnswerModel>, total_time: u64) -> test::ResultModel {
        test::ResultModel::new(
            format!("{}", self.id),
            self.get_full_name(),
            answers,
            total_time,
        )
    }
}

#[derive(Debug, Clone)]
struct QuestionE {
    text: String,
    a1: String,
    a2: String,
    a3: String,
    a4: String,
    correct: usize,
}

impl QuestionE {
    fn to_model(&self) -> test::QuestionModel {
        test::QuestionModel::new(
            self.text.clone(),
            vec![self.a1.clone(), self.a2.clone(), self.a3.clone(), self.a4.clone()],
            self.correct as u8,
        )
    }
}

#[derive(Debug, Clone)]
struct AnswerE {
    given: usize,
    time: usize,
    text: String,
    a1: String,
    a2: String,
    a3: String,
    a4: String,
    correct: usize,
}

impl AnswerE {
    fn to_model(&self) -> test::AnswerModel {
        let mut g = Some(self.given);
        if self.given == 42 {
            g = None;
        }
        test::AnswerModel::new(
            self.text.clone(),
            vec![self.a1.clone(), self.a2.clone(), self.a3.clone(), self.a4.clone()],
            self.correct as u8,
            g,
            self.given == self.correct,
            self.time as u64,
        )
    }
}

pub fn get_by_status(conn: &Connection, status: &str) -> Result<Vec<(String, String)>, rusqlite::Error> {
    let select = "SELECT id, name, date FROM exam WHERE status = :status";
    let mut stmt = conn.prepare(select)?;

    let rows = stmt.query_map([status], |row| Ok(
        TestE {
            id: row.get(0)?,
            name: row.get(1)?,
            date: row.get(2)?,
        }
    )).unwrap();
    let mut results: Vec<(String, String)> = Vec::new();
    for row  in rows {
        let _ = row.map(|r| results.push(r.get_short()));
    }

    Ok(results)
}

pub fn get_test_by_id(conn: &Connection, id: String) -> Option<test::TestModel> {
    let stmt_t = conn.prepare("SELECT id, name, date FROM exam WHERE exam.id = :id");
    let row = stmt_t.expect("WHAT").query_row([id.as_str()], |row| Ok(
        TestE {
            id: row.get(0)?,
            name: row.get(1)?,
            date: row.get(2)?,
        }
    ));

    if row.is_err() {
        return None;
    }

    // TODO learn why this has to be done that way and how to correct it
    let stmt_q = conn.prepare("SELECT text, a1, a2, a3, a4, correct FROM question WHERE examid = :id ORDER BY number ASC");
    let mut binding = stmt_q.expect("WHAT");
    let rows = binding.query_map([id.as_str()], |row| {Ok(
            QuestionE {
                text: row.get(0)?,
                a1: row.get(1)?,
                a2: row.get(2)?,
                a3: row.get(3)?,
                a4: row.get(4)?,
                correct: row.get(5)?,
            }
    )}).unwrap();
    let mut questions: Vec<test::QuestionModel> = vec![];
    for row in rows {
        let _ = row.map(|r| questions.push(r.to_model()));
    }

    Some(row.unwrap().to_model(questions))
}

pub fn get_result_by_id(conn: &Connection, id: String) -> Option<test::ResultModel> {
    let stmt_t = conn.prepare("SELECT id, name, date FROM exam WHERE exam.id = :id");
    let row = stmt_t.expect("WHAT").query_row([id.as_str()], |row| Ok(
        TestE {
            id: row.get(0)?,
            name: row.get(1)?,
            date: row.get(2)?,
        }
    ));

    if row.is_err() {
        return None;
    }

    let stmt_q = conn.prepare("SELECT result.given, result.time, question.text, question.a1, question.a2, question.a3, question.a4, question.correct 
        FROM result JOIN question ON result.qnumber = question.number WHERE question.examid = :id AND
        result.date = (SELECT MAX(date) FROM result GROUP BY examid HAVING examid = :id)");
    let mut binding = stmt_q.expect("WHAT");
    let rows = binding.query_map([id.as_str()], |row| {Ok(
            AnswerE {
                given: row.get(0)?,
                time: row.get(1)?,
                text: row.get(2)?,
                a1: row.get(3)?,
                a2: row.get(4)?,
                a3: row.get(5)?,
                a4: row.get(6)?,
                correct: row.get(7)?,
            }
    )}).unwrap();
    let mut answers: Vec<test::AnswerModel> = vec![];
    for row in rows {
        let _ = row.map(|r| answers.push(r.to_model()));
    }
    let total_time: u64 = answers.iter().map(|a| a.time).sum();

    Some(row.unwrap().to_result(answers, total_time))
}

pub fn update_status(conn: &Connection, id: String, status: &str) -> Result<(), Box<dyn std::error::Error>> {
    let stmt = conn.prepare("UPDATE exam SET status = :status WHERE id = :id");
    let _ = stmt?.execute([status, id.as_str()])?;
    Ok(())
}

pub fn save_result(conn: &Connection, result: test::ResultModel) -> Result<(), Box<dyn std::error::Error>> {
    let date = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or(Duration::ZERO).as_secs();
    let mut idx = 0;
    result.answers.iter()
        .map(|a| {
            idx += 1;
            (&result.id, idx, a.given.unwrap_or(42), a.time, date)
        })
        .for_each(|r| {
            let _ = conn.execute(
                "INSERT INTO result (examid, qnumber, given, time, date) 
                VALUES (?1, ?2, ?3, ?4, ?5)",
                (r.0, r.1, r.2, r.3, r.4),
            );
        });
    Ok(())
}

pub fn save_new_test(conn: &Connection, model: &test::TestModel) -> Result<(), Box<dyn std::error::Error>> {
    let date = Utc::now().format("%Y-%m-%d").to_string();
    let test_t = (&model.title, date, "NOT_STARTED");
    let mut stmt_t = conn.prepare(
        "INSERT INTO exam (name, date, status) VALUES (?1, ?2, ?3) RETURNING id",
    )?;
    let exam_id = stmt_t.query_row(test_t, |r| r.get::<_, i64>(0))?;

    let mut q_num = 0;
    model.questions.iter()
        .map(|q| { 
            q_num += 1;
            (q_num, &q.question, &q.answers[0], &q.answers[1], &q.answers[2], &q.answers[3], q.correct, exam_id)
        })
        .for_each(|r| {
            let _ = conn.execute(
                "INSERT INTO question (number, text, a1, a2, a3, a4, correct, examid)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                (r.0, r.1, r.2, r.3, r.4, r.5, r.6, r.7),
            );
    });

    Ok(())
}

pub fn create_schema(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS exam (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            date TEXT NOT NULL,
            status TEXT NOT NULL
        ) STRICT",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS question (
          id INTEGER PRIMARY KEY,
          number INTEGER,
          text TEXT NOT NULL,
          a1 TEXT NOT NULL,
          a2 TEXT NOT NULL,
          a3 TEXT,
          a4 TEXT,
          correct INTEGER NOT NULL,
          examid INTEGER REFERENCES exam(id)
        ) STRICT",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS result (
            id INTEGER PRIMARY KEY,
            examid INTEGER REFERENCES exam(id),
            qnumber INTEGER,
            given INTEGER,
            time INTEGER,
            date INTEGER
        ) STRICT",
        (),
    )?;

    Ok(())
}

pub fn populate_tests(conn: &Connection) {
    let data = &[
        ("English idioms with a twist", "2025-03-07", "NOT_STARTED"),
        ("Verbs and stuff", "2025-02-28", "NOT_STARTED"),
        ("Week exam #2", "2025-02-27", "NOT_STARTED"),
        ("Week exam #1", "2025-01-03", "FINISHED"),
        ("Adjectives", "2025-01-03", "FINISHED"),
        ("Reading between the lines", "2025-01-08", "FINISHED"),
        ("Nouns or nuns", "2025-01-12", "FINISHED"),
    ];

    data.iter().for_each(|r| {
        let _ = conn.execute(
            "INSERT INTO exam (name, date, status)
            VALUES (?1, ?2, ?3)",
            (r.0, r.1, r.2),
        );
    });
}


pub fn populate_questions(conn: &Connection) {
    let data = &[
        ( 
            1,
            "Lets imagine that you see your brother for the first time today and it is 1 pm. How do you greet him?",
            "Good evening",
            "Good morning",
            "Hi, do we know each other?",
            "Yo bro, shouldn't you be in Buenos Aires right now?",
            3,
            1,
        ),
        (
            2,
            "This is the ... I am telling you this!",
            "current time",
            "previous time",
            "last time",
            "any timme",
            2,
            1,
        ),
        (
            1,
            "How would you tell somebody to go and do something?",
            "Could you just move, please?",
            "Just do it!",
            "Move your behind and get the shit done",
            "Just stay there and let things happen on their own",
            2,
            2,
        ),
        (
            2,
            "Stop ... at the world through the pink glasses.",
            "seeing",
            "looking",
            "viewing",
            "he see",
            1,
            2,
        ),
        (
            3,
            "Why do policemen walk in pairs?",
            "To arrest you twice as fast for hate speech",
            "Who could possibly know that",
            "Two is better than one",
            "One can read and the other can write",
            3,
            2,
        ),
        (
            1,
            "I wish you ... so dumm.",
            "is",
            "weren't",
            "was not",
            "were",
            1,
            3,
        ),
        (
            2,
            "What is the best way to describe the following situation: A woman in her late twentees or early thirties is walking down the street early mornig. Her hair are a mess, her make up is well, like her hair and she is holding her high heels in one hand while covering her face with the other.",
            "Early morning jogging",
            "Stroll through the park",
            "Walk of shame",
            "My kind of a woman",
            2,
            3,
        ),
        (
            3,
            "What could be your reaction to learning the correct answer to the previouis question? Pick the one that fits best.",
            "I should have known that!",
            "Really? I had no idea!",
            "So it was not my kind of a woman?",
            "One of the above",
            3,
            3,
        ),
        (
            4,
            "What is the proper way to say: '3 * 3' 'is six' or 'are six'?",
            "is six",
            "are six",
            "is nine",
            "math? oh, come on!",
            2,
            3,
        ),
        (
            1,
           "Are you in the right class?",
            "Yes",
            "No",
            "I have no idea",
            "Maybe",
            0,
            4,
        ),
        (
            1,
            "Less is more. True or false?",
            "True",
            "False",
            "I have no idea",
            "It depends",
            3,
            5,
        ),
        (
            2,
            "The grass is always ... on the other side",
            "bigger",
            "greener",
            "bushier",
            "green",
            1,
            5,
        ),
        (
            3,
            "If he is younger than me then I am ... than him",
            "as young as",
            "younger",
            "old",
            "older",
            3,
            5,
        ),
        (
            1,
            "A deep dive into something means",
            "to swim underwater really deep",
            "to jump into water from a high place",
            "to focus someones attention on one topic in order to understand it very well",
            "all of the above",
            2,
            6,
        ),
        (
            2,
            "Which number cannot be represented by the phrase: 'two four two zero'",
            "2440",
            "4420",
            "4400",
            "2420",
            0,
            6,
        ),
        (
            1,
            "What is a truck?",
            "a car but not a car, a bigger car",
            "lower part of the tree",
            "a main branch in git repository",
            "a vihickle",
            0,
            7,
        ),
        (
            2,
            "Swims on water and donald is the name",
            "dack",
            "dak",
            "duck",
            "chicken",
            2,
            7,
        ),
    ];

    data.iter().for_each(|r| {
        let _ = conn.execute(
            "INSERT INTO question (number, text, a1, a2, a3, a4, correct, examid)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (r.0, r.1, r.2, r.3, r.4, r.5, r.6, r.7),
        );
    });
}

pub fn populate_answers(conn: &Connection) {
    let data = vec![
        (4, 1, 1, 56, 10001),
        (5, 1, 2, 66, 10011),
        (5, 2, 1, 120, 10011),
        (5, 3, 2, 61, 10011),
        (6, 1, 2, 9, 10101),
        (6, 2, 0, 12, 10101),
        (7, 1, 3, 100, 11001),
        (7, 2, 0, 23, 11001),
    ];

    data.iter().for_each(|r| {
        let _ = conn.execute(
            "INSERT INTO result (examid, qnumber, given, time, date) 
            VALUES (?1, ?2, ?3, ?4, ?5)",
            (r.0, r.1, r.2, r.3, r.4),
        );
    });
}

