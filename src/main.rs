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
    destination: Position,
}

struct Line {
    start: Position,
    end: Position,
    color: Color,
}

struct Rectangle {
    height: f32,
    width: f32,
    position: Position,
}

struct QuadTree {
    boundary: Rectangle
}

fn lerp(current: &mut Position, target: &Position, t: f32) {
    current.x = current.x + (target.x - current.x) * t;
    current.y = current.y + (target.y - current.y) * t;
}

fn fall_under_gravity(particle: &mut Particle, g: f32, t: f32) {
    particle.position.y = particle.position.y + particle.velocity * t + 0.5 * g * t * t;
    particle.velocity = particle.velocity + g * t;
}

fn get_random_color() -> Color {
    Color {
        r: gen_range(0.0, 1.0),
        g: gen_range(0.0, 1.0),
        b: gen_range(0.0, 1.0),
        a: 1.0,
    }
}


fn lerp_to_random_position(current: &mut Particle, t: f32) {
    let screen_width = macroquad::window::screen_width();
    let screen_height = macroquad::window::screen_height();

    // Check if the particle is within 30 units of its destination
    if (current.destination.x - current.position.x).abs() < 30.0 && 
       (current.destination.y - current.position.y).abs() < 30.0 {
        
        // Generate a new x position within a bound, check edge conditions
        let new_x = if current.position.x < 80.0 {
            // Near left edge
            gen_range(current.position.x, current.position.x + 160.0)
        } else if current.position.x > screen_width - 80.0 {
            // Near right edge
            gen_range(current.position.x - 160.0, current.position.x)
        } else {
            // Not near horizontal edges
            gen_range(current.position.x - 80.0, current.position.x + 80.0)
        };

        // Generate a new y position within a bound, check edge conditions
        let new_y = if current.position.y < 80.0 {
            // Near top edge
            gen_range(current.position.y, current.position.y + 160.0)
        } else if current.position.y > screen_height - 80.0 {
            // Near bottom edge
            gen_range(current.position.y - 160.0, current.position.y)
        } else {
            // Not near vertical edges
            gen_range(current.position.y - 80.0, current.position.y + 80.0)
        };

        // Set the new destination
        current.destination = Position {
            x: new_x,
            y: new_y,
        };
    }
    lerp(&mut current.position, &current.destination, t);
}

fn draw_rect(rect: &Rectangle) {
    //draw a hollow rectangle
    draw_line(rect.position.x, rect.position.y, rect.position.x + rect.width, rect.position.y, 1.0, WHITE);
    draw_line(rect.position.x, rect.position.y, rect.position.x, rect.position.y + rect.height, 1.0, WHITE);
    draw_line(rect.position.x + rect.width, rect.position.y, rect.position.x + rect.width, rect.position.y + rect.height, 1.0, WHITE);
    draw_line(rect.position.x, rect.position.y + rect.height, rect.position.x + rect.width, rect.position.y + rect.height, 1.0, WHITE);
}

#[macroquad::main(window_conf)]
async fn main() {
    let width = macroquad::window::screen_width();
    let height = macroquad::window::screen_height();
    let radius = 8.0;
    let restitution = 0.6;
    let speed = 1.0;
    let num_particles = 100;
    let mut particles: Vec<Particle> = Vec::new();

    let mut quadtree = QuadTree {
        boundary: Rectangle {
            height: height - 5.0,
            width: width - 5.0,
            position: Position {
                x: 5.0,
                y: 5.0,
            }
        }
    };


    for i in 0..100 {
        let start_x = gen_range(50.0, width - 50.0);
        let start_y = gen_range(50.0, height - 50.0);
        particles.push(Particle {
            position: Position {
                x: start_x,
                y: start_y,
            },
            radius: radius,
            color: get_random_color(),
            velocity: gen_range(0.0, 10.0),
            destination: Position {
                x: start_x,
                y: start_y,
            }
        });
    }



    loop { 
        clear_background(BLACK);
        let t = get_frame_time() * speed;

        for particle in particles.iter_mut() {
            lerp_to_random_position(particle, t);
            draw_circle(particle.position.x, particle.position.y, particle.radius, particle.color);
        }
        draw_rect(&quadtree.boundary);
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