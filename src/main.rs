use macroquad::rand::gen_range;
use macroquad::prelude::*;


struct Position {
    x: f32,
    y: f32,
}

struct Particle {
    position: Position,
    radius: f32,
    color: Color,
    velocity: f32,
}

struct Line {
    start: Position,
    end: Position,
    color: Color,
}

fn lerp(current: &mut Position, target: &Position, t: f32) {
    current.x = current.x + (target.x - current.x) * t;
    current.y = current.y + (target.y - current.y) * t;
}

fn fall_under_gravity(particle: &mut Particle, g: f32, t: f32) {
    particle.position.y = particle.position.y + particle.velocity * t + 0.5 * g * t * t;
    particle.velocity = particle.velocity + g * t;
}

fn bezier(p0: &Position, end_postion: &Position, hook: &Position, t: f32) -> Position {
    return Position {
        x: (1.0 - t) * (1.0 - t) * p0.x + 2.0 * (1.0 - t) * t * hook.x + t * t * end_postion.x,
        y: (1.0 - t) * (1.0 - t) * p0.y + 2.0 * (1.0 - t) * t * hook.y + t * t * end_postion.y,
    };
}

#[macroquad::main(window_conf)]
async fn main() {
    let width = macroquad::window::screen_width();
    let height = macroquad::window::screen_height();
    let radius = 10.0;
    let mut t = 0.001;
    let restitution = 0.6;
    let speed = 1.0;

    let floor = Line {
        start: Position { x: 0.0, y: height / 1.5 },
        end: Position { x: width, y: height / 1.5 },
        color: WHITE,
    };



    let mut particle = Particle {
        position: Position {x : width / 2.0, y: 60.0},
        radius: radius,
        color: WHITE,
        velocity: 0.0,
    };

    loop { 
        clear_background(BLACK);
        t = get_frame_time() * speed;
        fall_under_gravity(&mut particle, 9.8, t);

        if particle.position.y + particle.radius > floor.start.y {
            particle.position.y = floor.start.y - particle.radius;
            particle.velocity = -particle.velocity * restitution;
        }

        draw_circle(particle.position.x, particle.position.y, particle.radius, particle.color);


        draw_line(
            floor.start.x,
            floor.start.y,
            floor.end.x,
            floor.end.y,
            2.0, 
            floor.color,
        );
        next_frame().await;
    }

}

fn window_conf() -> Conf {
    Conf {
        window_title: "Particle Life".to_owned(),
        window_width: 1200,
        window_height: 800,
        ..Default::default()
    }
}