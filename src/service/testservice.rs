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

pub fn get_results_list() -> Vec<(String, String)> {
    vec![
        (String::from("r_1"), String::from("[2025-01-03] Week exam #1")),
        (String::from("r_2"), String::from("[2025-01-03] Adjectives")),
        (String::from("r_3"), String::from("[2025-01-08] Reading between the lines")),
        (String::from("r_4"), String::from("[2025-01-12] Nouns or nuns")),
    ]
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

