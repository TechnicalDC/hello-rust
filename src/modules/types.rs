use serde::Deserialize;
use serde::Serialize;

#[derive(Debug)]
pub enum GenderCategory {
   Male,
   _Female
}

pub struct Config {
    pub json: bool,
    pub extra: bool,
    pub generics: bool
}

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub phones: Vec<String>,
}

pub struct User {
    pub name:String,
    pub age:i32,
    pub gender: GenderCategory,
    pub address:String,
    pub email:String
}

impl User {
   pub fn update_age (&mut self) {
     self.age = 0;
   }
}
