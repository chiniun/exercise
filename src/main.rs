mod sort;
mod bubble;

fn main() {
    let mut s = vec![1,3,2,4,10,99,123,50];
    //i32类型
    bubble::bubble(&mut s);
    //泛型
    sort::bubble::trait_bubble(&mut s);
   
    println!("{:?}",&s);
}
