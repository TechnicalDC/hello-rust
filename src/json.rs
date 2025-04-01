use serde::{Deserialize, Serialize};
use std::io::stdin;
use serde_json::Result;

#[derive(Serialize,Deserialize, Debug)]
struct Employee {
    name: String,
    age: i32,
    location: String,
    department: String
}

#[warn(dead_code)]
pub fn play_with_json() -> Result<()> {
    let listen = stdin();
    let mut json_str: String = String::new();
    let _ = listen.read_line(&mut json_str);
    let emp: Employee = serde_json::from_str::<Employee>(&json_str.as_str())?;
    //emp = Employee {
    //    name: "dilip".to_string(),
    //    age: 28,
    //    location: "mumbai".to_string(),
    //    department: "qad".to_string()
    //};

    println!("{:?}", emp);
    println!("{:?}", emp.age);

    let outout = serde_json::to_string(&emp);
    println!("{:?}", outout);

    //let _ = typed_example();
    Ok(())
}
