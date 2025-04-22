use crate::models::test;

// should be a struct
// how to solve value borrowing problem?

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
            id: "xyz",
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
                    0,
                    Some(3),
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




