mod domain;

use domain::question::Question;

fn main() {
    let question = Question {
        title: String::from("My super question , is Joël smart ?"),
        answer: String::from("NO"),
        category: String::from("person"),
        propositions: vec![String::from("YES")],
    };

    let result = question.validate(String::from("NO"));
    if result == true {
        println!("Answer correct")
    } else {
        println!("Answer incorrect")
    }
}
