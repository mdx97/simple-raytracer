// Dimensions of the output image, in pixels.
const WIDTH: usize = 640;
const HEIGHT: usize = 480;

// Distance between the image plane and the vantage point.
const VANTAGE_DISTANCE: u32 = 100;

struct Sphere {
    center: Vec3,
    radius: u32,
}

impl Sphere {
    fn new(center: Vec3, radius: u32) -> Self {
        Self { center, radius }
    }

    fn intersection(&self, ray: &Ray) -> Option<Vec3> {
        let conn = &ray.origin.minus(&self.center);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&conn);
        let c = conn.dot(&conn) - (self.radius.pow(2) as f64);

        // If the discriminant is less than 0, there are no real roots to the quadratic equation.
        // In this context, that means there is no ray intersection.
        let discriminant = (b * b) - (4.0 * a * c);
        if discriminant < 0.0 {
            return None
        }

        // Solve the quadratic equation and return the point in space that the ray intersects with the sphere.
        // This requires ray.direction to be a unit vector in order for scaling to apply properly.
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        return Some(ray.origin.plus(&ray.direction.scale(t)));
    }
}

struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn dot(&self, other: &Vec3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    fn plus(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    fn minus(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    fn scale(&self, factor: f64) -> Vec3 {
        Vec3::new(self.x * factor, self.y * factor, self.z * factor)
    }
}

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
}

fn main() {
    let image: [[(u8, u8, u8); WIDTH]; HEIGHT] = [[(0, 0, 0); WIDTH]; HEIGHT];
    let scene_objects = vec![
        Sphere::new(Vec3::new(0.0, 0.0, 0.0), 30),
        Sphere::new(Vec3::new(50.0, 50.0, 50.0), 30)
    ];

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            // TODO: Compute ray direction to the given pixel.
            let primary_ray = Ray::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
            );

            for object in scene_objects.iter() {

            }
        }
    }
}
