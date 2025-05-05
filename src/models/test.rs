#[derive(Clone, Debug, PartialEq)]
pub struct TestModel {
    pub id: String, 
    pub title: String, 
    pub questions: Vec<QuestionModel>,
}

impl TestModel {
    pub fn new(
        id: String,
        title: String,
        questions: Vec<QuestionModel>
    ) -> Self {
        TestModel { id, title, questions, }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct QuestionModel {
    pub question: String,
    pub answers: Vec<String>,
    pub correct: u8,
}

impl QuestionModel {
    pub fn new(
        question: String,
        answers: Vec<String>,
        correct: u8,
    ) -> Self {
        QuestionModel {question, answers, correct}
    }

    pub fn is_correct(&self, answer: Option<usize>) -> bool {
        match answer {
	    None => return false,
	    Some(a) => {
		if a == usize::from(self.correct) {
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



