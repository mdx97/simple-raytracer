use super::ray::Ray;
use super::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: u32,
    pub color: (u8, u8, u8),
}

impl Sphere {
    pub fn new(center: Vec3, radius: u32, color: (u8, u8, u8)) -> Self {
        Self { center, radius, color }
    }
    pub fn intersection(&self, ray: &Ray) -> Option<Vec3> {
        let conn = ray.origin.minus(&self.center);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&conn);
        let c = conn.dot(&conn) - (self.radius.pow(2) as f64);

        // If the discriminant is less than 0, there are no real roots to the quadratic equation.
        // In this context, that means there is no ray intersection.
        let discriminant = b.powi(2) - (4.0 * a * c);
        if discriminant < 0.0 {
            return None
        }

        // Solve the quadratic equation and return the point in space that the ray intersects with the sphere.
        // NOTE: This requires ray.direction to be a unit vector in order for scaling to apply properly.
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        return Some(ray.origin.plus(&ray.direction.scale(t)));
    }
}
