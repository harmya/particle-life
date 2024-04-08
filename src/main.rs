use macroquad::prelude::*;


struct Position {
    x: f32,
    y: f32,
}

struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    color: Color,
}

fn lerp(start: &Position, end: &Position, t: f32) -> Position {
    Position {
        x: start.x + (end.x - start.x) * t,
        y: start.y + (end.y - start.y) * t,
    }
}

#[macroquad::main("Particle Life")]
async fn main() {
    let width = screen_width();
    let height = screen_height();
    let radius = 10.0;
    let mut t = 0.0;
    let speed = 0.2;

    let mut c1 : Circle = Circle {
        x: width / 2.0,
        y: 60.0,
        radius: radius,
        color: RED,
    };

    let mut c2 : Circle = Circle {
        x: width / 2.0,
        y: 120.0,
        radius: radius,
        color: BLUE,
    };

    let mut c3 : Circle = Circle {
        x: width / 2.0,
        y: 180.0,
        radius: radius,
        color: GREEN,
    };

    let start_point_c1 = Position {x: c1.x, y: c1.y};
    let start_point_c2 = Position {x: c2.x, y: c2.y};
    let start_point_c3 = Position { x: c3.x, y: c3.y };

    let end_point = Position { x: width, y: height / 2.0 };

    loop {
        clear_background(BLACK);
        t += get_frame_time() * speed;

        if t > 1.0 {
            t = 1.0;
        }

        let current_point_c1 = lerp(&start_point_c1, &end_point, t);
        let current_point_c2 = lerp(&start_point_c2, &end_point, t);
        let current_point_c3 = lerp(&start_point_c3, &end_point, t);
        
        c1.x = current_point_c1.x;
        c1.y = current_point_c1.y;

        c2.x = current_point_c2.x;
        c2.y = current_point_c2.y;

        c3.x = current_point_c3.x;
        c3.y = current_point_c3.y;

        draw_circle(c1.x, c1.y, c1.radius, c1.color);
        draw_circle(c2.x, c2.y, c2.radius, c2.color);
        draw_circle(c3.x, c3.y, c3.radius, c3.color);

        next_frame().await;
    }

}