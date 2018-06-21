use nalgebra::Vector3;

pub struct HitRec {
    pub t: f32,
    pub p: Vector3<f32>,
    pub n: Vector3<f32>,
}

impl HitRec {
    pub fn new(t: f32, p: Vector3<f32>, n: Vector3<f32>) -> Self {
        Self { t: t, p: p, n: n }
    }
}

pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }
    pub fn at(&self, t: &f32) -> Vector3<f32> {
        self.origin + *t * self.direction
    }
}
