mod domain;

use domain::question::Question;

fn main() {
  let question = Question {
    title: String::from("My super question , is joel smart ?"),
    answer: String::from("NO"),
    category: String::from("person"),
    propositions: vec![String::from("YES")],
  };

}


