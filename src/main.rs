use macroquad::rand::gen_range;
use macroquad::prelude::*;


struct Position {
    x: f32,
    y: f32,
}

struct Particle {
    x: f32,
    y: f32,
    radius: f32,
    color: Color,
    velocity: f32,
    acceleration: f32,
}

struct Line {
    start: Position,
    end: Position,
    color: Color,
}

fn lerp(start: &Position, end: &Position, t: f32) -> Position {
    Position {
        x: start.x + (end.x - start.x) * t,
        y: start.y + (end.y - start.y) * t,
    }
}

fn fall_under_gravity(particle: &mut Particle, g: f32, t: f32) {
    particle.y = particle.y + particle.velocity * t + 0.5 * (g - particle.acceleration) * t * t;
    particle.velocity = particle.velocity + g * t;
}

#[macroquad::main("Particle Life")]
async fn main() {
    let width = screen_width();
    let height = screen_height();
    let radius = 10.0;
    let mut t = 0.1;
    let restitution = 0.6;
    let speed = 5.0;

    let floor = Line {
        start: Position { x: 0.0, y: height / 1.5 },
        end: Position { x: width, y: height / 1.5 },
        color: WHITE,
    };

    let mut particles : Vec<Particle> = Vec::new();

    for _i in 0..1 {
        particles.push(Particle {
            x: width / 2.0,
            y: 50.0,
            radius: radius,
            color: WHITE,
            velocity: 0.0,
            acceleration: 0.0,
        });
    }

    let g = 10.0;

    loop { 
        clear_background(BLACK);
        t = get_frame_time() * speed;
        for particle in particles.iter_mut() {

            if particle.y + particle.radius > floor.start.y {
                particle.y = floor.start.y - particle.radius;
                particle.velocity = -particle.velocity * restitution;
                if particle.velocity.abs() < 5.0 {
                    particle.velocity = 0.0;
                }
            }
            fall_under_gravity(particle, g, t);
            draw_circle(particle.x, particle.y, particle.radius, particle.color);
        }


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