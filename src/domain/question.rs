struct Question {
    title: String,
    category: String,
    propositions: Vec<String>,
    answer: String,
}

impl Question {
    fn validate(&self, proposition: String) -> bool {
        return self.answer == proposition;
    }
}
