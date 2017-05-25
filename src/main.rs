extern crate piston_window;
use piston_window::*;
// mod color;
// mod gui;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", (1000, 800))
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g| { clear([0.15, 0.15, 0.2, 0.7], g); });
    }

}
