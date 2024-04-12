use std::borrow::Borrow;

use macroquad::rand::gen_range;
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Clone)]
struct Particle {
    position: Position,
    color: Color,
    velocity: Velocity,
}

impl Particle {
    fn new(position: Position, color: Color, velocity: Velocity) -> Particle {
        Particle {
            position,
            color,
            velocity,
        }
    }
}

struct Rectangle {
    height: f32,
    width: f32,
    position: Position,
}

struct QuadTree {
    boundary: Rectangle,
    capacity: u32,
    points: Vec<Particle>,
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

    }

    fn within_boundary(&self, point: &Position) -> bool {
        let x = point.x;
        let y = point.y;
        let bx = self.boundary.position.x;
        let by = self.boundary.position.y;
        let w = self.boundary.width;
        let h = self.boundary.height;

        return x >= bx && x <= bx + w && y >= by && y <= by + h;
    }

    fn insert(&mut self, particle: Option<Particle>) -> Option<Particle> {

        if particle.is_none() {
            return None;
        }

        if !self.within_boundary(&particle.as_ref().unwrap().position) {
            return Some(particle.unwrap());
        }

        if self.points.len() < self.capacity as usize {
            self.points.push(particle.unwrap());
            return None;
        } else {
            if !self.is_divided {
                self.subdivide();
            }

            let return_particle = self.top_left.as_mut().unwrap().insert(particle);
            let return_particle = self.top_right.as_mut().unwrap().insert(return_particle);
            let return_particle = self.bottom_left.as_mut().unwrap().insert(return_particle);
            let return_particle = self.bottom_right.as_mut().unwrap().insert(return_particle);
            
            return return_particle;
            
        }

    }

    fn does_range_overlap(&self, range: &Rectangle) -> bool {
        let x = range.position.x;
        let y = range.position.y;
        let w = range.width;
        let h = range.height;

        let bx = self.boundary.position.x;
        let by = self.boundary.position.y;
        let bw = self.boundary.width;
        let bh = self.boundary.height;

        return x + w >= bx && x <= bx + bw && y + h >= by && y <= by + bh;
    }

    fn query(&self, range: &Rectangle) -> Vec<Particle> {
        let mut found = Vec::new();
        if !self.does_range_overlap(&range) {
            return found;
        } else {
            for point in self.points.iter() {
                if self.within_boundary(point.position.borrow()) {
                    found.push(point.clone());
                }
            }

            if self.is_divided {
                found.append(&mut self.top_left.as_ref().unwrap().query(range));
                found.append(&mut self.top_right.as_ref().unwrap().query(range));
                found.append(&mut self.bottom_left.as_ref().unwrap().query(range));
                found.append(&mut self.bottom_right.as_ref().unwrap().query(range));
            }
            
        }
        return found;
    }

    fn clear_quadtree(&mut self) {
        self.points.clear();
        self.is_divided = false;
        self.top_left = None;
        self.top_right = None;
        self.bottom_left = None;
        self.bottom_right = None;
    }
}

fn move_particle(particle: &mut Particle, t: f32) {
    particle.position.x = particle.position.x + particle.velocity.x * t;
    particle.position.y = particle.position.y + particle.velocity.y * t;
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

fn pick_one_color() -> Color {
    let colors = vec![RED, GREEN, BLUE, YELLOW, PURPLE];
    let index = gen_range(0, colors.len());
    return colors[index];
}

#[macroquad::main(window_conf)]
async fn main() {
    let width = macroquad::window::screen_width();
    let height = macroquad::window::screen_height();
    let radius = 5.0;
    let speed = 1.3;
    let num_particles = 1000;
    let mut particles: Vec<Particle> = Vec::new();

    let mut quadtree = QuadTree::new(Rectangle {
        height: height - 5.0,
        width: width - 5.0,
        position: Position {
            x: 5.0,
            y: 5.0,
        }
    }, 4);

    for _ in 0..num_particles {
        let start_x = gen_range(50.0, width - 50.0);
        let start_y = gen_range(50.0, height - 50.0);
        let velocity_x = gen_range(-30.0, 30.0);
        let velocity_y = gen_range(-30.0, 30.0);
        let random_color = pick_one_color();
        let particle = Particle::new(Position {
            x: start_x,
            y: start_y,
        }, random_color, Velocity {
            x: velocity_x,
            y: velocity_y,
        });

        particles.push(particle.clone());
        quadtree.insert(Some(particle));
    }


    loop { 
        clear_background(BLACK);
        let t = get_frame_time() * speed;
        quadtree.clear_quadtree();
        for particle in particles.iter_mut() {
            let next_time_position = Position {
                x: particle.position.x + particle.velocity.x * t,
                y: particle.position.y + particle.velocity.y * t,
            };

            let mut near_particles = quadtree.query(&Rectangle {
                height: 2.0 * radius,
                width: 2.0 * radius,
                position: Position {
                    x: next_time_position.x - 2.0 * radius,
                    y: next_time_position.y - 2.0 * radius
                }
            });

            for near_particle in near_particles.iter_mut() {
                if near_particle.position.x != particle.position.x && near_particle.position.y != particle.position.y {
                    let dx = near_particle.position.x - particle.position.x;
                    let dy = near_particle.position.y - particle.position.y;
                    let distance_squared = dx.powi(2) + dy.powi(2);
                    let magnitude = distance_squared.sqrt();

                    let direction_x = dx / magnitude;
                    let direction_y = dy / magnitude;

                    if near_particle.color == particle.color {
                        let particle_velocity_mag = (particle.velocity.x.powi(2) + particle.velocity.y.powi(2)).sqrt();
                        let near_particle_velocity_mag = (near_particle.velocity.x.powi(2) + near_particle.velocity.y.powi(2)).sqrt();

                        particle.velocity.x = direction_x * particle_velocity_mag;
                        particle.velocity.y = direction_y * particle_velocity_mag;

                        near_particle.velocity.x = direction_x * near_particle_velocity_mag;
                        near_particle.velocity.y = direction_y * near_particle_velocity_mag;
                    }
                

                    if distance_squared < 4.0 * radius.powi(2) {
                        let distance = distance_squared.sqrt();
                        let nx = dx / distance;
                        let ny = dy / distance;
            
                        let vx = near_particle.velocity.x - particle.velocity.x;
                        let vy = near_particle.velocity.y - particle.velocity.y;
            
                        let dot_product = vx * nx + vy * ny;

                        if dot_product < 0.0 {
                            let impulse_x = dot_product * nx;
                            let impulse_y = dot_product * ny;
            
                            near_particle.velocity.x -= impulse_x;
                            near_particle.velocity.y -= impulse_y;
                            particle.velocity.x += impulse_x;
                            particle.velocity.y += impulse_y;
                        }
                    }
                }
            }
            

            if next_time_position.x < radius || next_time_position.x + radius > width {
                particle.velocity.x = -particle.velocity.x;
                
            }

            if next_time_position.y < radius || next_time_position.y + radius > height {
                particle.velocity.y = -particle.velocity.y;
            }

            move_particle(particle, t);
            quadtree.insert(Some(particle.clone()));
            draw_circle(particle.position.x, particle.position.y, radius, particle.color);
        }
        //draw_quadtree(&quadtree);
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