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

    pub fn is_correct(&self, answer: Option<usize>) -> bool {
        match answer {
	    None => return false,
	    Some(a) => {
		if a == self.correct.into() {
		    true
		} else {
		    false
		}
	    }
	}
    }
}

#[derive(Clone)]
pub struct ResultModel<'a> {
    pub answers: Vec<AnswerModel<'a>>,
    pub total_time: usize,
}

impl<'a> ResultModel<'a> {
    pub fn new(answers: Vec<AnswerModel<'a>>, total_time: usize) -> Self {
        ResultModel { answers, total_time }
    }
}

#[derive(Clone, Debug)]
pub struct AnswerModel<'a> {
    pub question: &'a str,
    pub correct: u8,
    pub given: Option<usize>,
    pub is_correct: bool,
    pub time: usize,
}

impl<'a> AnswerModel<'a> {
    pub fn new(
	question: &'a str,
	correct: u8,
	given: Option<usize>,
	is_correct: bool,
	time: usize,
    ) -> Self {
	AnswerModel { question, correct, given, is_correct, time}
    }
}



