use std::borrow::Borrow;

use macroquad::rand::gen_range;
use macroquad::{color, prelude::*};

#[derive(Clone, Copy)]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy)]
struct Velocity {
    x: f64,
    y: f64,
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
    height: f64,
    width: f64,
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

fn move_particle(particle: &mut Particle, t: f64, width: f64, height: f64, radius: f64) {

    // if particle out side right boundary, wrap to left boundary
    if particle.position.x - radius > width {
        particle.position.x = 0.0 - radius;
    }
    // if particle out side left boundary, wrap to right boundary
    if particle.position.x + radius < 0.0 {
        particle.position.x = width + radius;
    }
    // if particle out side bottom boundary, wrap to top boundary
    if particle.position.y - radius > height as f64 {
        particle.position.y = 0.0 - radius;
    }
    // if particle out side top boundary, wrap to bottom boundary
    if particle.position.y + radius < 0.0 {
        particle.position.y = height as f64 + radius;
    }


    particle.position.x = particle.position.x + particle.velocity.x * t;
    particle.position.y = particle.position.y + particle.velocity.y * t;


}


fn draw_rect(rect: &Rectangle) {
    //draw a hollow rectangle
    // draw_line(rect.position.x, rect.position.y, rect.position.x + rect.width, rect.position.y, 1.0, WHITE);
    // draw_line(rect.position.x, rect.position.y, rect.position.x, rect.position.y + rect.height, 1.0, WHITE);
    // draw_line(rect.position.x + rect.width, rect.position.y, rect.position.x + rect.width, rect.position.y + rect.height, 1.0, WHITE);
    // draw_line(rect.position.x, rect.position.y + rect.height, rect.position.x + rect.width, rect.position.y + rect.height, 1.0, WHITE);

    // cast to double

    draw_line(rect.position.x as f32, rect.position.y as f32, (rect.position.x + rect.width) as f32, rect.position.y as f32, 1.0, WHITE);
    draw_line(rect.position.x as f32, rect.position.y as f32, rect.position.x as f32, (rect.position.y + rect.height) as f32, 1.0, WHITE);
    draw_line((rect.position.x + rect.width) as f32, rect.position.y as f32, (rect.position.x + rect.width) as f32, (rect.position.y + rect.height) as f32, 1.0, WHITE);
    draw_line(rect.position.x as f32, (rect.position.y + rect.height) as f32, (rect.position.x + rect.width) as f32, (rect.position.y + rect.height) as f32, 1.0, WHITE);
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
    let colors = vec![RED, GREEN];
    let index = gen_range(0, colors.len());
    return colors[index];
}

fn colour_attraction_factor_matrix() -> Vec<Vec<f64>> {
    let matrix = vec![vec![0.0; 4]; 4];
    return matrix;
}

fn red_to_red (matrix: &mut Vec<Vec<f64>>) {
    matrix[0][0] = 1.0;
}

fn green_to_red (matrix: &mut Vec<Vec<f64>>) {
    matrix[1][0] = 0.3;
}

fn color_to_index(color: Color) -> usize {
    if color == RED {
        return 0;
    } else if color == GREEN {
        return 1;
    } else if color == BLUE {
        return 2;
    } else {
        return 3;
    }
}

fn get_force(r: f64, p1_color: Color, p2_color: Color, color_matrix: &Vec<Vec<f64>>) -> f64 {
    let c_1_idx = color_to_index(p1_color);
    let c_2_idx = color_to_index(p2_color);
    let attraction_factor = color_matrix[c_1_idx][c_2_idx];
    const BETA : f64 = 0.3;
    if r < BETA {
        return r / BETA - 1.0;
    } else if BETA < r && r < 1.0 {
        return (1.0 - ((2.0 * r - BETA - 1.0).abs() / (1.0 - BETA))) * attraction_factor;
    } else {
        return 0.0;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let width = macroquad::window::screen_width() as f64;
    let height = macroquad::window::screen_height() as f64;
    let radius = 2.0;
    let num_particles: u32 = 1000;
    let mut particles: Vec<Particle> = Vec::new();

    let mut quadtree = QuadTree::new(Rectangle {
        height: height - 5.0,
        width: width - 5.0,
        position: Position {
            x: 5.0,
            y: 5.0,
        }
    }, 8);

    for _ in 0..num_particles {
        let random_color = pick_one_color();
        let location = Position {
            x: gen_range(radius + 5.0, radius + width - 5.0),
            y: gen_range(radius + 5.0, radius + height - 5.0),
        };

        let velocity_x = gen_range(-10.0, 10.0);
        let velocity_y = gen_range(-10.0, 10.0);
        let particle = Particle::new(location, random_color, Velocity {
            x: velocity_x,
            y: velocity_y,
        });

        particles.push(particle.clone());
        quadtree.insert(Some(particle));
    }

    let mut color_matrix = colour_attraction_factor_matrix();
    let mut i: u128 = 0;
    loop { 
        i += 1;
        if i % 300 == 0 {
            red_to_red(&mut color_matrix);
        }
        if i % 600 == 0 {
             green_to_red(&mut color_matrix);
        }
        
        clear_background(BLACK);
        let t = get_frame_time() as f64;
        quadtree.clear_quadtree();
        for particle in particles.iter_mut() {
            let next_time_position = Position {
                x: particle.position.x + particle.velocity.x * t,
                y: particle.position.y + particle.velocity.y * t,
            };

            let threshold = 100.0;

            let mut near_particles = quadtree.query(&Rectangle {
                height: threshold * 2.0,
                width: threshold * 2.0,
                position: Position {
                    x: next_time_position.x - radius - threshold,
                    y: next_time_position.y - radius - threshold,
                }
            });

            let mut final_force_x = 0.0;
            let mut final_force_y = 0.0;

            for near_particle in near_particles.iter_mut() {
                
                let dx = near_particle.position.x - particle.position.x;
                let dy = near_particle.position.y - particle.position.y;
                let distance_squared = dx.powi(2) + dy.powi(2);
                let distance = distance_squared.sqrt();
                let direction_x = dx / distance_squared.sqrt();
                let direction_y = dy / distance_squared.sqrt();
                let force = get_force(distance / threshold, particle.color, near_particle.color, &color_matrix);
                final_force_x += force * direction_x;
                final_force_y += force * direction_y;
                    
                
            }
            
            let final_acceleration_x = final_force_x * threshold * 2.0;
            let final_acceleration_y = final_force_y * threshold * 2.0;
          
            let velocity_decay = 0.5f64.powf(t/0.02);
            particle.velocity.x = velocity_decay * particle.velocity.x + final_acceleration_x * t;
            particle.velocity.y = velocity_decay * particle.velocity.y + final_acceleration_y * t;

            move_particle(particle, t, width, height, radius);
            quadtree.insert(Some(particle.clone()));
            draw_circle(particle.position.x as f32, particle.position.y as f32, radius as f32, particle.color);
        }
        //draw_quadtree(&quadtree);
        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Particle Life".to_owned(),
        window_width: 500,
        window_height: 500,
        ..Default::default()
    }
}