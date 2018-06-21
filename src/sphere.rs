use nalgebra::Vector3;
use pow2;
use ray::{HitRec, Ray};
use shape::Shape;

pub struct Sphere {
    m_center: Vector3<f32>,
    m_radius: f32,
}

impl Sphere {
    pub fn new(c: Vector3<f32>, r: f32) -> Self {
        Self {
            m_center: c,
            m_radius: r,
        }
    }
}

impl Shape for Sphere {
    fn hit(&self, r: &Ray, t0: f32, t1: f32) -> Option<HitRec> {
        let oc = r.origin - self.m_center;
        let a = r.direction.dot(&r.direction);
        let b = 2f32 * r.direction.dot(&oc);
        let c = oc.dot(&oc) - pow2(&self.m_radius);
        // 判別式
        let d = b * b - 4f32 * a * c;
        match d > 0f32 {
            false => None,
            true => {
                let root = d.sqrt();
                {
                    let t = (-b - root) / (2.0f32 * a);
                    if t < t1 && t > t0 {
                        let p = r.at(&t);
                        return Some(HitRec::new(t, p, (p - self.m_center) / self.m_radius));
                    }
                }
                {
                    let t = (-b + root) / (2.0f32 * a);
                    if t < t1 && t > t0 {
                        let p = r.at(&t);
                        return Some(HitRec::new(t, p, (p - self.m_center) / self.m_radius));
                    }
                }
                None
            }
        }
    }
}
