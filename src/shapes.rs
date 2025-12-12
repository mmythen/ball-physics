use macroquad::prelude::*;
use glam::Vec2;

pub struct Ball {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub acceleration: f32,
}

pub struct Triangle {
    pub v1: Vec2,
    pub v2: Vec2,
    pub v3: Vec2,
}

pub struct Box {
    pub tl: Vec2,
    pub br: Vec2,
}

pub struct Face {
    pub p1: Vec2,
    pub p2: Vec2,
}

pub trait GetFaces {
    fn get_faces(&self) -> Vec<Face>;
}

impl GetFaces for Triangle {
    fn get_faces(&self) -> Vec<Face> {
        return vec![Face {p1: (self.v1), p2: (self.v2)},
                    Face {p1: (self.v1), p2: (self.v3)},
                    Face {p1: (self.v2), p2: (self.v3)}]
    }
}

impl GetFaces for Box {
    fn get_faces(&self) -> Vec<Face> {
        return vec![Face {p1: (self.tl), p2: vec2(self.br.x, self.tl.y)},
                    Face {p1: vec2(self.br.x, self.tl.y), p2: (self.br)},
                    Face {p1: (self.br), p2: vec2(self.tl.x, self.br.y)},
                    Face {p1: vec2(self.tl.x, self.br.y), p2: (self.tl)}]
    }
}

pub fn make_triangle(p1: (f32, f32), p2: (f32, f32), p3: (f32, f32)) -> Triangle {
    return Triangle { v1: vec2(p1.0, screen_height() - p1.1), v2: vec2(p2.0, screen_height() - p2.1), v3: vec2(p3.0, screen_height() - p3.1) };
}

pub fn make_box(p1: (f32, f32), p2: (f32, f32)) -> Box {
    return Box { tl: vec2(p1.0, screen_height() - p1.1), br: vec2(p2.0, screen_height() - p2.1) }
}