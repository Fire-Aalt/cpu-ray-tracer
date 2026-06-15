use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::math::{AABB, Interval, ONB, Ray, Vec3};
use crate::utils::random_f32;
use std::f32::consts::PI;
use std::sync::Arc;

pub struct Sphere {
    center: Ray,
    radius: f32,
    material: Arc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, mat: Arc<dyn Material>) -> Arc<Self> {
        assert!(center.is_finite(), "sphere center must be finite");
        assert!(
            radius.is_finite() && radius > 0.0,
            "sphere radius must be finite and positive"
        );
        let radius = radius.max(0.001);
        let r_vec = Vec3::new(radius, radius, radius);

        Arc::new(Sphere {
            center: Ray::new(center, Vec3::default()),
            radius,
            material: mat,
            bbox: AABB::from_min_max(center - r_vec, center + r_vec),
        })
    }

    pub fn new_dynamic(
        center1: Vec3,
        center2: Vec3,
        radius: f32,
        mat: Arc<dyn Material>,
    ) -> Arc<Self> {
        assert!(
            center1.is_finite() && center2.is_finite(),
            "sphere centers must be finite"
        );
        assert!(
            radius.is_finite() && radius > 0.0,
            "sphere radius must be finite and positive"
        );
        let radius = radius.max(0.001);
        let r_vec = Vec3::new(radius, radius, radius);
        let box1 = AABB::from_min_max(center1 - r_vec, center1 + r_vec);
        let box2 = AABB::from_min_max(center2 - r_vec, center2 + r_vec);

        Arc::new(Sphere {
            center: Ray::new(center1, center2 - center1),
            radius,
            material: mat,
            bbox: AABB::from_combined(box1, box2),
        })
    }

    pub fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    fn get_sphere_uv(point: &Vec3, u: &mut f32, v: &mut f32) {
        // point: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = (-point.y).clamp(-1.0, 1.0).acos();
        let phi = (-point.z).atan2(point.x) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }

    fn random_to_sphere(radius: f32, distance_squared: f32) -> Vec3 {
        let r1 = random_f32();
        let r2 = random_f32();
        let sin_theta_max_squared = (radius * radius / distance_squared).clamp(0.0, 1.0);
        let z = 1.0 + r2 * ((1.0 - sin_theta_max_squared).sqrt() - 1.0);

        let phi = 2.0 * PI * r1;
        let radial = (1.0 - z * z).max(0.0).sqrt();
        let x = phi.cos() * radial;
        let y = phi.sin() * radial;

        Vec3::new(x, y, z)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_i: Interval, record: &mut HitRecord) -> Option<&dyn Material> {
        let current_center = self.center.at(ray.time());
        let oc = current_center - *ray.origin();

        let direction = ray.direction();
        let a = direction.x as f64 * direction.x as f64
            + direction.y as f64 * direction.y as f64
            + direction.z as f64 * direction.z as f64;
        if !a.is_finite() || a <= f64::MIN_POSITIVE {
            return None;
        }
        let h = direction.x as f64 * oc.x as f64
            + direction.y as f64 * oc.y as f64
            + direction.z as f64 * oc.z as f64;
        let c = oc.x as f64 * oc.x as f64 + oc.y as f64 * oc.y as f64 + oc.z as f64 * oc.z as f64
            - self.radius as f64 * self.radius as f64;

        let discriminant = h * h - a * c;
        if !discriminant.is_finite() || discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = ((h - sqrt_d) / a) as f32;
        if !ray_i.surrounds(root) {
            root = ((h + sqrt_d) / a) as f32;
            if !ray_i.surrounds(root) {
                return None;
            }
        }

        record.t = root;
        record.point = ray.at(root);
        let outward_normal = (record.point - current_center) / self.radius;
        record.set_face_normal(ray, &outward_normal);
        Self::get_sphere_uv(&outward_normal, &mut record.u, &mut record.v);

        Some(self.material.as_ref())
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    /// This method only works for stationary spheres.
    fn pdf_value(&self, origin: &Vec3, direction: &Vec3) -> f32 {
        if !origin.is_finite() || direction.try_unit_vector().is_none() {
            return 0.0;
        }

        let mut rec = HitRecord::default();
        if self
            .hit(
                &Ray::new(*origin, *direction),
                Interval::new(0.001, f32::INFINITY),
                &mut rec,
            )
            .is_none()
        {
            return 0.0;
        }

        let distance_squared = (self.center.at(0.0) - *origin).length_squared();
        let radius_squared = self.radius * self.radius;
        if !distance_squared.is_finite() {
            return 0.0;
        }
        if distance_squared <= radius_squared {
            return 1.0 / (4.0 * PI);
        }

        let ratio = (radius_squared / distance_squared).clamp(0.0, 1.0);
        let cos_theta_max = (1.0 - ratio).sqrt();
        // This form avoids cancellation in 1 - cos(theta) for distant spheres.
        let solid_angle = 2.0 * PI * ratio / (1.0 + cos_theta_max);
        if !solid_angle.is_finite() || solid_angle <= f32::MIN_POSITIVE {
            return 0.0;
        }

        let pdf = 1.0 / solid_angle;
        if pdf.is_finite() { pdf } else { 0.0 }
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let direction = self.center.at(0.0) - *origin;
        let distance_squared = direction.length_squared();
        if !distance_squared.is_finite() || distance_squared <= self.radius * self.radius {
            return Vec3::random_unit_vector();
        }
        let uvw = ONB::new(direction);
        uvw.transform(&Self::random_to_sphere(self.radius, distance_squared))
    }

    fn can_sample(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::material::Lambertian;

    #[test]
    fn sampling_from_inside_a_sphere_stays_finite() {
        let sphere = Sphere::new(Vec3::default(), 1.0, Lambertian::from_color(Color::WHITE));
        let origin = Vec3::default();

        for _ in 0..128 {
            let direction = sphere.random(&origin);
            assert!(direction.is_finite());
            let pdf = sphere.pdf_value(&origin, &direction);
            assert!(pdf.is_finite() && pdf > 0.0);
        }
    }

    #[test]
    fn distant_sphere_pdf_does_not_overflow_from_cancellation() {
        let sphere = Sphere::new(
            Vec3::new(1.0e4, 0.0, 0.0),
            1.0,
            Lambertian::from_color(Color::WHITE),
        );
        let origin = Vec3::default();
        let direction = sphere.random(&origin);
        let pdf = sphere.pdf_value(&origin, &direction);

        assert!(pdf.is_finite() && pdf > 0.0);
    }
}
