use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use std::io::stdin;
use serde_json::Result;

pub mod modules;
pub mod extras;
pub mod generics;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[derive(Serialize, Debug)]
struct Employee {
    name: String,
    age: i32,
    location: String,
    department: String
}

fn main() {
    //extras::display_user();
    let mut emp: Employee;
    let listen = stdin();
    let mut json_str: String = String::new();
    let _ = listen.read_line(&mut json_str);

    emp = Employee {
        name: "dilip".to_string(),
        age: 28,
        location: "mumbai".to_string(),
        department: "qad".to_string()
    };

    //emp = serde_json::from_str(&json_str);
    println!("{:?}", emp.age);

    //let outout = serde_json::to_string(&emp);
    //println!("{:?}", outout);

    let _ = typed_example();
}

fn typed_example() -> Result<()> {
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
