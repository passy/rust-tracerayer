extern crate image;

use image::ImageBuffer;
use std::path::Path;
use std::f64::INFINITY;

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

    fn plus(self: &Vector, v: &Vector) -> Vector {
        Vector {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    fn minus(self: &Vector, v: &Vector) -> Vector {
        Vector {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }

    fn dot(self: &Vector, v: &Vector) -> f64 {
        self.x * v.x +
            self.y * v.y +
            self.z * v.z
    }

    fn cross(self: &Vector, v: &Vector) -> Vector {
        // Don't screw this up, Passy!
        Vector {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    fn mag(self: &Vector) -> f64 {
        (self.x * self.x +
         self.y * self.y +
         self.z * self.z).sqrt()
    }

    fn norm(self: &Vector) -> Vector {
        let mag = self.mag();
        let div = if mag == 0.0 { INFINITY } else { 1.0 / mag };
        self.times(div)
    }

    fn dot_pos_neg<T, F1, F2>(self: &Vector, v: &Vector, pos: F1, neg: F2) -> T
        where F1: FnOnce(f64) -> T, F2: FnOnce(f64) -> T {
        let d = self.dot(&v);
        if d > 0.0 { pos(d) } else { neg(d) }
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

    fn white() -> Color {
        Color { r: 1.0, g: 1.0, b: 1.0 }
    }

    fn grey() -> Color {
        Color { r: 0.5, g: 0.5, b: 0.5 }
    }

    fn black() -> Color {
        Color { r: 0.0, g: 0.0, b: 0.0 }
    }

    fn background() -> Color {
        Color::black()
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

struct Shiny;

impl Surface for Shiny {
    fn diffuse(&self, _: &Vector) -> Color {
        Color::white()
    }

    fn specular(&self, _: &Vector) -> Color {
        Color::grey()
    }

    fn reflect(&self, _: &Vector) -> f64 {
        0.7
    }

    fn roughness(&self) -> i32 {
        250
    }
}

struct Checkerboard;

impl Surface for Checkerboard {
    fn diffuse(&self, pos: &Vector) -> Color {
        if (pos.z.floor() + pos.x.floor()) as u32 % 2 == 0 {
            Color::white()
        } else {
            Color::black()
        }
    }

    fn specular(&self, _: &Vector) -> Color {
        Color::white()
    }

    fn reflect(&self, pos: &Vector) -> f64 {
        if (pos.z.floor() + pos.x.floor()) as u32 % 2 == 0 {
            0.1
        } else {
            0.7
        }
    }

    fn roughness(&self) -> i32 {
        150
    }
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

impl Camera {
    fn new(pos: Vector, look_at: Vector) -> Camera {
        let fwd = look_at.minus(&pos).norm();
        let down = Vector { x: 0.0, y: -1.0, z: 0.0 };
        let right = fwd.cross(&down).norm().times(1.5);
        let up = fwd.cross(&right).norm().times(1.5);

        Camera {
            pos: pos,
            fwd: fwd,
            right: right,
            up: up,
        }
    }
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
        pos.minus(&self.center).norm()
    }

    fn intersect<'a>(&'a self, ray: &Ray) -> Option<Intersect<'a>> {
        let eo = self.center.minus(&ray.start);
        eo.dot_pos_neg(&ray.dir, |v| {
            let disc = self.radius * self.radius
                - eo.dot(&eo)
                + v * v;
            if disc < 0.0 {
                None
            } else {
                Some(
                    Intersect {
                        thing: self,
                        dist: v - disc.sqrt(),
                    }
                )
            }
        }, |_| None)
    }
}

fn make_scene() -> Scene {
    Scene {
        things: vec![
        ],
        lights: vec![
        ],
        camera: Camera::new(
            Vector {
                x: 3.0,
                y: 2.0,
                z: 4.0,
            }, Vector {
                x: -1.0,
                y: 0.5,
                z: 0.0,
            }
        ),
    }
}

fn render_to_file(scene: &Scene, width: u32, height: u32, path: &Path) {

}

fn main() {
    println!("Let's do some rendering, shall we?");

    render_to_file(&make_scene(), 512, 512, &Path::new("out.png"));

    println!("Well, that's just fine and dandy. Open out.png to marvel at the results.");
}
