
pub struct Circle{
    pub center: (i32, i32),
    pub radius: u32,
}  

impl Circle {
    fn new(center: (i32, i32), radius: u32) -> Circle {
        Circle {
            center: center,
            radius: radius,
        }
    }
     
    fn area(&self) -> f64 {
        // converting self.radius, a u32, to an f64.
        let f_radius = self.radius as f64;
         
        f_radius * f_radius * 3.14159
    }
     
    fn move_to(&mut self, new_center: (i32, i32)) {
        self.center = new_center;
    }
}