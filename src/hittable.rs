use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord{
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
}



pub trait Hittable{
    fn hit(self: &Self,  ray: &Ray, t_mix: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}