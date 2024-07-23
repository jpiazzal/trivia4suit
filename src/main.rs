mod domain;

use domain::question::Question;
use reqwest::Result;


#[tokio::main]
async fn main() {

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


    match get_questions().await {
        Ok(response) => { println!("{}", response)}
        _ => { println!("ERRRRRROOOOR")}
    }
}


async fn get_questions() -> Result<(String)> {
    let body = reqwest::get("https://opentdb.com/api.php?amount=10")
        .await?
        .text()
        .await?;

    Ok((body))
}