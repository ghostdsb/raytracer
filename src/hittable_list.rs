use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hittable::{Hittable, HitRecord};

#[derive()]
pub struct HittableList{
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList{
    pub fn new()-> Self{
        HittableList{
            list: Vec::new()
        }
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>){
        self.list.push(hittable);
    }

}

impl Hittable for HittableList{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool{
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.list{
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec){
                hit_anything = true;
                closest_so_far = temp_rec.t;
                hit_record.set_hit_record(&temp_rec.point(), &temp_rec.normal(), temp_rec.t(), temp_rec.front_face());
            }
        }
        hit_anything
    }
}