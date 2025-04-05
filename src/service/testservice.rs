use crate::models::test;

// should be a struct
// how to solve value borrowing problem?

pub fn get_to_do<'a>() -> Vec<(&'a str, &'a str)> {
    vec![
        test::TestS {
            id: "xyz",
            title: "[2025-03-07] English idioms with twist",
        },
        test::TestS {
            id: "abc",
            title: "[2025-02-28] Verbs and stuff",
        },
        test::TestS {
            id: "cde",
            title: "[2025-02-27] Week exam #2",
        },
    ].iter()
	.map(|t| (t.id, t.title))
	.collect()
}

