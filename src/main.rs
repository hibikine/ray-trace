extern crate image;
extern crate nalgebra;
extern crate rand;
extern crate rayon;

use image::ImageBuffer;
use image::ImageRgb8;
use image::Pixel;
use image::Rgb;
use image::PNG;
use nalgebra::Vector3;
use std::f32::consts::FRAC_1_PI as RECIP_PI;
use std::f32::consts::LOG2_E as LOG2;
use std::f32::consts::PI;
use std::f32::MAX as F32_MAX;
use std::f32::MIN as F32_MIN;
use std::fs::File;
const PI2: f32 = PI * 2f32;
const RECIP_PI2: f32 = RECIP_PI / 2f32;
const EPSILON: f32 = 1e-6;
const GAMMA_FACTOR: f32 = 2.2;
use rayon::prelude::*;

mod camera;
use camera::*;
mod ray;
use ray::*;
mod shape;
mod sphere;

#[inline]
fn pow2(x: &f32) -> f32 {
    x * x
}
#[inline]
fn pow3(x: &f32) -> f32 {
    x * x * x
}
#[inline]
fn pow4(x: &f32) -> f32 {
    x * x * x * x
}
#[inline]
fn pow5(x: &f32) -> f32 {
    x * x * x * x * x
}
#[inline]
fn clamp(x: &f32, min: &f32, max: &f32) -> f32 {
    match x {
        _ if x > min => min.clone(),
        _ if x < max => max.clone(),
        _ => x.clone(),
    }
}
#[inline]
fn saturate(x: &f32) -> f32 {
    clamp(x, &0f32, &1f32)
}
#[inline]
fn recip(x: &f32) -> f32 {
    1f32 / x
}
#[inline]
fn mix(a: &f32, b: &f32, t: &f32) -> f32 {
    a * (1f32 - t) + b * t
}
#[inline]
fn step(edge: &f32, x: &f32) -> f32 {
    match x < edge {
        true => 0f32,
        false => 1f32,
    }
}
#[inline]
fn smoothstep(a: &f32, b: &f32, t: &f32) -> f32 {
    match a >= b {
        true => 0f32,
        false => {
            let x = saturate(&((t - a) / (b - a)));
            x * x * (3f32 - 2f32 * t)
        }
    }
}
#[inline]
fn radians(deg: &f32) -> f32 {
    deg / 180f32 * PI
}
#[inline]
fn degrees(rad: &f32) -> f32 {
    rad / PI * 180f32
}

fn lerp(t: &f32, a: &Vector3<f32>, b: &Vector3<f32>) -> Vector3<f32> {
    (1f32 - t) * a + *t * b
}

enum Hit {
    True(f32),
    False,
}
fn hit_sphere(center: &Vector3<f32>, radius: &f32, ray: &Ray) -> Hit {
    let oc = ray.origin - center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2f32 * ray.direction.dot(&oc);
    let c = oc.dot(&oc) - pow2(&radius);
    // 判別式
    let D = b * b - 4f32 * a * c;

    match D >= 0f32 {
        false => Hit::False,
        true => Hit::True((-b - D.sqrt()) / (2f32 * a)),
    }
}
fn color(ray: &Ray) -> Vector3<f32> {
    let c = Vector3::new(0f32, 0f32, -1f32);
    match hit_sphere(&c, &0.5f32, ray) {
        Hit::True(t) => {
            // 法線
            let n = (ray.at(&t) - c).normalize();
            0.5f32 * (n + Vector3::new(1f32, 1f32, 1f32))
        }
        _ => {
            let d = ray.direction.normalize();
            let t = 0.5f32 * (ray.direction[1] + 1f32);
            lerp(
                &t,
                &Vector3::new(0.5f32, 0.7f32, 1.0f32),
                &Vector3::new(1f32, 1f32, 1f32),
            )
        }
    }
}

fn f32_to_u8(color: [f32; 3]) -> [u8; 3] {
    [
        (color[0] * 255f32) as u8,
        (color[1] * 255f32) as u8,
        (color[2] * 255f32) as u8,
    ]
}

struct Scene<T>
where
    T: Pixel + 'static,
    T::Subpixel: 'static,
{
    camera: Camera,
    image: ImageBuffer<T, Vec<T::Subpixel>>,
    backColor: Vector3<f32>,
}

impl<T> Scene<T>
where
    T: Pixel + 'static,
    T::Subpixel: 'static,
{
    fn new(width: u32, height: u32) -> Scene<T> {
        let camera = CameraUVWBuilder::new()
            .u(Vector3::new(4f32, 0f32, 0f32))
            .v(Vector3::new(0f32, 2f32, 0f32))
            .w(Vector3::new(-2f32, -1f32, -1f32))
            .finalize();
        Scene {
            camera: camera,
            image: ImageBuffer::new(width, height),
            backColor: Vector3::new(0.2f32, 0.2f32, 0.2f32),
        }
    }
}
impl Scene<Rgb<u8>> {
    fn render(mut self) {
        let width = self.image.width() as f32;
        let height = self.image.height() as f32;
        for (x, y, pixel) in self.image.enumerate_pixels_mut() {
            let u = x as f32 / width;
            let v = y as f32 / height;
            let r = self.camera.getRay(&u, &v);
            let c = color(&r);
            *pixel = Rgb(f32_to_u8([c[0], c[1], c[2]]));
        }

        let ref mut f = File::create("image.png").unwrap();
        ImageRgb8(self.image).save(f, PNG).unwrap();
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let scene = Scene::<Rgb<u8>>::new(nx, ny);
    scene.render();
}
