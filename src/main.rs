extern crate piston_window;
use piston_window::*;
extern crate rand;
use rand::Rng;
use std::f64::consts::PI;

struct Donut {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    cream_radius: f64, // Radius of the white cream
    num_sprinkles: usize,
    sprinkles: Vec<(f64, f64, [f32; 4])>, // Store sprinkle positions and colors
    rng: rand::rngs::ThreadRng, // Add RNG field
}

impl Donut {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let cream_radius = 35.0; // Radius where sprinkles are placed
        let num_sprinkles = 50; // Number of sprinkles
        let mut sprinkles = Vec::with_capacity(num_sprinkles);

        // Generate sprinkles with fixed positions
        for _ in 0..num_sprinkles {
            let angle = rng.gen::<f64>() * 2.0 * PI; // Random angle
            let distance = rng.gen_range(0.0..cream_radius); // Random distance within cream radius

            let x = distance * angle.cos();
            let y = distance * angle.sin();

            let color = [
                rng.gen::<f32>(),
                rng.gen::<f32>(),
                rng.gen::<f32>(),
                1.0,
            ];
            sprinkles.push((x, y, color));
        }

        Donut {
            x: 400.0, // Centered position
            y: 300.0, // Centered position
            vx: 2.0, // Reduced speed
            vy: 2.0, // Reduced speed
            cream_radius,
            num_sprinkles,
            sprinkles,
            rng, // Initialize RNG field
        }
    }

    fn update(&mut self) {
        self.x += self.vx;
        self.y += self.vy;

        // Reflect off the walls
        if self.x < 30.0 { // Border collision on the left
            self.x = 30.0;
            self.vx *= -1.0;
        } else if self.x > 770.0 { // Border collision on the right
            self.x = 770.0;
            self.vx *= -1.0;
        }

        if self.y < 30.0 { // Border collision on the top
            self.y = 30.0;
            self.vy *= -1.0;
        } else if self.y > 570.0 { // Border collision on the bottom
            self.y = 570.0;
            self.vy *= -1.0;
        }

        // Update sprinkles positions relative to the donut
        for (i, (x, y, color)) in self.sprinkles.iter_mut().enumerate() {
            let angle = i as f64 * (2.0 * PI / self.num_sprinkles as f64); // Even distribution
            let distance = self.cream_radius; // Fixed distance from center

            let x_offset = distance * angle.cos();
            let y_offset = distance * angle.sin();
            *x = self.x + x_offset;
            *y = self.y + y_offset;
        }
    }

    fn draw(&self, c: Context, g: &mut G2d) {
        // Draw white cream as a ring around the donut
        ellipse(
            [1.0, 1.0, 1.0, 0.9], // White color with transparency
            [self.x - 40.0, self.y - 40.0, 80.0, 80.0], // Larger ellipse for cream
            c.transform,
            g,
        );
        ellipse(
            [0.5, 0.5, 0.5, 1.0], // Gray color for the hole in the cream
            [self.x - 22.0, self.y - 22.0, 44.0, 44.0], // Slightly smaller hole
            c.transform,
            g,
        );

        // Draw brown donut
        ellipse(
            [0.8, 0.52, 0.25, 1.0], // Brown color for the donut
            [self.x - 35.0, self.y - 35.0, 70.0, 70.0], // Position and size
            c.transform,
            g,
        );

        // Draw the hole in the donut
        ellipse(
            [0.5, 0.5, 0.5, 1.0], // Light gray color for inner hole
            [self.x - 15.0, self.y - 15.0, 30.0, 30.0], // Position and size of the hole
            c.transform,
            g,
        );

        // Draw fixed colorful sprinkles on the cream
        for (x, y, color) in &self.sprinkles {
            ellipse(
                *color,
                [x - 2.0, y - 2.0, 4.0, 4.0], // Position and size of each sprinkle
                c.transform,
                g,
            );
        }
    }
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Donut", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut donut = Donut::new();

    while let Some(e) = window.next() {
        if let Some(_u) = e.update_args() {
            donut.update();
        }

        if let Some(_r) = e.render_args() {
            window.draw_2d(&e, |c, g, _| {
                clear([0.5, 0.5, 0.5, 1.0], g); // Clear screen with gray color
                donut.draw(c, g); // Draw donut
            });
        }
    }
}

