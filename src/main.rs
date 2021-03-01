mod vec3;
mod ray;
mod constants;
mod sphere;
mod hittable;
mod hittable_list;

use std::env;
use std::process::Command;
use std::fs;
use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use hittable::{Hittable, HitRecord};
use hittable_list::HittableList;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    match fs::write(filename, make_image()){
        Ok(_) => "written to file",
        Err(_) => "error writing to file"
    };
    show_image(filename);
}

fn make_image()->String{

    let mut image = String::new();
    let width:u32 = constants::WIDTH as u32;
    let height:u32 = constants::HEIGHT as u32;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(constants::VP_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, constants::VP_HEIGHT, 0.0);
    let focal = Vec3::new(0.0, 0.0, constants::FOCAL_LENGTH);
    let llc = origin - horizontal/2.0 - vertical/2.0 - focal;

    let mut world = HittableList::new();
    let sphere = Sphere::new(-focal, 0.5);
    let sphere2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    world.add(Box::new(sphere2));
    world.add(Box::new(sphere));

    image.push_str(&format!("P3\n{} {}\n255\n",width, height));
    for y in (0..height).rev(){
        for x in 0..width{
            let u:f32 = x as f32 / width as f32;
            let v:f32 = y as f32 / height as f32;

            let ray: Ray = Ray::new(origin, llc + horizontal*u + vertical*v - origin);
            let color = ray_color(&ray, &world);

            let r_pixel_val = (255f32 * color[0]) as u32;
            let g_pixel_val = (255f32 * color[1]) as u32;
            let b_pixel_val = (255f32 * color[2]) as u32;

            image.push_str(&format!("{} {} {}\n", r_pixel_val, g_pixel_val, b_pixel_val));
        }
    }
    image
}

fn ray_color(ray: &Ray, world: &HittableList) -> Vec3 {
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.0, 0.0, 1.0);
    let mut rec: HitRecord = HitRecord::default(); 
    
    if world.hit(ray, 0.0, f32::INFINITY, &mut rec){
        return (rec.normal + white) * 0.5
    }else{
        let unit_direction = Vec3::unit_vector(ray.direction());
        let t = 0.5*(unit_direction.y() + 1.0);
        white*(1.0-t) + blue*t
    }

}

fn show_image(filename: &String){
    Command::new("wslview").arg(filename).spawn().expect("ls command failed to start");
}