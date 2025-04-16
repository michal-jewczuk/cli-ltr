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

#[derive(Clone, Debug)]
pub struct ResultModel {
    pub id: String,
    pub title: String,
    pub answers: Vec<AnswerModel>,
    pub total_time: u64,
}

impl ResultModel {
    pub fn new(
        id: String,
        title: String,
        answers: Vec<AnswerModel>, 
        total_time: u64
        ) -> Self {
        ResultModel { 
            id,
            title,
            answers, 
            total_time 
        }
    }
}

#[derive(Clone, Debug)]
pub struct AnswerModel {
    pub question: String,
    pub answers: Vec<String>,
    pub correct: u8,
    pub given: Option<usize>,
    pub is_correct: bool,
    pub time: u64,
}

impl AnswerModel {
    pub fn new(
	question: String,
        answers: Vec<String>,
	correct: u8,
	given: Option<usize>,
	is_correct: bool,
	time: u64,
    ) -> Self {
	AnswerModel { question, answers, correct, given, is_correct, time}
    }
}



