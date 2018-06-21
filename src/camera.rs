use nalgebra::Vector3;
use ray::*;

pub struct Camera {
    origin: Vector3<f32>,
    uvw: [Vector3<f32>; 3],
}

impl Camera {
    pub fn get_ray(&self, u: &f32, v: &f32) -> Ray {
        Ray::new(
            self.origin,
            self.uvw[2] + self.uvw[0] * *u + self.uvw[1] * *v - self.origin,
        )
    }
}

pub struct CameraUVWBuilder {
    u: Vector3<f32>,
    v: Vector3<f32>,
    w: Vector3<f32>,
    origin: Vector3<f32>,
}

impl CameraUVWBuilder {
    pub fn new() -> CameraUVWBuilder {
        CameraUVWBuilder {
            u: Vector3::zeros(),
            v: Vector3::zeros(),
            w: Vector3::zeros(),
            origin: Vector3::zeros(),
        }
    }
    pub fn u(&mut self, u: Vector3<f32>) -> &mut CameraUVWBuilder {
        self.u = u;
        self
    }
    pub fn v(&mut self, v: Vector3<f32>) -> &mut CameraUVWBuilder {
        self.v = v;
        self
    }
    pub fn w(&mut self, w: Vector3<f32>) -> &mut CameraUVWBuilder {
        self.w = w;
        self
    }
    pub fn origin(&mut self, origin: Vector3<f32>) -> &mut CameraUVWBuilder {
        self.origin = origin;
        self
    }
    pub fn finalize(&self) -> Camera {
        Camera {
            origin: self.origin,
            uvw: [self.u, self.v, self.w],
        }
    }
}
pub struct CameraLookAtBuilder {
    lookfrom: Vector3<f32>,
    lookat: Vector3<f32>,
    vup: Vector3<f32>,
    vfov: f32,
    aspect: f32,
}

impl CameraLookAtBuilder {
    pub fn new() -> CameraLookAtBuilder {
        CameraLookAtBuilder {
            lookfrom: Vector3::zeros(),
            lookat: Vector3::new(0f32, 0f32, 1f32),
            vup: Vector3::new(0f32, 1f32, 0f32),
            vfov: 30f32,
            aspect: 1f32,
        }
    }
    pub fn lookfrom(&mut self, lookfrom: Vector3<f32>) -> &mut Self {
        self.lookfrom = lookfrom;
        self
    }
    pub fn lookat(&mut self, lookat: Vector3<f32>) -> &mut Self {
        self.lookat = lookat;
        self
    }
    pub fn vup(&mut self, vup: Vector3<f32>) -> &mut Self {
        self.vup = vup;
        self
    }
    pub fn vfov(&mut self, vfov: f32) -> &mut Self {
        self.vfov = vfov;
        self
    }
    pub fn aspect(&mut self, aspect: f32) -> &mut Self {
        self.aspect = aspect;
        self
    }
    pub fn finalize(&self) -> Camera {
        let halfH = (self.vfov.to_radians() / 2f32).tan();
        let halfW = self.aspect * halfH;
        let w = (self.lookfrom - self.lookat).normalize();
        let u = (self.vup.cross(&w)).normalize();
        let v = w.cross(&u);

        Camera {
            origin: self.lookfrom,
            uvw: [
                2f32 * halfW * u,
                2f32 * halfH * v,
                self.lookfrom - halfW * u - halfH * v - w,
            ],
        }
    }
}
