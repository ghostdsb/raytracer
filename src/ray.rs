use crate::vec3::Vec3;

pub struct Ray{
    A: Vec3,
    B: Vec3
}

impl Ray{
    pub fn new(a: Vec3, b: Vec3) -> Ray{
        Ray{ A: a, B: b }
    }

    pub fn origin(&self) -> Vec3 {self.A}

    pub fn direction(&self) -> Vec3 {self.B}

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin() + self.direction()*t
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_of_ray() {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(3.0, 4.0, 0.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.origin(), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn direction_of_ray() {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(3.0, 4.0, 0.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.direction(), Vec3::new(3.0, 4.0, 0.0));
    }

    #[test]
    fn point_at_parameter_of_ray() {
        let origin = Vec3::new(1.0, 1.0, 1.0);
        let direction = Vec3::new(3.0, 4.0, 0.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.point_at_parameter(2.0), Vec3::new(7.0, 9.0, 1.0));
    }
}