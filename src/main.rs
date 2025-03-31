use serde::Serialize;
use std::io::stdin;
//use serde_json;

pub mod modules;
pub mod extras;
pub mod generics;

#[derive(Serialize)]
struct Employee {
    name: String,
    //age: Option<i32>,
    age: i32,
    location: String,
    department: String
}

fn main() {
    //extras::display_user();
    let mut emp: Employee;
    let listen = stdin();
    let mut json_str: String = String::new();
    //let _ = listen.read_line(&mut json_str);

    emp = Employee {
        name: "dilip".to_string(),
        //age: None,
        //age: Some(28),
        age: 28,
        location: "mumbai".to_string(),
        department: "qad".to_string()
    };

    println!("{:?}", emp.age);

    //let json_string = serde_json::to_string(&emp);

    //println!("{}", json_string);
}
