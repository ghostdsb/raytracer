mod vec3;
mod ray;
mod constants;
mod sphere;
mod hittable;
mod hittable_list;
mod utils;
mod camera;

use std::env;
use std::process::Command;
use std::fs;
use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use hittable::{Hittable, HitRecord};
use hittable_list::HittableList;
use camera::Camera;


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

    let focal = Vec3::new(0.0, 0.0, constants::FOCAL_LENGTH);

    let camera = Camera::camera();

    let mut world = HittableList::new();
    let sphere = Sphere::new(-focal, 0.5);
    let sphere2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    world.add(Box::new(sphere2));
    world.add(Box::new(sphere));

    image.push_str(&format!("P3\n{} {}\n255\n",width, height));
    for y in (0..height).rev(){
        for x in 0..width{
            // let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            // for _ in 0..constants::SAMPLES_PER_PIXEL as usize {
            //     let u:f32 = (x as f32 + utils::random_number_ranged(0.0, 1.0)) / width as f32;
            //     let v:f32 = (y as f32 + utils::random_number_ranged(0.0, 1.0)) / height as f32;
                
            // }
            let u:f32 = (x as f32) / width as f32;
            let v:f32 = (y as f32) / height as f32;

            let ray: Ray = camera.get_ray(u, v);

            let pixel_color = ray_color(&ray, &world, constants::MAX_DEPTH);

            write_color(&mut image, &pixel_color);
        }
    }
    image
}

fn ray_color(ray: &Ray, world: &HittableList, depth: u32) -> Vec3 {
    let white = Vec3::new(1.0, 1.0, 1.0);
    let black = Vec3::new(0.0, 0.0, 0.0);
    let blue = Vec3::new(0.0, 0.0, 1.0);
    let mut rec: HitRecord = HitRecord::default(); 

    if depth <=0 {
        return black
    }
    
    if world.hit(ray, 0.01, constants::INFINITY, &mut rec){
        // return (rec.normal + white) * 0.5
        let target = rec.point() + Vec3::random_in_hemisphere(&rec.normal()) ;
        let dif_ray = Ray::new(rec.point(), target-rec.point());
        return ray_color(&dif_ray, world, depth - 1 ) * 0.5
        // return (rec.normal + white) * 0.5
    }else{
        let unit_direction = Vec3::unit_vector(ray.direction());
        let t = 0.5*(unit_direction.y() + 1.0);
        white*(1.0-t) + blue*t
    }

}

fn write_color(image: &mut String, color: &Vec3){
    let r_pixel_val = (255f32 * color.x()) as u32;
    let g_pixel_val = (255f32 * color.y()) as u32;
    let b_pixel_val = (255f32 * color.z()) as u32;

    image.push_str(&format!("{} {} {}\n", r_pixel_val, g_pixel_val, b_pixel_val));
}

fn show_image(filename: &String){
    Command::new("wslview").arg(filename).spawn().expect("ls command failed to start");
}