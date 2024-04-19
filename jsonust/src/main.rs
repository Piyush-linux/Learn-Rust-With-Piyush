use serde::{Serialize,Deserialize};
use serde_json::to_string;

#[derive(Debug,Serialize,Deserialize)]
enum Gender {
    M,
    F
}

#[derive(Serialize,Deserialize)]
struct Dog{
    name: String,
    age: i8,
    gender: Gender
}

fn main() {
    let dog1: Dog = Dog{
        name: String::from("Koko"),
        age:5,
        gender: Gender::F
    };
    let dog_str = to_string(&dog1);
    if dog_str.is_ok() {
        println!("{}",dog_str.ok().unwrap());
    }else {
        println!("{:#?}",dog_str.err());
    }

}
