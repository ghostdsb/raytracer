use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Default)]
pub struct HitRecord{
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

pub trait Hittable{
    fn hit(&self,  ray: &Ray, t_mix: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}

impl HitRecord{
    // pub fn new() -> Self {
    //     HitRecord{
    //         point: Vec3::new(0.0, 0.0, 0.0),
    //         normal: Vec3::new(0.0, 0.0, 0.0),
    //         t: 0.0,
    //         front_face: true
    //     }
    // }

    pub fn point(&self) -> Vec3{
        self.point
    }

    pub fn normal(&self) -> Vec3{
        self.normal
    }

    pub fn t(&self) -> f32{
        self.t
    }

    pub fn front_face(&self) -> bool{
        self.front_face
    }

    pub fn set_hit_record(&mut self, point: &Vec3, normal: &Vec3, t: f32, front_face: bool){
        *self = HitRecord{
            point: *point, normal: *normal, t, front_face
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(*outward_normal) < 0.0;
        
        if self.front_face{
            self.normal = *outward_normal;
        }else{
            self.normal = -*outward_normal;

        }
    }
}