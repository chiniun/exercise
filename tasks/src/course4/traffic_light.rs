
pub enum TrafficLight {
    Green(u32),
    Black,
}

pub enum Color {
    Green,
    Black
}


impl TrafficLight {
    pub fn new(c :Color) ->Self{
        match c {
            Color::Green => TrafficLight::Green(10),
            _ => TrafficLight::Black 
        }
    }
    pub fn get_time(self)->u32 {
        match self {
            TrafficLight::Green(t) => t,
            TrafficLight::Black =>0,
        }
        // if let TrafficLight::Green(t) = self  {
        //     t
        // }else{
        //     0
        // }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       let lt = TrafficLight::new(Color::Green);
       assert_eq!(lt.get_time(),10);
    }
}

