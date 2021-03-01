mod vec3;
mod ray;
mod constants;
mod sphere;
mod hittable;

use std::env;
use std::process::Command;
use std::fs;
use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use hittable::{Hittable, HitRecord};

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

    let sphere = Sphere::new(-focal, 0.5);
    let sphere2 = Sphere::new(Vec3::new(0.0, -100.0, -1.0), 100.0);
    
    image.push_str(&format!("P3\n{} {}\n255\n",width, height));
    for y in (0..height).rev(){
        for x in 0..width{
            let u:f32 = x as f32 / width as f32;
            let v:f32 = y as f32 / height as f32;

            let ray: Ray = Ray::new(origin, llc + horizontal*u + vertical*v - origin);
            // let color = ray_color(&ray);
            let color = ray_color_n(&ray, &sphere);

            let r_pixel_val = (255f32 * color[0]) as u32;
            let g_pixel_val = (255f32 * color[1]) as u32;
            let b_pixel_val = (255f32 * color[2]) as u32;

            image.push_str(&format!("{} {} {}\n", r_pixel_val, g_pixel_val, b_pixel_val));
        }
    }

    image
}

fn ray_color_n(ray: &Ray, sphere: &Sphere) -> Vec3 {
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.0, 0.0, 1.0);
    let red = Vec3::new(1.0, 0.0, 0.0);
    let center = Vec3::new(0.0, 0.0, -1.0);
    // let t = hit_sphere(&center, 0.5, ray);
    let mut rec: HitRecord = HitRecord{
        point: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        t: 0.0,
    }; 
    
    if sphere.hit(ray, 0.0, f32::INFINITY, &mut rec){
        return (rec.normal + white) * 0.5
    }else{
        let unit_direction = Vec3::unit_vector(ray.direction());
        let t = 0.5*(unit_direction.y() + 1.0);
        white*(1.0-t) + blue*t
    }

}

fn ray_color(ray: &Ray) -> Vec3 {
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.0, 0.0, 1.0);
    let red = Vec3::new(1.0, 0.0, 0.0);
    let center = Vec3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(&center, 0.5, ray);
    if t <= 0.0 {
        let unit_direction = Vec3::unit_vector(ray.direction());
        let t = 0.5*(unit_direction.y() + 1.0);
        white*(1.0-t) + blue*t
    }else{
        let ray_point_at_t = ray.point_at_parameter(t);
        let unit_vec_at_t = Vec3::unit_vector(ray_point_at_t - center);
        Vec3::new(unit_vec_at_t.x()+1.0, unit_vec_at_t.y()+1.0, unit_vec_at_t.z()+1.0) * 0.5
    }
}

fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> f32{
    // (P−C)⋅(P−C)=r_2
    // (P(t)−C)⋅(P(t)−C)=r_2
    // (A+tB−C)⋅(A+tB−C)=r_2
    // t_2B⋅B+2tB⋅(A−C)+(A−C)⋅(A−C)−r2=0
    // a = B⋅B; b = 2B⋅(A−C); c=(A−C)⋅(A−C)−r2
    // C -> Center
    // Ray -> A: Origin, B: Direction
    // t = -B +- sqrt(B_2 - 4 A.C) / 2A

    let oc = ray.origin()-*center;
    // let a = Vec3::dot(&ray.direction(), ray.direction());
    let a = ray.direction().length_squared();
    // let b = Vec3::dot(&ray.direction(), ray.origin()-*center) * 2.0;
    let half_b = Vec3::dot(&ray.direction(), oc);
    let c = oc.length_squared() - radius*radius;
    let d = half_b*half_b - a*c;
    if d < 0.0 {
        -1.0
    }else {
        (-half_b - d.sqrt()) / a
    }
}

fn show_image(filename: &String){
    Command::new("wslview").arg(filename).spawn().expect("ls command failed to start");
}