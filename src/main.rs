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
use std::fs::File;
use vec3::Vec3;

// Dimensions of the output image, in pixels.
const WIDTH: usize = 640;
const HEIGHT: usize = 480;

// Distance between the image plane and the vantage point.
const VANTAGE_DISTANCE: f64 = 50.0;

fn main() {
    let mut image: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let scene_objects = vec![
        Sphere::new(Vec3::new(-100.0, 0.0, 50.0), 25),
        Sphere::new(Vec3::new(-50.0, 0.0, 50.0), 25),
        Sphere::new(Vec3::new(0.0, 0.0, 50.0), 25),
        Sphere::new(Vec3::new(50.0, 0.0, 50.0), 25),
        Sphere::new(Vec3::new(100.0, 0.0, 50.0), 25),
    ];

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            // Cast a ray from the vantage point to the center of the current pixel.
            let primary_ray = Ray::new(
                origin.clone(),
                Vec3::new(
                    -((WIDTH / 2) as f64) + j as f64,
                    ((HEIGHT / 2) as f64) - i as f64,
                    VANTAGE_DISTANCE
                ),
            );
            
            // Compute where the ray intersects objects in the scene.
            let mut hits = Vec::new();
            for object in scene_objects.iter() {
                match object.intersection(&primary_ray) {
                    Some(hit) => hits.push(hit),
                    None => (),
                }
            }
            
            // Find the intersection closest to the camera and use that to color the pixel.
            if hits.len() > 0 {
                let mut closest = &hits[0];
                for hit in hits[1..].iter() {
                    if origin.distance(hit) < origin.distance(&closest) {
                        closest = hit;
                    }
                }
                
                // TODO: Actually color based off of the object material.
                image.put_pixel(j as u32, i as u32, Rgb([255, 255, 255]));
            }
        }
    }

    let mut file = File::create("render.jpg").unwrap();
    JpegEncoder::new(&mut file)
        .encode_image(&image)
        .unwrap();
}
