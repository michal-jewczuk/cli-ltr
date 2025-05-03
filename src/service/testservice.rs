use crate::models::test;
use rusqlite::Connection;

pub fn init_table() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    let _ = create_schema(&conn);
    let _ = populate_tests(&conn);
    let _ = populate_questions(&conn);
    conn
}

fn create_schema(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "CREATE TABLE exam (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            date TEXT NOT NULL
        ) STRICT",
        (),
    )?;

    conn.execute(
        "CREATE TABLE question (
          id INTEGER PRIMARY KEY
          text TEXT NOT NULL,
          a1 TEXT NOT NULL,
          a2 TEXT NOT NULL,
          a3 TEXT,
          a4 TEXT,
          correct NUMBER NOT NULL,
          FOREIGN KEY(examid) REFERENCES exam(id)
        ) STRICT",
        (),
    )?;

    Ok(())
}

fn populate_tests(conn: &Connection) {
    let data = &[
        ("English idioms with a twist", "2025-03-07"),
        ("Verbs and stuff", "2025-02-28"),
        ("Week exam #2", "2025-02-27"),
    ];

    data.iter().for_each(|r| {
        let _ = conn.execute(
            "INSERT INTO exam (name, date)
            VALUES (?1, ?2)",
            (r.0, r.1),
        );
    });
}

fn populate_questions(conn: &Connection) {
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
    ];

    data.iter().for_each(|r| {
        let _ = conn.execute(
            "INSERT INTO question (text, a1, a2, a3, a4, correct, examid)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (r.0, r.1, r.2, r.3, r.4, r.5, r.6),
        );
    });
}

pub fn get_fresh(conn: &Connection) -> Result<Vec<(String, String)>, rusqlite::Error> {
    let select = "SELECT id, name, date FROM exam";
    let mut stmt = conn.prepare(select)?;

    let rows = stmt.query_map([], |row| Ok(
            (row.get::<_, usize>(0), row.get::<_, String>(1), row.get::<_, String>(2))
        ))?;
    let mut results: Vec<(String, String)> = Vec::new();
    for name_r  in rows {
        let tmp = name_r.unwrap();
        let id = tmp.0.map_or(0, |v| v);
        let date = tmp.2.map_or(String::from(""), |v| v);
        let name = tmp.1.map_or(String::from("Invalid"), |v| v); 
        results.push((format!("{}", id), format!("[{}] {}", date, name)));
    }

    Ok(results)
}

struct TestE {
    id: usize,
    name: String,
    date: String,
}

// TODO get back to it once TestModel is in String
//pub fn get_q_by_id<'a>(conn: &Connection, id: String) -> Option<test::TestModel<'a>> {
//   let mut stmt_t = conn.prepare("SELECT id, name, date FROM exam WHERE exam.id = :id");
//   let tE = stmt_t.expect("PANICKED").query_row([":id", id.as_str()], |row| {Ok(
//           TestE {
//               //id: row.get::<_, usize>(0).map_or(0, |v| v),
//               id: row.get(0)?,
//               name: row.get(1).map_or(String::from("Ooops"), |v| v),
//               date: row.get(2).map_or(String::from("0000-00-00"), |v| v),
//           }
//   )}).unwrap();
//
//   let test = test::TestModel::new(
//       format!("{}", tE.id).clone().as_str(),
//       format!("[{}] {}", tE.date, tE.name).clone().as_str(),
//       vec![]
//   );
//
//   Some(test)
//}

pub fn get_to_do<'a>() -> Vec<(String, String)> {
    vec![
        test::TestModel {
            id: "xyz",
            title: "[2025-03-07] English idioms with twist",
            questions: vec![
            ],
        },
        test::TestModel {
            id: "abc",
            title: "[2025-02-28] Verbs and stuff",
            questions: vec![
            ],
        },
        test::TestModel {
            id: "cde",
            title: "[2025-02-27] Week exam #2",
            questions: vec![
            ],
        },
    ].iter()
	.map(|t| (String::from(t.id), String::from(t.title)))
	.collect()
}

