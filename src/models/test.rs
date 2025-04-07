#[derive(Clone)]
pub struct TestModel<'a> {
    pub id: &'a str,
    pub title: &'a str,
}

impl<'a> TestModel<'a> {
    pub fn new(
        id: &'a str,
        title: &'a str,
    ) -> Self {
        TestModel {
            id: id,
            title: title,
        }
    }
}

