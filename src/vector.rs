use std::ops::{Add, Sub, Mul};

#[derive(Debug, PartialEq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}


impl<T> Vector3<T> where 
    T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Copy {
    fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 {x: x, y: y, z: z}
    }
    fn dot(a: Vector3<T>, b: Vector3<T>) -> T{
        a.x * b.x + a.y * b.y + a.z * b.z 
    }
    fn cross(a: Vector3<T>, b: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: a.y * b.z - b.y * a.z,
            y: a.z * b.x - b.z * a.x,
            z: a.x * b.y - b.x * a.y,
        }
    }
}

trait Dot<T> {
    fn dot(self, other: T) -> f64;
}

impl<T> Add for Vector3<T> where
    T: Add<Output=T> {
    type Output = Vector3<T>;
    fn add(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl<T> Sub for Vector3<T> where
    T: Sub<Output=T> {
    type Output = Vector3<T>;

    fn sub(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

#[test]
fn add_test() {
    assert!(Vector3::new(0f64, 1f64, 2f64) + Vector3::new(2f64, 3f64, -4f64) == Vector3::new(2f64, 4f64, -2f64));
}
#[test]
fn sub_test() {
    assert!(Vector3::new(0f64, 1f64, 2f64) - Vector3::new(2f64, 1f64, 0f64) == Vector3::new(-2f64, 0f64, 2f64));
}

