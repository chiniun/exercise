mod sort;
mod bubble;



use tasks::course4::traffic_light::{Color,TrafficLight};

use crate::sort::bubble::trait_bubble as tbubble;

fn main() {
    //03作业冒泡
    let mut s = vec![1,3,2,4,10,99,123,50];
    bubble::bubble(&mut s);//i32类型
    tbubble(&mut s); //泛型
   

    //04交通灯
    let tl = TrafficLight::new(Color::Green);
    println!("{:?}",tl.get_time());
    

}