pub fn get_by_id<'a>(id: String) -> Option<test::TestModel<'a>> {
    vec![
        test::TestModel {
            id: "1",
            title: "[2025-03-07] English idioms with twist",
            questions: vec![
                test::QuestionModel::new(
                    "Lets imagine that you see your brother for the first time today and it is 1 pm. How do you greet him?",
                    vec![
                        "Good evening",
                        "Good morning",
                        "Hi, do we know each other?",
                        "Yo bro, shouldn't you be in Buenos Aires right now?"
                    ],
                    3,
                ),
                test::QuestionModel::new(
                    "This is the ... I am telling you this!",
                    vec![
                        "current time",
                        "previous time",
                        "last time",
                        "any timme",
                    ],
                    2,
                ),
            ],
        },
        test::TestModel {
            id: "2",
            title: "[2025-02-28] Verbs and stuff",
            questions: vec![
                test::QuestionModel::new(
                    "How would you tell somebody to go and do something?",
                    vec![
                        "Could you just move, please?",
                        "Just do it!",
                        "Move your behind and get the shit done",
                        "Just stay there and let things happen on their own",
                    ],
                    0,
                ),
                test::QuestionModel::new(
                    "... through pink glasses.",
                    vec![
                        "Sees",
                        "Look",
                        "View",
                        "He see",
                    ],
                    1,
                ),
                test::QuestionModel::new(
                    "Why do policemen walk in paris?",
                    vec![
                        "To arrest you twice as fast for hate speech",
                        "Who could possibly know that",
                        "Two is better than one",
                        "One can read and the other can write",
                    ],
                    3,
                ),
            ],
        },
        test::TestModel {
            id: "3",
            title: "[2025-02-27] Week exam #2",
            questions: vec![
                test::QuestionModel::new(
                    "I wish you ... so dumm.",
                    vec![
                        "is",
                        "weren't",
                        "was not",
                        "were",
                    ],
                    1,
                ),
                test::QuestionModel::new(
                    "What is the best way to describe the following situation: A woman in her late twentees or early thirties is walking down the street early mornig. Her hair are a mess, her make up is well, like her hair and she is holding her high heels in one hand while covering her face with the other.",
                    vec![
                        "Early morning jogging",
                        "Stroll through the park",
                        "Walk of shame",
                        "My kind of a woman",
                    ],
                    2,
                ),
                test::QuestionModel::new(
                    "What could be your reaction to learning the correct answer to the previouis question? Pick the one that fits best.",
                    vec![
                        "I should have known that!",
                        "Really? I had no idea!",
                        "So it was not my kind of a woman?",
                        "One of the above",
                    ],
                    3,
                ),
                test::QuestionModel::new(
                    "What is the proper way to say it: '3 + 3' 'is six' or 'are six'?",
                    vec![
                        "is six",
                        "are six",
                        "is nine",
                        "math? oh, come on!",
                    ],
                    2,
                ),
            ],
        },
        test::TestModel {
            id: "r_1",
            title: "[2025-01-03] Week exam #1",
            questions: vec![
                test::QuestionModel::new(
                    "Are you in the right class?",
                    vec![
                        "Yes",
                        "No",
                        "I have no idea",
                        "Maybe",
                    ],
                    0,
                ),
            ],
        },
        test::TestModel {
            id: "r_2",
            title: "[2025-01-03] Adjectives",
            questions: vec![
                test::QuestionModel::new(
                    "Less is more. True or false?",
                    vec![
                        "True",
                        "False",
                        "I have no idea",
                        "It depends",
                    ],
                    3,
                ),
                test::QuestionModel::new(
                    "The grass is always ... on the other side",
                    vec![
                        "bigger",
                        "greener",
                        "bushier",
                        "green",
                    ],
                    1,
                ),
                test::QuestionModel::new(
                    "If he is younger than me then I am ... than him",
                    vec![
                        "as young as",
                        "younger",
                        "old",
                        "older",
                    ],
                    3,
                ),
            ],
        },
        test::TestModel {
            id: "r_3",
            title: "[2025-01-08] Reading between the lines",
            questions: vec![
                test::QuestionModel::new(
                    "A deep dive into something means",
                    vec![
                        "to swim underwater really deep",
                        "to jump into water from a high place",
                        "to focus someones attention on one topic in order to understand it very well",
                        "all of the above",
                    ],
                    2,
                ),
                test::QuestionModel::new(
                    "Which number cannot be represented by the phrase: 'two four two zero'",
                    vec![
                        "2440",
                        "4420",
                        "4400",
                        "2420",
                    ],
                    0,
                ),
            ],
        },
        test::TestModel {
            id: "r_4",
            title: "[2025-01-12] Nouns or nuns",
            questions: vec![
                test::QuestionModel::new(
                    "What is a truck?",
                    vec![
                        "a car but not a car, a bigger car",
                        "lower part of the tree",
                        "a main branch in git repository",
                        "a vihickle",
                    ],
                    0,
                ),
                test::QuestionModel::new(
                    "Swims on water and donald is the name",
                    vec![
                        "dack",
                        "dak",
                        "duck",
                        "chicken",
                    ],
                    2,
                ),
            ],
        },
    ].iter()
        .find(|t| t.id == id)
        .cloned()
}

