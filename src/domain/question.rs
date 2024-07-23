pub struct Question {
    pub title: String,
    pub category: String,
    pub propositions: Vec<String>,
    pub answer: String,
}

impl Question {
    pub fn validate(&self, proposition: String) -> bool {
        return self.answer == proposition;
    }
}
