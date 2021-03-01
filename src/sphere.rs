use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hittable::{Hittable, HitRecord};

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere{
        Sphere {
            center, radius
        }
    }
}

impl Hittable for Sphere{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let oc = ray.origin()-self.center;
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(&ray.direction(), oc);
        let c = oc.length_squared() - self.radius*self.radius;
        let d = half_b*half_b - a*c;
        
        if d < 0.0{
            return false 
        }else{
            let d_sqrt = d.sqrt();
            let mut root = (-half_b - d_sqrt) / a;
            if root < t_min || root > t_max{
                root = (-half_b + d_sqrt) / a;
                if root < t_min || root > t_max{
                    return false; 
                }
            }
            hit_record.t = root;
            hit_record.point = ray.point_at_parameter(root);
            let outward_normal = (hit_record.point - self.center) / self.radius;
            hit_record.set_face_normal(ray, &outward_normal);
        }
        true
    }
}
