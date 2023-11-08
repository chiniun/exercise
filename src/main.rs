mod sort;
mod bubble;

use attempt::student;
use attempt::student::Student;

use sort::bubble::trait_bubble as tbubble;
fn main() {
    let mut s = vec![1,3,2,4,10,99,123,50];
    //i32类型
    bubble::bubble(&mut s);
    //泛型
    tbubble(&mut s);
   
    println!("{:?}",&s);

    let i:i32 = 10;
    student::get_age(&Student::from(i));
}
