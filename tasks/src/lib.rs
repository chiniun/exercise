pub mod course4;
use course4::traffic_light::{*};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       let lt = TrafficLight::new(Color::Green);
       assert_eq!(lt.get_time(),15);
    }
}