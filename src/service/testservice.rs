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
    let _ = dbservice::populate_answers(&conn);
    conn
}

pub fn init_conn() -> Connection {
    // TODO move to file once schema done
    Connection::open_in_memory().unwrap()
}

pub fn get_to_do(conn: &Connection) -> Vec<(String, String)> {
    match dbservice::get_by_status(conn, "NOT_STARTED") {
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

pub fn get_result_by_id(conn: &Connection, id: String) -> Option<test::ResultModel> {
    dbservice::get_result_by_id(conn, id)
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

