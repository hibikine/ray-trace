use nalgebra::Vector3;
use ray::{HitRec, Ray};
use std::marker;
pub trait Shape {
    fn hit(&self, ray: &Ray, t0: f32, t1: f32) -> Option<HitRec>;
}

pub struct ShapeList<T>
where
    T: marker::Sized + Shape,
{
    list: Vec<T>,
}

impl<T> ShapeList<T>
where
    T: marker::Sized + Shape,
{
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn add(&mut self, shape: T) {
        self.list.push(shape);
    }
}

impl<T> Shape for ShapeList<T>
where
    T: marker::Sized + Shape,
{
    fn hit(&self, ray: &Ray, t0: f32, t1: f32) -> Option<HitRec> {
        match self.list.iter().fold(
            (
                false,
                HitRec::new(
                    t1,
                    Vector3::<f32>::new(0f32, 0f32, 0f32),
                    Vector3::<f32>::new(0f32, 0f32, 0f32),
                ),
            ),
            |acc, x| match x.hit(ray, t0, acc.1.t) {
                None => acc,
                Some(hrec) => (true, hrec),
            },
        ) {
            (false, _) => None,
            (true, hrec) => Some(hrec),
        }
    }
}
