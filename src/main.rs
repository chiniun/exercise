use tasks::course3::bubble::{*};
use tasks::course4::traffic_light::{Color,TrafficLight};
use tasks::course4::add_u32::add_u32;


fn main() {
    //03课冒泡
    let mut s = vec![1,3,2,4,10,99,123,50];
    bubble(&mut s);//i32类型
    trait_bubble(&mut s); //泛型
   
    //04课第8题交通灯
    let tl = TrafficLight::new(Color::Green);
    assert_eq!(tl.get_time(),10);

    //04课第9题 addU32
    let x = [1,2];
    assert_eq!(add_u32(&x),Some(3));




}
