use kutil::std::string::*;

pub fn main() {
    let options = Vec::<&str>::default();
    println!("joined: {}", options.join_conjunction("or"));

    let options = &["one"];
    println!("joined: {}", options.join_conjunction("or"));

    let options = &["one", "two"];
    println!("joined: {}", options.join_conjunction("or"));

    let options = vec!["one", "two", "three"];
    println!("joined: {}", options.join_conjunction("or"));

    let options = vec!["one".to_string(), "two".to_string(), "three".to_string(), "four".to_string()];
    println!("joined: {}", options.join_conjunction("and"));
}
