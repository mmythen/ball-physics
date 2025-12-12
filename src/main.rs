use std::vec;
use macroquad::prelude::*;
use glam::Vec2;


struct Ball {
    pos: Vec2,
    velocity: Vec2,
    acceleration: f32,
}


fn reflection(v: Vec2, n: Vec2) -> Vec2 {
    //n is normal vector of reflecting surface
    let normal = n.normalize();
    let dot = v.dot(normal);
    return v - (2.0 * dot * normal);
}

fn get_normal(p1: Vec2, p2: Vec2) -> Vec2 {
    let slope = (p1.y - p2.y) / (p1.x - p2.x);
    let perp = Vec2 { x: (slope), y: (1.0)};
    let mut normal = Vec2 { x: (-1.0 * slope), y: (1.0)};

    if perp.dot(normal) != 0.0 {
        normal *= -1.0;
    }
    return normal;
}

fn circle_hits_triangle(c: Vec2, r: f32, a: Vec2, b: Vec2, d: Vec2) -> bool {
    // point inside triangle
    let inside = {
        fn sign(p: Vec2, a: Vec2, b: Vec2) -> f32 { (p.x - b.x)*(a.y - b.y) - (a.x - b.x)*(p.y - b.y) }
        let s1 = sign(c, a, b);
        let s2 = sign(c, b, d);
        let s3 = sign(c, d, a);
        (s1 >= 0.0 && s2 >= 0.0 && s3 >= 0.0) || (s1 <= 0.0 && s2 <= 0.0 && s3 <= 0.0)
    };
    if inside { return true; }

    // distance from point to segment
    fn dist_to_seg(p: Vec2, a: Vec2, b: Vec2) -> f32 {
        let ab = b - a;
        let t = ((p - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0);
        p.distance(a + ab * t)
    }

    return dist_to_seg(c, a, b) <= r ||
    dist_to_seg(c, b, d) <= r ||
    dist_to_seg(c, d, a) <= r
}

#[macroquad::main("Gravity")]
async fn main() {
    let mut balls: Vec<Ball> = Vec::new();


    loop {
        clear_background(WHITE);

        let p1 = vec2(screen_width() / 2.0, screen_height() / 1.05); //top point
        let p2 = vec2(screen_width() / 2.0, screen_height()); //bottom 1
        let p3 = vec2(screen_width() / 2.0 - 200.0,screen_height()); //bottom 2
        draw_triangle(p1, p2, p3, BLUE);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            balls.push(Ball {
                pos: Vec2 { x: (mouse_x), y: (mouse_y) },
                velocity: Vec2 { x: (0.0), y: (0.5) },
                acceleration: 0.2
            });
        }

        for ball in &mut balls {
            ball.velocity.y += ball.acceleration;

            //check if touching floor
            if ball.pos.y > screen_height() - 10.0 {
                ball.pos.y = screen_height() - 10.0;
                ball.velocity.y *= -0.8;

                if ball.velocity.y > -0.01 {
                    ball.acceleration = 0.0;
                }
            }

            //check if touching triangle
            if circle_hits_triangle(ball.pos, 10.0, p1, p2, p3) {
                let reflected = reflection(ball.velocity, get_normal(p1, p3));
                ball.velocity = reflected;
            }


            ball.pos += ball.velocity;
            draw_circle(ball.pos.x, ball.pos.y, 10.0, BLACK);
        }

        next_frame().await;
    }
}
