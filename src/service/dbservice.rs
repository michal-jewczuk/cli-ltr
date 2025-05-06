use crate::models::test;
use rusqlite::Connection;


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
}

#[derive(Debug, Clone)]
struct QuestionE {
    id: usize,
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

pub fn get_fresh(conn: &Connection) -> Result<Vec<(String, String)>, rusqlite::Error> {
    let select = "SELECT id, name, date FROM exam WHERE status = 'NOT_STARTED'";
    let mut stmt = conn.prepare(select)?;

    let rows = stmt.query_map([], |row| Ok(
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

pub fn get_q_by_id(conn: &Connection, id: String) -> Option<test::TestModel> {
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
    let stmt_q = conn.prepare("SELECT id, text, a1, a2, a3, a4, correct FROM question WHERE examid = :id");
    let mut binding = stmt_q.expect("WHAT");
    let rows = binding.query_map([id.as_str()], |row| {Ok(
            QuestionE {
                id: row.get(0)?,
                text: row.get(1)?,
                a1: row.get(2)?,
                a2: row.get(3)?,
                a3: row.get(4)?,
                a4: row.get(5)?,
                correct: row.get(6)?,
            }
    )}).unwrap();
    let mut questions: Vec<test::QuestionModel> = vec![];
    for row in rows {
        let _ = row.map(|r| questions.push(r.to_model()));
    }

    Some(row.unwrap().to_model(questions))
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
            "Lets imagine that you see your brother for the first time today and it is 1 pm. How do you greet him?",
            "Good evening",
            "Good morning",
            "Hi, do we know each other?",
            "Yo bro, shouldn't you be in Buenos Aires right now?",
            3,
            1,
        ),
        (
            "This is the ... I am telling you this!",
            "current time",
            "previous time",
            "last time",
            "any timme",
            2,
            1,
        ),
        (
            "How would you tell somebody to go and do something?",
            "Could you just move, please?",
            "Just do it!",
            "Move your behind and get the shit done",
            "Just stay there and let things happen on their own",
            2,
            2,
        ),
        (
            "Stop ... at the world through the pink glasses.",
            "seeing",
            "looking",
            "viewing",
            "he see",
            1,
            2,
        ),
        (
            "Why do policemen walk in paris?",
            "To arrest you twice as fast for hate speech",
            "Who could possibly know that",
            "Two is better than one",
            "One can read and the other can write",
            3,
            2,
        ),
        (
            "I wish you ... so dumm.",
            "is",
            "weren't",
            "was not",
            "were",
            1,
            3,
        ),
        (
            "What is the best way to describe the following situation: A woman in her late twentees or early thirties is walking down the street early mornig. Her hair are a mess, her make up is well, like her hair and she is holding her high heels in one hand while covering her face with the other.",
            "Early morning jogging",
            "Stroll through the park",
            "Walk of shame",
            "My kind of a woman",
            2,
            3,
        ),
        (
            "What could be your reaction to learning the correct answer to the previouis question? Pick the one that fits best.",
            "I should have known that!",
            "Really? I had no idea!",
            "So it was not my kind of a woman?",
            "One of the above",
            3,
            3,
        ),
        (
            "What is the proper way to say: '3 + 3' 'is six' or 'are six'?",
            "is six",
            "are six",
            "is nine",
            "math? oh, come on!",
            2,
            3,
        ),
    ];

    data.iter().for_each(|r| {
        let _ = conn.execute(
            "INSERT INTO question (text, a1, a2, a3, a4, correct, examid)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (r.0, r.1, r.2, r.3, r.4, r.5, r.6),
        );
    });
}

