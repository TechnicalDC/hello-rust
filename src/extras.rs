use std::io::{stdin, stdout, Write};
use serde_json::Result;

use crate::modules::types::GenderCategory;
use crate::modules::types::User;
use crate::modules::types::Person;
use crate::extras;

pub fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}

pub fn display_user() {
    let mut user: User;
    user = extras::get_user();
    println!("The name is {}, and his age is {} and lives at {}", user.name, user.age, user.address);
    user.update_age();
    println!("The updated age is {}", user.age );
    println!("Here are the rest of the details where email is {} and gender is {:?}", user.email, user.gender);
}


pub fn chat() {
    let mut say = stdout();
    let listen = stdin();
    let mut message: String = String::new();
    for num in 1..10 {
        println!("{}",num);
        let _ = listen.read_line(&mut message);
        let _ = say.write(message.as_bytes()).unwrap();
    }
}

pub fn get_user() -> User {
    return User {
        name: String::from("dilip chauhan"),
        age:    28,
        gender: GenderCategory::Male,
        address: String::from("virar, india"),
        email: String::from("xyz@gmail.com")
    };
}

pub fn looper() {
    for inx in 1..10 {
        println!("{}", inx)
    }
}

pub fn is_greater(input:i32) -> bool {
    if input > 10 {
        return true;
    } else {
        return false;
    }
}

pub fn get_state_name(state_code:&str){
    let state_code = state_code;
    let state = match state_code {
        "MH" => {
            println!("Found match for MH"); "Maharashtra"
        },
        "KL" => "Kerala",
        "KA" => "Karnadaka",
        "GA" => "Goa",
        _ => "Unknown"
    };
    println!("State name is {}",state);
}
