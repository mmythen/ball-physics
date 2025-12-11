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

#[macroquad::main("Gravity")]
async fn main() {
    let mut balls: Vec<Ball> = Vec::new();


    loop {
        clear_background(WHITE);

        let p1 = vec2(screen_width() / 2.0, screen_height() / 1.2); //top point
        let p2 = vec2(screen_width() / 2.0, screen_height()); //bottom left
        let p3 = vec2(screen_width() / 1.5,screen_height()); //bottom right
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
            if ball.pos.y > screen_height() - 15.0 {
                ball.pos.y = screen_height() - 15.0;
                ball.velocity.y *= -0.8;

                if ball.velocity.y > -0.01 {
                    ball.acceleration = 0.0;
                }
            }

            //check if touching triangle
            if ball.pos.x > screen_width() /2.0 && ball.pos.x < screen_width() /1.5 && ball.pos.y > screen_height() / 1.2 {
                let reflected = reflection(ball.velocity, get_normal(p1, p3));
                ball.velocity = reflected;
            }


            ball.pos += ball.velocity;
            draw_circle(ball.pos.x, ball.pos.y, 10.0, BLACK);
        }

        next_frame().await;
    }
}
