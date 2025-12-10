use macroquad::prelude::*;


struct Ball {
    x: f32,
    y: f32,
    velocity: f32,
    acceleration: f32,
}

#[macroquad::main("Gravity")]
async fn main() {
    let mut balls: Vec<Ball> = Vec::new();

    loop {
        clear_background(WHITE);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            balls.push(Ball {
                x: mouse_x,
                y: mouse_y,
                velocity: 1.0,
                acceleration: 0.5
            });
        }

        for ball in &mut balls {
            ball.velocity += ball.acceleration;
            ball.y += ball.velocity;

            if ball.y > screen_height() - 15.0 {
                ball.y = screen_height() - 15.0;
                ball.velocity *= -0.8;

                if ball.velocity > -0.01 {
                    ball.acceleration = 0.0;
                }
            }

            draw_circle(ball.x, ball.y, 10.0, BLACK);
        }

        next_frame().await;
    }
}
