mod sphere;
mod ray;
mod vec3;
use image::{
    codecs::jpeg::JpegEncoder,
    ImageBuffer,
    Rgb,
    RgbImage,
};
use ray::Ray;
use sphere::Sphere;
use std::cmp::max;
use std::fs::File;
use vec3::Vec3;

// Dimensions of the output image, in pixels.
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

// Distance between the image plane and the vantage point.
const VANTAGE_DISTANCE: f64 = 200.0;

// Determines the quality of anti-aliasing.
// TODO: Currently the scaling needs fixing.
const AA_SCALE: usize = 1;

struct Hit<'a> {
    pos: Vec3,
    object: &'a Sphere,
}

fn main() {
    // Calculate the size of the ImageBuffer with respect to our anti-aliasing factor.
    let buffer_width = (WIDTH * AA_SCALE) as u32;
    let buffer_height = (HEIGHT * AA_SCALE) as u32;
    let mut image: RgbImage = ImageBuffer::new(buffer_width, buffer_height);

    let origin = Vec3::new(0.0, 0.0, VANTAGE_DISTANCE);
    let light = Vec3::new(0.0, 200.0, 50.0);
    let scene_objects = vec![
        Sphere::new(Vec3::new(-200.0, 0.0, -25.0), 50),
        Sphere::new(Vec3::new(-100.0, 0.0, -25.0), 50),
        Sphere::new(Vec3::new(0.0, 0.0, -25.0), 50),
        Sphere::new(Vec3::new(100.0, 0.0, -25.0), 50),
        Sphere::new(Vec3::new(200.0, 0.0, -25.0), 50),
    ];

    for i in 0..buffer_height {
        for j in 0..buffer_width {
            // Cast a ray from the vantage point to the center of the current pixel.
            let primary_ray = Ray::new(
                origin.clone(),
                Vec3::new(
                    -((buffer_width / 2) as f64) + j as f64,
                    ((buffer_height / 2) as f64) - i as f64,
                    VANTAGE_DISTANCE
                ).unit(),
            );
            
            // Compute where the ray intersects objects in the scene.
            let mut hits = Vec::new();
            for object in scene_objects.iter() {
                match object.intersection(&primary_ray) {
                    Some(pos) => hits.push(Hit { pos, object: &object }),
                    None => (),
                }
            }
            
            // Find the intersection closest to the camera and use that to color the pixel.
            if hits.len() > 0 {
                let mut closest = &hits[0];
                for hit in hits[1..].iter() {
                    if origin.distance(&hit.pos) < origin.distance(&closest.pos) {
                        closest = hit;
                    }
                }

                let surface_normal = closest.pos.minus(&closest.object.center).unit();
                let lighting_coef = closest.pos.minus(&light).unit().dot(&surface_normal);

                // TODO: Actually color based off of the object material.
                let saturation = max((255.0 * lighting_coef) as u8, 10);
                image.put_pixel(j as u32, i as u32, Rgb([saturation, saturation, saturation]));
            }
        }
    }

    // Use supersampling to provide Anti-Aliasing.
    let mut final_image: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let mut color_totals = [0.0, 0.0, 0.0];
            for k in 0..AA_SCALE {
                for l in 0..AA_SCALE {
                    let pixel = image.get_pixel(((j * AA_SCALE) + l) as u32, ((i * AA_SCALE) + k) as u32);
                    color_totals[0] += pixel[0] as f64;
                    color_totals[1] += pixel[1] as f64;
                    color_totals[2] += pixel[2] as f64;
                }
            }
            final_image.put_pixel(j as u32, i as u32, Rgb([
                (color_totals[0] / AA_SCALE.pow(2) as f64) as u8,
                (color_totals[1] / AA_SCALE.pow(2) as f64) as u8,
                (color_totals[2] / AA_SCALE.pow(2) as f64) as u8,
            ]));
        }
    }

    let mut file = File::create("render.jpg").unwrap();
    JpegEncoder::new(&mut file)
        .encode_image(&final_image)
        .unwrap();
}
