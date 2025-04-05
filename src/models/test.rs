#[derive(Clone)]
pub struct TestS<'a> {
    pub id: &'a str,
    pub title: &'a str,
}

impl<'a> TestS<'a> {
    pub fn new(
        id: &'a str,
        title: &'a str,
    ) -> Self {
        TestS {
            id: id,
            title: title,
        }
    }
}

