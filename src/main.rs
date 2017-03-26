extern crate image;

use image::ImageBuffer;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    fn times(self: &Vector, k: f64) -> Vector {
        Vector {
            x: k * self.x,
            y: k * self.y,
            z: k * self.z,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    fn scale(self: &Color, k: f64) -> Color {
        Color {
            r: k * self.r,
            g: k * self.g,
            b: k * self.b,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Ray<'a> {
    dir: &'a Vector,
    start: &'a Vector,
}

trait Surface {
    fn diffuse(&self, pos: &Vector) -> Color;
    fn specular(&self, pos: &Vector) -> Color;
    fn reflect(&self, pos: &Vector) -> f64;
    fn roughness(&self) -> i32;
}

trait Thing {
    fn normal(&self, pos: &Vector) -> Vector;
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<Intersect<'a>>;
    fn surface(&self) -> &Surface;
}

struct Intersect<'a> {
    thing: &'a Thing,
    dist: f64,
}

#[derive(Debug, PartialEq)]
struct Light {
    pos: Vector,
    color: Color,
}

#[derive(Debug, PartialEq)]
struct Camera {
    pos: Vector,
    fwd: Vector,
    right: Vector,
    up: Vector,
}

struct Scene {
    things: Vec<Box<Thing>>,
    lights: Vec<Light>,
    camera: Camera,
}

struct Sphere {
    center: Vector,
    radius: f64,
    surface: Box<Surface>,
}

impl Thing for Sphere {
    fn surface(&self) -> &Surface {
        &*self.surface
    }

    fn normal(&self, pos: &Vector) -> Vector {
        panic!("Not implemented")
    }

    fn intersect<'a>(&'a self, ray: &Ray) -> Option<Intersect<'a>> {
        panic!("Not implemented")
    }
}