pub fn get_results_list() -> Vec<(String, String)> {
    vec![
        (String::from("r_1"), String::from("[2025-01-03] Week exam #1")),
        (String::from("r_2"), String::from("[2025-01-03] Adjectives")),
        (String::from("r_3"), String::from("[2025-01-08] Reading between the lines")),
        (String::from("r_4"), String::from("[2025-01-12] Nouns or nuns")),
    ]
}

pub fn get_results_by_id(id: String) -> Option<test::ResultModel> {
    vec![
        test::ResultModel::new(
            String::from("r_1"), String::from("[2025-01-03] Week exam #1"),
            vec![
                test::AnswerModel::new(
                    String::from("Are you in the right class?"),
                    vec![
                        String::from("Yes"),
                        String::from("No"),
                        String::from("I have no idea"),
                        String::from("Maybe"),
                    ],
                    0,
                    Some(1),
                    false,
                    56
                ),
            ],
            56,
        ),
        test::ResultModel::new(
            String::from("r_2"), String::from("[2025-01-03] Adjectives"),
            vec![
                test::AnswerModel::new(
                    String::from("Less is more. True or false?"),
                    vec![
                        String::from("True"),
                        String::from("False"),
                        String::from("I have no idea"),
                        String::from("It depends"),
                    ],
                    3,
                    Some(2),
                    false,
                    66 
                ),
                test::AnswerModel::new(
                    String::from("The grass is always ... on the other side"),
                    vec![
                        String::from("bigger"),
                        String::from("greener"),
                        String::from("bushier"),
                        String::from("green"),
                    ],
                    1,
                    Some(1),
                    true,
                    120 
                ),
                test::AnswerModel::new(
                    String::from("If he is younger than me then I am ... than him"),
                    vec![
                        String::from("as young as"),
                        String::from("younger"),
                        String::from("old"),
                        String::from("older"),
                    ],
                    3,
                    Some(2),
                    false,
                    61 
                ),
            ],
            247,
        ),
        test::ResultModel::new(
            String::from("r_3"), String::from("[2025-01-08] Reading between the lines"),
            vec![
                test::AnswerModel::new(
                    String::from("A deep dive into something means"),
                    vec![
                        String::from("to swim underwater really deep"),
                        String::from("to jump into water from a high place"),
                        String::from("to focus someones attention on one topic in order to understand it very well"),
                        String::from("all of the above"),
                    ],
                    2,
                    Some(2),
                    true,
                    9
                ),
                test::AnswerModel::new(
                    String::from("Which number cannot be represented by the phrase: 'two four two zero'"),
                    vec![
                        String::from("2440"),
                        String::from("4420"),
                        String::from("4400"),
                        String::from("2420"),
                    ],
                    0,
                    Some(0),
                    true,
                    12 
                ),
            ],
            21,
        ),
        test::ResultModel::new(
            String::from("r_4"), String::from("[2025-01-12] Nouns or nuns"),
            vec![
                test::AnswerModel::new(
                    String::from("What is a truck?"),
                    vec![
                        String::from("a car but not a car, a bigger car"),
                        String::from("lower part of the tree"),
                        String::from("a main branch in git repository"),
                        String::from("a vihickle"),
                    ],
                    0,
                    Some(3),
                    false,
                    100 
                ),
                test::AnswerModel::new(
                    String::from("Swims on water and donald is the name"),
                    vec![
                        String::from("dack"),
                        String::from("dak"),
                        String::from("duck"),
                        String::from("chicken"),
                    ],
                    2,
                    Some(0),
                    false,
                    23 
                ),
            ],
            123,
        ),
    ].iter()
        .find(|m| m.id == id)
        .cloned()
}




