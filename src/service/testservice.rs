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

pub fn get_finished(conn: &Connection) -> Vec<(String, String)> {
    match dbservice::get_by_status(conn, "FINISHED") {
        Ok(rows) => rows,
        _ => vec![]
    }
}

pub fn get_test_by_id(conn: &Connection, id: String) -> Option<test::TestModel> {
    dbservice::get_test_by_id(conn, id)
}

pub fn get_results_by_id(id: String) -> Option<test::ResultModel> {
    vec![
        test::ResultModel::new(
            String::from("4"), String::from("[2025-01-03] Week exam #1"),
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
            String::from("5"), String::from("[2025-01-03] Adjectives"),
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
            String::from("6"), String::from("[2025-01-08] Reading between the lines"),
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
            String::from("7"), String::from("[2025-01-12] Nouns or nuns"),
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

