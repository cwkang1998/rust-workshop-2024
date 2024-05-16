use serde::{Deserialize, Serialize};

fn part_one() {
    let x = 5;
    println!("Hello world {:?}", x);

    // You can treat if else statement as an expression
    let flag = if x > 6 {
        true
    } else {
        false
    };

    println!("Flag is {:?}", flag);

    
    // Now let's try making a simple guessing game

}


fn part_two() {
    // Add ownership exercises.
}


// Calling https://jsonplaceholder.typicode.com/ for the mock data

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct UserPost {
    user_id: i32,
    id: i32,
    title: String,
    body: String,
}

fn part_three() {
    let res = reqwest::blocking::get("https://jsonplaceholder.typicode.com/posts").unwrap();
    // let body_text = res.text().unwrap();
    // println!("Body text: {}", body_text);

    let body_json: Vec<UserPost> = res.json().unwrap();
    println!("Body json: {:?}", body_json);
}


fn main() {
    // Comment out whichever is being ran
    // part_one();
    // part_two();
    part_three();
}
