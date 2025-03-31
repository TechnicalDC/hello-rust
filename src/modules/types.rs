#[derive(Debug)]
pub enum GenderCategory {
   Male,
   _Female
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
