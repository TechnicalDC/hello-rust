use std::io::{stdin, stdout, BufWriter, Write};
use ferris_says::say;

use crate::modules::types::GenderCategory;
use crate::modules::types::User;
use crate::extras;

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

pub fn said(text:&str) {
    let stdout = stdout();
    let message = String::from(text);
    let width = message.chars().count();

    let writer = BufWriter::new(stdout.lock());
    let _ = say(&message, width, writer);
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
