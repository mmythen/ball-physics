mod shapes;

use macroquad::prelude::*;
use glam::Vec2;
use shapes:: {Ball, Triangle, Box, Face, GetFaces};
use crate::shapes::{make_box, make_triangle};

fn reflection(v: Vec2, n: Vec2) -> Vec2 {
    //n is normal vector of reflecting surface
    let normal = n.normalize();
    let dot = v.dot(normal);
    return v - (2.0 * dot * normal);
}

fn get_normal(p1: Vec2, p2: Vec2) -> Vec2 {
    //fix for vertical lines
    if p1.x == p2.x {
        return vec2(1.0, 0.0);
    }

    let slope = (p1.y - p2.y) / (p1.x - p2.x);
    let perp = Vec2 { x: (slope), y: (1.0)};
    let mut normal = Vec2 { x: (-1.0 * slope), y: (1.0)};

    if perp.dot(normal) != 0.0 {
        normal *= -1.0;
    }
    return normal;
}

fn center_distance(c: Vec2, p1: Vec2, p2: Vec2) -> f32 {
    let ab = p2-p1;
    let t = ((c-p1).dot(ab) / ab.length_squared()).clamp(0.0, 1.0);
    return c.distance(p1 + ab * t);
}


//TURN ALL COLLISIONS INTO FACE COLLISIONS
fn face_collision(c: Vec2, r: f32, p1: Vec2, p2: Vec2) -> bool {
    return center_distance(c, p1, p2) <= r;
}

#[macroquad::main("Gravity")]
async fn main() {
    let mut balls: Vec<Ball> = Vec::new();
    let mut faces: Vec<Face> = Vec::new();
    let mut triangles: Vec<Triangle> = Vec::new();
    let mut boxes: Vec<Box> = Vec::new();

    // creating playground faces
    faces.push( Face { p1: vec2(0.0, 0.0), p2: vec2(0.0, screen_height())}); // left wall
    faces.push( Face { p1: vec2(screen_width(), screen_height()), p2: vec2(0.0, screen_height())}); // floor
    faces.push( Face { p1: vec2(screen_width(), screen_height()), p2: vec2(screen_width(), 0.0)}); // right wall

    let t1 = make_triangle((0.0, 0.0), (0.0, 100.0), (100.0, 0.0));

    let b1 = make_box((300.0, 100.0), (400.0, 0.0));

    faces.append(&mut t1.get_faces());
    faces.append(&mut b1.get_faces());

    boxes.push(b1);
    triangles.push(t1);




    loop {
        clear_background(WHITE);
        // for tri in &triangles {
        //     draw_triangle(tri.v1, tri.v2, tri.v3, BLUE);
        // };
        for face in &faces {
            draw_line(face.p1.x, face.p1.y, face.p2.x, face.p2.y, 2.0, BLUE);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            balls.push(Ball {
                pos: Vec2 { x: (mouse_x), y: (mouse_y) },
                velocity: Vec2 { x: (0.0), y: (0.5) },
                acceleration: 0.1
            });
        }


        for ball in &mut balls {

            //check collision with all faces
            for face in &faces {
                if !face_collision(ball.pos, 10.0, face.p1, face.p2) {
                    continue;
                }
                //stop balls from sinking into objects
                ball.pos -= ball.velocity;


                let mut norm = get_normal(face.p1, face.p2);
                //fix normal vector if ball is going up
                if ball.velocity.y > 0.0 {
                    norm *= -1.0;
                }

                let reflect_vec = reflection(ball.velocity, norm);
                ball.velocity = reflect_vec;
                ball.velocity *= 0.9;
            }

            // update ball position and display
            ball.velocity.y += ball.acceleration;
            ball.pos += ball.velocity;
            draw_circle(ball.pos.x, ball.pos.y, 10.0, BLACK);
        }

        next_frame().await;
    }
}
