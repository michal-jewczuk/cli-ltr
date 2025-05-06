use super::dbservice;
use crate::models::test;
use rusqlite::Connection;

pub fn init_conn_and_populate() -> Connection {
    // TODO move to file 
    // remove current dev db and recreate with mock data
    let conn = Connection::open_in_memory().unwrap();
    let _ = dbservice::create_schema(&conn);
    let _ = dbservice::populate_tests(&conn);
    let _ = dbservice::populate_questions(&conn);
    conn
}

pub fn init_conn() -> Connection {
    // TODO move to file once schema done
    Connection::open_in_memory().unwrap()
}

pub fn get_to_do(conn: &Connection) -> Vec<(String, String)> {
    match dbservice::get_fresh(conn) {
        Ok(rows) => rows,
        _ => vec![]
    }
}

pub fn get_q_by_id(conn: &Connection, id: String) -> Option<test::TestModel> {
    dbservice::get_q_by_id(conn, id)
}

pub fn get_by_id(id: String) -> Option<test::TestModel> {
    vec![
        test::TestModel {
            id: String::from("1"),
            title: String::from("[2025-03-07] English idioms with twist"),
            questions: vec![
                test::QuestionModel::new(
                    String::from("Lets imagine that you see your brother for the first time today and it is 1 pm. How do you greet him?"),
                    vec![
                        String::from("Good evening"),
                        String::from("Good morning"),
                        String::from("Hi, do we know each other?"),
                        String::from("Yo bro, shouldn't you be in Buenos Aires right now?"),
                    ],
                    3,
                ),
                test::QuestionModel::new(
                    String::from("This is the ... I am telling you this!"),
                    vec![
                        String::from("current time"),
                        String::from("previous time"),
                        String::from("last time"),
                        String::from("any timme"),
                    ],
                    2,
                ),
            ],
        },
        test::TestModel {
            id: String::from("2"),
            title: String::from("[2025-02-28] Verbs and stuff"),
            questions: vec![
                test::QuestionModel::new(
                    String::from("How would you tell somebody to go and do something?"),
                    vec![
                       String::from( "Could you just move, please?"),
                       String::from( "Just do it!"),
                       String::from( "Move your behind and get the shit done"),
                       String::from( "Just stay there and let things happen on their own"),
                    ],
                    0,
                ),
                test::QuestionModel::new(
                    String::from("... through pink glasses."),
                    vec![
                        String::from("Sees"),
                        String::from("Look"),
                        String::from("View"),
                        String::from("He see"),
                    ],
                    1,
                ),
                test::QuestionModel::new(
                    String::from("Why do policemen walk in paris?"),
                    vec![
                        String::from("To arrest you twice as fast for hate speech"),
                        String::from("Who could possibly know that"),
                        String::from("Two is better than one"),
                        String::from("One can read and the other can write"),
                    ],
                    3,
                ),
            ],
        },
        test::TestModel {
            id: String::from("3"),
            title: String::from("[2025-02-27] Week exam #2"),
            questions: vec![
                test::QuestionModel::new(
                    String::from("I wish you ... so dumm."),
                    vec![
                       String::from("is"),
                       String::from("weren't"),
                       String::from("was not"),
                       String::from("were"),
                    ],
                    1,
                ),
                test::QuestionModel::new(
                    String::from("What is the best way to describe the following situation: A woman in her late twentees or early thirties is walking down the street early mornig. Her hair are a mess, her make up is well, like her hair and she is holding her high heels in one hand while covering her face with the other."),
                    vec![
                        String::from("Early morning jogging"),
                        String::from("Stroll through the park"),
                        String::from("Walk of shame"),
                        String::from("My kind of a woman"),
                    ],
                    2,
                ),
                test::QuestionModel::new(
                    String::from("What could be your reaction to learning the correct answer to the previouis question? Pick the one that fits best."),
                    vec![
                        String::from("I should have known that!"),
                        String::from("Really? I had no idea!"),
                        String::from("So it was not my kind of a woman?"),
                        String::from("One of the above"),
                    ],
                    3,
                ),
                test::QuestionModel::new(
                    String::from("What is the proper way to say it: '3 + 3' 'is six' or 'are six'?"),
                    vec![
                        String::from("is six"),
                        String::from("are six"),
                        String::from("is nine"),
                        String::from("math? oh, come on!"),
                    ],
                    2,
                ),
            ],
        },
        test::TestModel {
            id: String::from("r_1"),
            title: String::from("[2025-01-03] Week exam #1"),
            questions: vec![
                test::QuestionModel::new(
                    String::from("Are you in the right class?"),
                    vec![
                        String::from("Yes"),
                        String::from("No"),
                        String::from("I have no idea"),
                        String::from("Maybe"),
                    ],
                    0,
                ),
            ],
        },
        test::TestModel {
            id: String::from("r_2"),
            title: String::from("[2025-01-03] Adjectives"),
            questions: vec![
                test::QuestionModel::new(
                    String::from("Less is more. True or false?"),
                    vec![
                        String::from("True"),
                        String::from("False"),
                        String::from("I have no idea"),
                        String::from("It depends"),
                    ],
                    3,
                ),
                test::QuestionModel::new(
                    String::from("The grass is always ... on the other side"),
                    vec![
                        String::from("bigger"),
                        String::from("greener"),
                        String::from("bushier"),
                        String::from("green"),
                    ],
                    1,
                ),
                test::QuestionModel::new(
                    String::from("If he is younger than me then I am ... than him"),
                    vec![
                        String::from("as young as"),
                        String::from("younger"),
                        String::from("old"),
                        String::from("older"),
                    ],
                    3,
                ),
            ],
        },
        test::TestModel {
            id: String::from("r_3"),
            title: String::from("[2025-01-08] Reading between the lines"),
            questions: vec![
                test::QuestionModel::new(
                    String::from("A deep dive into something means"),
                    vec![
                        String::from("to swim underwater really deep"),
                        String::from("to jump into water from a high place"),
                        String::from("to focus someones attention on one topic in order to understand it very well"),
                        String::from("all of the above"),
                    ],
                    2,
                ),
                test::QuestionModel::new(
                    String::from("Which number cannot be represented by the phrase: 'two four two zero'"),
                    vec![
                        String::from("2440"),
                        String::from("4420"),
                        String::from("4400"),
                        String::from("2420"),
                    ],
                    0,
                ),
            ],
        },
        test::TestModel {
            id: String::from("r_4"),
            title: String::from("[2025-01-12] Nouns or nuns"),
            questions: vec![
                test::QuestionModel::new(
                    String::from("What is a truck?"),
                    vec![
                        String::from("a car but not a car, a bigger car"),
                        String::from("lower part of the tree"),
                        String::from("a main branch in git repository"),
                        String::from("a vihickle"),
                    ],
                    0,
                ),
                test::QuestionModel::new(
                    String::from("Swims on water and donald is the name"),
                    vec![
                        String::from("dack"),
                        String::from("dak"),
                        String::from("duck"),
                        String::from("chicken"),
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

#[cfg(test)]
mod tests {
    use super::*;

    // TODO create a setup with in memomy conneciton once moved to file db

    #[test]
    fn test_query_by_id() {
        let questions = vec![
            test::QuestionModel::new(String::from(""), 
                vec![
                    String::from(""), 
                    String::from(""),
                    String::from(""),
                    String::from(""),
                ],
                2),
        ];
        let expected = Some(test::TestModel::new(
                String::from("1"), 
                String::from("[2025-03-07] English idioms with a twist"), 
                questions
        ));
        let conn = init_table();

        let result = get_q_by_id(&conn, String::from("1"));

        assert_eq!(result, expected);
    }
}

