pub trait Graph {
    fn get_area(&self) -> f64;
}

pub struct Square {
    pub side_len: f64,
}

impl Graph for Square {
    fn get_area(&self) -> f64 {
        self.side_len * self.side_len
    }
}

pub struct Circle {
    pub radius: f64,
}
impl Graph for Circle {
    fn get_area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

pub fn calculate_area(s: &impl Graph) -> f64 {
    s.get_area()
}
