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
    boundary: Rectangle,
    capacity: u32,
    points: Vec<Position>,
    is_divided: bool,
    top_left: Option<Box<QuadTree>>,
    top_right: Option<Box<QuadTree>>,
    bottom_left: Option<Box<QuadTree>>,
    bottom_right: Option<Box<QuadTree>>,
}

impl QuadTree {
    fn new(boundary: Rectangle, capacity: u32) -> QuadTree {
        QuadTree {
            boundary,
            capacity,
            points: Vec::new(),
            is_divided: false,
            top_left: None,
            top_right: None,
            bottom_left: None,
            bottom_right: None,
        }
    }

    fn subdivide(&mut self) {
        let x = self.boundary.position.x;
        let y = self.boundary.position.y;
        let w = self.boundary.width;
        let h = self.boundary.height;
        let new_capacity = self.capacity * 4;

        let top_left = QuadTree::new(Rectangle {
            position: Position {
                x: x,
                y: y,
            },
            width: w / 2.0,
            height: h / 2.0,
        }, self.capacity);

        let top_right = QuadTree::new(Rectangle {
            position: Position {
                x: x + w / 2.0,
                y: y,
            },
            width: w / 2.0,
            height: h / 2.0,
        }, self.capacity);

        let bottom_left = QuadTree::new(Rectangle {
            position: Position {
                x: x,
                y: y + h / 2.0,
            },
            width: w / 2.0,
            height: h / 2.0,
        }, self.capacity);

        let bottom_right = QuadTree::new(Rectangle {
            position: Position {
                x: x + w / 2.0,
                y: y + h / 2.0,
            },
            width: w / 2.0,
            height: h / 2.0,
        }, self.capacity);

        self.top_left = Some(Box::new(top_left));
        self.top_right = Some(Box::new(top_right));
        self.bottom_left = Some(Box::new(bottom_left));
        self.bottom_right = Some(Box::new(bottom_right));
        self.is_divided = true;
        self.capacity = new_capacity;

    }

    fn insert(&mut self, point: Position) {
        if self.points.len() < self.capacity as usize {
            self.points.push(point);
        } else {
            if !self.is_divided {
                self.subdivide();
            }
        }
    }


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

fn draw_quadtree(quadtree: &QuadTree) {
    draw_rect(&quadtree.boundary);
    if quadtree.is_divided {
        if let Some(top_left) = &quadtree.top_left {
            draw_quadtree(top_left);
        }
        if let Some(top_right) = &quadtree.top_right {
            draw_quadtree(top_right);
        }
        if let Some(bottom_left) = &quadtree.bottom_left {
            draw_quadtree(bottom_left);
        }
        if let Some(bottom_right) = &quadtree.bottom_right {
            draw_quadtree(bottom_right);
        }
    }
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

    let mut quadtree = QuadTree::new(Rectangle {
        height: height - 5.0,
        width: width - 5.0,
        position: Position {
            x: 5.0,
            y: 5.0,
        }
    }, 4);

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
        draw_quadtree(&quadtree);
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