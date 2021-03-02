use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::constants;

pub struct Camera{
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    llc: Vec3,
}

impl Camera{
    pub fn camera()-> Camera{
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(constants::VP_WIDTH, 0.0, 0.0);
        let vertical = Vec3::new(0.0, constants::VP_HEIGHT, 0.0);
        let focal = Vec3::new(0.0, 0.0, constants::FOCAL_LENGTH);
        let llc = origin - horizontal/2.0 - vertical/2.0 - focal;
        Camera{origin,horizontal,vertical,llc}
    }

    pub fn get_ray(&self, u: f32, v: f32)-> Ray{
        Ray::new(self.origin, self.llc + self.horizontal*u + self.vertical*v - self.origin)
    }
}