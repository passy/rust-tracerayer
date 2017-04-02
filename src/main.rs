extern crate image;

use image::ImageBuffer;
use std::path::Path;
use std::f64::INFINITY;

const MAX_DEPTH: u32 = 5;

#[derive(Debug, PartialEq, Clone)]
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

    fn plus(self: &Color, v: &Color) -> Color {
        Color {
            r: self.r + v.r,
            g: self.g + v.g,
            b: self.b + v.b,
        }
    }

    fn times(self: &Color, v: &Color) -> Color {
        Color {
            r: self.r * v.r,
            g: self.g * v.g,
            b: self.b * v.b,
        }
    }

    fn to_triple(&self) -> [u8; 3] {
        let norm = |d| if d > 1.0 { 255 } else { (d * 255.0) as u8 };
        [norm(self.r), norm(self.g), norm(self.b)]
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

struct Plane {
    norm: Vector,
    offset: f64,
    surface: Box<Surface>,
}

impl Thing for Plane {
    fn surface(&self) -> &Surface {
        &*self.surface
    }

    fn normal(&self, _: &Vector) -> Vector {
        self.norm.clone()
    }

    fn intersect<'a>(&'a self, ray: &Ray) -> Option<Intersect<'a>> {
        self.norm.dot_pos_neg(&ray.dir, |_| None, |denom| {
            let dist = (self.norm.dot(&ray.start) + self.offset) / -denom;

            Some(Intersect {
                thing: self,
                dist: dist,
            })
        })
    }
}

fn trace_ray(ray: &Ray, scene: &Scene, depth: u32) -> Color {
    closest_intersection(&ray, &scene)
        .map_or(Color::background(),
                |isect| shade(&isect, &scene, &ray, depth))
}

fn test_ray(ray: &Ray, scene: &Scene) -> Option<f64> {
    closest_intersection(&ray, &scene)
        .map(|v| v.dist)
}

fn closest_intersection<'a>(ray: &Ray, scene: &'a Scene) -> Option<Intersect<'a>> {
    let (_, res) = scene.things
        .iter()
        .fold((INFINITY, None), |(closest, closest_isect), thing| {
            match thing.intersect(&ray) {
                Some(isect) => {
                    if isect.dist < closest {
                        (isect.dist, Some(isect))
                    } else {
                        (closest, closest_isect)
                    }
                }
                None => (closest, closest_isect)
            }
        });

    res
}

fn shade(isect: &Intersect, scene: &Scene, ray: &Ray, depth: u32) -> Color {
    let pos = ray.dir.times(isect.dist).plus(&ray.start);
    let normal = isect.thing.normal(&pos);
    let reflect_dir = ray.dir.minus(&normal.times(2.0 * normal.dot(ray.dir)));
    let natural_color = &Color::background().plus(
        &natural_color(isect.thing, &pos, &normal, &reflect_dir, &scene)
    );
    let reflection_color = if depth >= MAX_DEPTH {
        Color::grey()
    } else {
        reflection_color(isect.thing, &pos, &reflect_dir, &scene, depth)
    };
    natural_color.plus(&reflection_color)
}

fn reflection_color(thing: &Thing, pos: &Vector, dir: &Vector, scene: &Scene, depth: u32) -> Color {
    let ray = Ray {
        start: &pos,
        dir: &dir,
    };

    trace_ray(&ray, &scene, depth + 1).scale(
        thing.surface().reflect(&pos)
    )
}

fn natural_color(thing: &Thing, pos: &Vector, normal: &Vector, dir: &Vector, scene: &Scene) -> Color {
    let color_light = |light: &Light| {
        let light_dist = light.pos.minus(&pos);
        let light_norm = light_dist.norm();
        let isect = test_ray(
            &Ray { start: &pos, dir: &light_norm },
            &scene
        );
        let in_shadow = isect.map_or(false, |v| v <= light_dist.mag());

        if in_shadow {
            None
        } else {
            let light_color = light_norm.dot_pos_neg(
                &normal,
                |illum| light.color.scale(illum),
                |_| Color::black(),
            );
            let spec_color = light_norm.dot_pos_neg(
                &dir.norm(),
                |spec| light.color.scale(spec.powi(thing.surface().roughness())),
                |_| Color::black(),
            );
            Some(
                light_color.times(&thing.surface().diffuse(pos))
                    .plus(&spec_color.times(&thing.surface().specular(pos)))
            )
        }
    };

    scene.lights
        .iter()
        .filter_map(color_light)
        .fold(Color::black(), |acc, c| c.plus(&acc))
}

fn make_scene() -> Scene {
    Scene {
        things: vec![
            Box::new(Plane {
                norm: Vector { x: 0.0, y: 1.0, z: 0.0 },
                offset: 0.0,
                surface: Box::new(Checkerboard),
            }),
            Box::new(Sphere {
                center: Vector { x: 0.0, y: 1.0, z: -0.25 },
                radius: 0.5,
                surface: Box::new(Shiny),
            }),
            Box::new(Sphere {
                center: Vector { x: -1.0, y: 0.5, z: 1.5 },
                radius: 1.0,
                surface: Box::new(Shiny),
            }),
        ],
        lights: vec![
            Light {
                pos: Vector { x: -2.0, y: 2.5, z: 0.0 },
                color: Color { r: 0.49, g: 0.07, b: 0.07 },
            },
            Light {
                pos: Vector { x: 1.5, y: 2.5, z: 1.5 },
                color: Color { r: 0.07, g: 0.07, b: 0.49 },
            },
            Light {
                pos: Vector { x: 1.5, y: 2.5, z: -1.5 },
                color: Color { r: 0.07, g: 0.49, b: 0.071 },
            },
            Light {
                pos: Vector { x: 0.0, y: 3.5, z: 0.0 },
                color: Color { r: 0.21, g: 0.21, b: 0.35 },
            },
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
    let get_point = |x, y| {
        let recenter_x = |x: f64| {
            (x - ((width as f64) / 2.0))
                / (2.0 * (width as f64))
        };
        let recenter_y = |y: f64| {
            -(y - ((height as f64) / 2.0))
                / (2.0 * (height as f64))
        };

        scene.camera.fwd.plus(
            &scene.camera.right.times(recenter_x(x as f64)).plus(
                &scene.camera.up.times(recenter_y(y as f64))
            )
        ).norm()
    };

    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let ray = Ray {
            start: &scene.camera.pos,
            dir: &get_point(x, y),
        };

        let color = trace_ray(&ray, &scene, 0).to_triple();
        image::Rgb(color)
    });

    let _ = img.save(path);
}

fn main() {
    println!("Let's do some rendering, shall we?");

    render_to_file(&make_scene(), 512, 512, &Path::new("out.png"));

    println!("Well, that's just fine and dandy. Open out.png to marvel at the results.");
}
