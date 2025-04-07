use crate::models::test;

// should be a struct
// how to solve value borrowing problem?

pub fn get_to_do<'a>() -> Vec<(&'a str, &'a str)> {
    vec![
        test::TestModel {
            id: "xyz",
            title: "[2025-03-07] English idioms with twist",
        },
        test::TestModel {
            id: "abc",
            title: "[2025-02-28] Verbs and stuff",
        },
        test::TestModel {
            id: "cde",
            title: "[2025-02-27] Week exam #2",
        },
    ].iter()
	.map(|t| (t.id, t.title))
	.collect()
}

pub fn get_by_id<'a>(id: String) -> Option<test::TestModel<'a>> {
    vec![
        test::TestModel {
            id: "xyz",
            title: "[2025-03-07] English idioms with twist",
        },
        test::TestModel {
            id: "abc",
            title: "[2025-02-28] Verbs and stuff",
        },
        test::TestModel {
            id: "cde",
            title: "[2025-02-27] Week exam #2",
        },
    ].iter()
        .find(|t| t.id == id)
        .cloned()
}

