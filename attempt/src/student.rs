
pub struct Student {
    name: String,
    age: i32,
 }
 
impl From<i32> for Student {
    fn from(value: i32) -> Self{
        Student { name: "xx".to_string(), age: value }
    }
}
 
pub fn get_age(s : &Student){
     println!("{:?}",s.age);
}