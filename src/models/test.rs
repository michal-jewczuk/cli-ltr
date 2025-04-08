#[derive(Clone)]
pub struct TestModel<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub questions: Vec<QuestionModel<'a>>,
}

impl<'a> TestModel<'a> {
    pub fn new(
        id: &'a str,
        title: &'a str,
        questions: Vec<QuestionModel<'a>>
    ) -> Self {
        TestModel {
            id: id,
            title: title,
            questions: questions,
        }
    }
}

#[derive(Clone)]
pub struct QuestionModel<'a> {
    pub question: &'a str,
    pub answers: Vec<&'a str>,
    pub correct: u8,
}

impl<'a> QuestionModel<'a> {
    pub fn new(
        question: &'a str,
        answers: Vec<&'a str>,
        correct: u8,
    ) -> Self {
        QuestionModel {question, answers, correct}
    }
}

