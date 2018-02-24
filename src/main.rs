extern crate nalgebra;
extern crate image;
extern crate rand;

use image::ImageBuffer;
use image::Rgb;
use std::fs::File;
use image::ImageRgb8;
use image::PNG;
use nalgebra::Vector3;
use std::f32::consts::PI;
use std::f32::consts::FRAC_1_PI as RECIP_PI;
use std::f32::consts::LOG2_E as LOG2;
use std::f32::MAX as F32_MAX;
use std::f32::MIN as F32_MIN;
const PI2: f32 = PI * 2f32;
const RECIP_PI2: f32 = RECIP_PI / 2f32;
const EPSILON: f32 = 1e-6;
const GAMMA_FACTOR: f32 = 2.2;

#[inline]
fn pow2(x: &f32) -> f32 { x*x }
#[inline]
fn pow3(x: &f32) -> f32 { x*x*x }
#[inline]
fn pow4(x: &f32) -> f32 { x*x*x*x }
#[inline]
fn pow5(x: &f32) -> f32 { x*x*x*x*x }
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
fn recip(x: &f32) -> f32 { 1f32 / x }
#[inline]
fn mix(a: &f32, b: &f32, t: &f32) -> f32 { a * (1f32 - t) + b * t }
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
fn radians(deg: &f32) -> f32 { deg / 180f32 * PI }
fn degrees(rad: &f32) -> f32 { rad / PI * 180f32 }

struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {origin: origin, direction: direction}
    }
    fn at(self, t: &f32) -> Vector3<f32> {
        self.origin + *t * self.direction
    }
}

struct Camera {
    origin: Vector3<f32>,
    uvw: [Vector3<f32>; 3]
}

impl Camera {
    fn getRay(&self, u: f32, v: f32) -> Ray {
        Ray::new( 
            self.origin,
            self.uvw[2] + self.uvw[0] * u + self.uvw[1] * v - self.origin
        )
    }
}

struct CameraUVWBuilder {
    u: Vector3<f32>,
    v: Vector3<f32>,
    w: Vector3<f32>,
    origin: Vector3<f32>,
}

impl CameraUVWBuilder {
    fn new() -> CameraUVWBuilder {
        CameraUVWBuilder {
            u: Vector3::zeros(),
            v: Vector3::zeros(),
            w: Vector3::zeros(),
            origin: Vector3::zeros(),
        }
    }
    fn u(&mut self, u: Vector3<f32>) -> &mut CameraUVWBuilder {
        self.u = u;
        self
    }
    fn v(&mut self, v: Vector3<f32>) -> &mut CameraUVWBuilder {
        self.v = v;
        self
    }
    fn w(&mut self, w: Vector3<f32>) -> &mut CameraUVWBuilder {
        self.w = w;
        self
    }
    fn origin(&mut self, origin: Vector3<f32>) -> &mut CameraUVWBuilder {
        self.origin = origin;
        self
    }
    fn finalize(&self) -> Camera {
        Camera {
            origin: self.origin,
            uvw: [
                self.u,
                self.v,
                self.w,
            ]
        }
    }
}
struct CameraLookAtBuilder {
    lookfrom: Vector3<f32>,
    lookat: Vector3<f32>,
    vup: Vector3<f32>,
    vfov: f32,
    aspect: f32,
}

impl CameraLookAtBuilder {
    fn new() -> CameraLookAtBuilder {
        CameraLookAtBuilder {
            lookfrom: Vector3::zeros(),
            lookat: Vector3::new(0f32, 0f32, 1f32),
            vup: Vector3::new(0f32, 1f32, 0f32),
            vfov: 30f32,
            aspect: 1f32,
        }
    }
    fn lookfrom(&mut self, lookfrom: Vector3<f32>) -> &mut Self {
        self.lookfrom = lookfrom;
        self
    }
    fn lookat(&mut self, lookat: Vector3<f32>) -> &mut Self {
        self.lookat = lookat;
        self
    }
    fn vup(&mut self, vup: Vector3<f32>) -> &mut Self {
        self.vup = vup;
        self
    }
    fn vfov(&mut self, vfov: f32) -> &mut Self {
        self.vfov = vfov;
        self
    }
    fn aspect(&mut self, aspect: f32) -> &mut Self {
        self.aspect = aspect;
        self
    }
    fn finalize(&self) -> Camera {
        let halfH = (self.vfov.to_radians() / 2f32).tan();
        let halfW = self.aspect * halfH;
        let w: Vector3<f32> = (self.lookfrom - self.lookat).normalize();
        let u = (self.vup.cross(&w)).normalize();
        let v = w.cross(&u);

        Camera {
            origin: self.lookfrom, 
            uvw: [
                2f32 * halfW * u, 
                2f32 * halfH * v, 
                self.lookfrom - halfW * u - halfH * v - w
            ]
        }
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let mut image = ImageBuffer::new(nx, ny);

    for (x, y, pixel) in image.enumerate_pixels_mut() {    
        let r = (std::u8::MAX as u32 * x / nx) as u8;
        let g = (std::u8::MAX as u32 * y / ny) as u8;
        let b = std::u8::MAX as u8 >> 1;
        *pixel = Rgb([r, g, b]);
    }
    
    let ref mut f = File::create("image.png").unwrap();
    ImageRgb8(image).save(f, PNG).unwrap();
}
